use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{self, BufRead, Read, Write};
use std::thread;

mod ast;
mod viz;

fn main() -> anyhow::Result<()> {
    // Initialize the native PTY subsystem
    let pty_system = NativePtySystem::default();
    let pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    // Spawn the underlying Unix shell
    let cmd = CommandBuilder::new("/bin/bash");
    let _child = pair.slave.spawn_command(cmd)?;

    let mut master_reader = pair.master.try_clone_reader()?;
    let mut master_writer = pair.master.take_writer()?;

    // Thread 1: Stream PTY output directly to the real terminal screen
    thread::spawn(move || {
        let mut buf = [0u8; 1024];
        let mut stdout = io::stdout();
        while let Ok(n) = master_reader.read(&mut buf) {
            if n == 0 { break; }
            let _ = stdout.write_all(&buf[..n]);
            let _ = stdout.flush();
        }
    });

    // Main Thread: Intercept, evaluate, and route incoming stdin commands
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin);
    let mut line = String::new();

    while let Ok(n) = reader.read_line(&mut line) {
        if n == 0 { break; }
        
        let trimmed = line.trim();
        
        // 1. The Autonomous Security Interceptor (LLM-in-the-loop)
        if trimmed.starts_with("rm ") || trimmed.starts_with("git push -f") || trimmed.starts_with("drop ") {
            let warning = format!("\r\n[AGENTPROXY BLOCK] Prevented destructive command: {}\r\n", trimmed);
            let _ = io::stdout().write_all(warning.as_bytes());
            
            // Inject synthetic error back to the AI
            let agent_feedback = format!("bash: {}: command rejected by AgentProxy security policy. Modifying or deleting this resource is strictly prohibited. Find a non-destructive workaround.\n", trimmed.split_whitespace().next().unwrap_or("command"));
            let _ = io::stdout().write_all(agent_feedback.as_bytes());
            let _ = io::stdout().flush();
            
            // Dummy newline to reset prompt
            let _ = master_writer.write_all(b"\n"); 
            let _ = master_writer.flush();
        } 
        // 2. The Universal AST Token Compressor (Tree-sitter)
        else if trimmed.starts_with("cat-min ") {
            let filename = trimmed.trim_start_matches("cat-min ").trim();
            match std::fs::read_to_string(filename) {
                Ok(raw_code) => {
                    // Route to tree-sitter with the filename to detect language extensions
                    match crate::ast::compress_code(&raw_code, filename) {
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
        // 4. Normal Execution Pass-through
        else {
            let _ = master_writer.write_all(line.as_bytes());
            let _ = master_writer.flush();
        }
        line.clear();
    }

    Ok(())
}