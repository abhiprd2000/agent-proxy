use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{self, BufRead, Read, Write};
use std::thread;

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
        
        // The Core Security Heuristic
        if trimmed.starts_with("rm ") || trimmed.starts_with("git push -f") || trimmed.starts_with("drop ") {
            let warning = format!("\r\n[AEGIS BLOCKED] Destructive command intercepted: {}\r\n", trimmed);
            let _ = io::stdout().write_all(warning.as_bytes());
            let _ = io::stdout().flush();
            
            // We intentionally DO NOT write this command to the PTY. 
            // We simulate a clean prompt return to trick the agent.
            let _ = master_writer.write_all(b"\n"); 
            let _ = master_writer.flush();
        } else {
            // Safe command: Pass it through to the actual shell
            let _ = master_writer.write_all(line.as_bytes());
            let _ = master_writer.flush();
        }
        line.clear();
    }

    Ok(())
}