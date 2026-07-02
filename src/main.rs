use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{self, BufRead, Read, Write};
use std::thread;

mod ast;
mod audit;
mod config;
mod policy;
mod viz;

fn main() -> anyhow::Result<()> {
let pty_system = NativePtySystem::default();

let pair = pty_system.openpty(PtySize {
    rows: 24,
    cols: 80,
    pixel_width: 0,
    pixel_height: 0,
})?;

let cmd = CommandBuilder::new("/bin/bash");
let _child = pair.slave.spawn_command(cmd)?;

let mut master_reader = pair.master.try_clone_reader()?;
let mut master_writer = pair.master.take_writer()?;

thread::spawn(move || {
    let mut buf = [0u8; 1024];
    let mut stdout = io::stdout();

    while let Ok(n) = master_reader.read(&mut buf) {
        if n == 0 {
            break;
        }

        let _ = stdout.write_all(&buf[..n]);
        let _ = stdout.flush();
    }
});

let stdin = io::stdin();
let mut reader = io::BufReader::new(stdin);
let mut line = String::new();

while let Ok(n) = reader.read_line(&mut line) {
    if n == 0 {
        break;
    }

    let trimmed = line.trim();

    match policy::evaluate(trimmed) {
        policy::Decision::Block(reason) => {
            audit::log("BLOCK", trimmed);

            let warning = format!(
                "\r\n[AGENTPROXY BLOCK]\r\nReason: {}\r\nCommand: {}\r\n",
                reason,
                trimmed
            );

            let _ = io::stdout().write_all(warning.as_bytes());

            let feedback = format!(
                "bash: command rejected by AgentProxy policy: {}\n",
                reason
            );

            let _ = io::stdout().write_all(feedback.as_bytes());
            let _ = io::stdout().flush();

            let _ = master_writer.write_all(b"\n");
            let _ = master_writer.flush();
        }

        policy::Decision::Review(reason) => {
            let review = format!(
                "\r\n[AgentProxy Review]\r\nReason: {}\r\nContinue? [y/N]: ",
                reason
            );

            let _ = io::stdout().write_all(review.as_bytes());
            let _ = io::stdout().flush();

            let mut approval = String::new();
            io::stdin().read_line(&mut approval)?;

            if approval.trim().eq_ignore_ascii_case("y") {
                audit::log("REVIEW_ALLOW", trimmed);

                let _ = master_writer.write_all(line.as_bytes());
                let _ = master_writer.flush();
            } else {
                audit::log("REVIEW_DENY", trimmed);

                let _ = io::stdout()
                    .write_all(b"\r\nCommand cancelled.\r\n");
                let _ = io::stdout().flush();
            }
        }

        policy::Decision::Allow => {
            if trimmed.starts_with("cat-min ") {
                audit::log("ALLOW", trimmed);

                let filename =
                    trimmed.trim_start_matches("cat-min ").trim();

                match std::fs::read_to_string(filename) {
                    Ok(raw_code) => {
                        match crate::ast::compress_code(
                            &raw_code,
                            filename,
                        ) {
                            Ok(compressed) => {
                                let msg = format!(
                                    "\r\n--- COMPRESSED OUTPUT ({}) ---\r\n{}\r\n",
                                    filename,
                                    compressed
                                );

                                let _ =
                                    io::stdout().write_all(msg.as_bytes());
                            }

                            Err(e) => {
                                let err = format!(
                                    "\r\n[AST ERROR] {}\r\n",
                                    e
                                );

                                let _ =
                                    io::stdout().write_all(err.as_bytes());
                            }
                        }
                    }

                    Err(_) => {
                        let err = format!(
                            "\r\n[FILE ERROR] Could not read file: {}\r\n",
                            filename
                        );

                        let _ =
                            io::stdout().write_all(err.as_bytes());
                    }
                }

                let _ = io::stdout().flush();
                let _ = master_writer.write_all(b"\n");
                let _ = master_writer.flush();
            } else if trimmed.starts_with("map-dir") {
                audit::log("ALLOW", trimmed);

                match crate::viz::generate_html_map() {
                    Ok(filename) => {
                        let msg = format!(
                            "\r\n[VISUALIZER] Generated: {}\r\n",
                            filename
                        );

                        let _ =
                            io::stdout().write_all(msg.as_bytes());
                    }

                    Err(e) => {
                        let err = format!(
                            "\r\n[VIZ ERROR] {}\r\n",
                            e
                        );

                        let _ =
                            io::stdout().write_all(err.as_bytes());
                    }
                }

                let _ = io::stdout().flush();
                let _ = master_writer.write_all(b"\n");
                let _ = master_writer.flush();
            } else {
                audit::log("ALLOW", trimmed);

                let _ = master_writer.write_all(line.as_bytes());
                let _ = master_writer.flush();
            }
        }
    }

    line.clear();
}

Ok(())

}
