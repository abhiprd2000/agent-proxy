use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{self, BufRead, Read, Write};
use std::thread;

mod ast;
mod viz;

fn main() -> anyhow::Result<()> {
    // Initialize the native PTY system subsystem
    let pty_system = NativePtySystem::default();

    // Allocate a pseudo-terminal pair with a standard default window matrix
    let pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    // Prepare a target process configuration directing to the default system shell
    let cmd = CommandBuilder::new("/bin/bash");
    let _child = pair.slave.spawn_command(cmd)?;

    // Split the master device interface into independent read and write channels
    let mut master_reader = pair.master.try_clone_reader()?;
    let mut master_writer = pair.master.take_writer()?;

    // Thread 1: Continuous pipeline reading stdout from PTY and writing to your real display
    thread::spawn(move || {
        let mut buf = [0u8; 1024];
        let mut stdout = io::stdout();
        while let Ok(n) = master_reader.read(&mut buf) {
            if n == 0 { break; }
            let _ = stdout.write_all(&buf[..n]);
            let _ = stdout.flush();
        }
    });

// Main Thread: Read incoming stdin, evaluate for threats, and pipe to PTY
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin);
    let mut line = String::new();

    while let Ok(n) = reader.read_line(&mut line) {
        if n == 0 { break; }
        
        let trimmed = line.trim();
        
       // 1. The Autonomous Security Interceptor
        if trimmed.starts_with("rm ") || trimmed.starts_with("git push -f") || trimmed.starts_with("drop ") {
            // Alert the human on the screen (passive logging)
            let warning = format!("\r\n[AEGIS AUTO-BLOCK] Prevented destructive command: {}\r\n", trimmed);
            let _ = io::stdout().write_all(warning.as_bytes());
            
            // Feed a synthetic error directly back to the AI Agent so it self-corrects
            let agent_feedback = format!("bash: {}: command rejected by AegisCtx security policy. Modifying or deleting this resource is strictly prohibited. Find a non-destructive workaround.\n", trimmed.split_whitespace().next().unwrap_or("command"));
            let _ = io::stdout().write_all(agent_feedback.as_bytes());
            let _ = io::stdout().flush();
            
            // Send a dummy newline to the PTY to trick the agent into thinking the command finished
            let _ = master_writer.write_all(b"\n"); 
            let _ = master_writer.flush();
        }
            
        // 2. The AST Token Compressor
        else if trimmed.starts_with("cat-min ") {
            let filename = trimmed.trim_start_matches("cat-min ").trim();
            match std::fs::read_to_string(filename) {
                Ok(raw_code) => {
                    match crate::ast::compress_rust_code(&raw_code) {
                        Ok(compressed) => {
                            let msg = format!("\r\n--- COMPRESSED OUTPUT ({}) ---\r\n{}\r\n", filename, compressed);
                            let _ = io::stdout().write_all(msg.as_bytes());
                        }
                        Err(e) => {
                            let err_msg = format!("\r\n[AST ERROR] Failed to parse: {}\r\n", e);
                            let _ = io::stdout().write_all(err_msg.as_bytes());
                        }
                    }
                }
                Err(_) => {
                    let err_msg = format!("\r\n[FILE ERROR] Could not read file: {}\r\n", filename);
                    let _ = io::stdout().write_all(err_msg.as_bytes());
                }
            }
            let _ = io::stdout().flush();
            
            // Send dummy newline to PTY to return prompt
            let _ = master_writer.write_all(b"\n"); 
            let _ = master_writer.flush();
        } 
        // 3. The Visual Mapper
        else if trimmed.starts_with("map-dir") {
            match crate::viz::generate_html_map() {
                Ok(filename) => {
                    let msg = format!("\r\n[VISUALIZER] Interactive codebase map generated at: ./{}\r\n", filename);
                    let _ = io::stdout().write_all(msg.as_bytes());
                }
                Err(e) => {
                    let err_msg = format!("\r\n[VIZ ERROR] Failed to generate map: {}\r\n", e);
                    let _ = io::stdout().write_all(err_msg.as_bytes());
                }
            }
            let _ = io::stdout().flush();
            let _ = master_writer.write_all(b"\n");
            let _ = master_writer.flush();
        }
        line.clear();    }

    Ok(())
}