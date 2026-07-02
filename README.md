# agent-proxy

A low-latency, systems-level PTY proxy for AI agents, featuring autonomous LLM-in-the-loop security guardrails and universal AST-based token compression.

## Technical Overview

`agent-proxy` implements a pseudo-terminal (PTY) wrapper in Rust. It functions as a transparent middleware layer between a parent shell and a child process, intercepting the `stdin`/`stdout` byte stream to perform real-time security heuristic filtering and on-demand AST minification.

## Architecture

* **PTY Bridge:** Uses `portable-pty` to handle POSIX terminal emulation.
* **AST Parser:** Integrates `tree-sitter` C-bindings to perform language-agnostic structural parsing.
* **Security Logic:** Implements a byte-stream filter that detects destructive shell patterns (`rm -rf`, `git push -f`) and injects synthetic terminal errors back to the caller process, preventing rogue execution without process termination.
* **Visualizer:** A recursive directory crawler that outputs a JSON-based structure for `vis.js` graph rendering.

## Build Requirements

* `rustc` 1.80+ (or latest stable)
* `cargo`
* Build essentials (for `tree-sitter` C-binding compilation)

```bash
sudo apt-get install build-essential # Required for C-bindings

```

## Installation

```bash
git clone https://github.com/abhiprd2000/agent-proxy.git
cd agent-proxy
cargo build --release

```

## Usage

The proxy initializes a sub-shell upon launch. All commands pass through unless they match the intercepted patterns.

### Token Compression (`cat-min`)

Invokes the `tree-sitter` parser to prune non-essential nodes (comments, documentation, redundant whitespace) from source files.

**Supported Grammars:**

* `rust` (`.rs`)
* `python` (`.py`)

*To extend support to new languages, add the dependency to `Cargo.toml` and update the `match` arm in `src/ast.rs`.*

### Security Interception

The proxy monitors the byte stream for blacklisted patterns.

* **Trigger:** `rm -rf`, `git push -f`, `drop`
* **Response:** Packet drop + synthetic `stderr` injection.
* **Config:** See `src/main.rs` line ~45 to extend the blacklist.

### Visualization (`map-dir`)

Generates `aegis-map.html` in the root directory. This uses a physics-based `vis.js` engine to render the dependency graph.

## Developer Configuration

To extend the AST parser, ensure the language grammar is included in `Cargo.toml` and updated in the `compress_code` function in `src/ast.rs`:

```rust
// Example grammar registration
"js" => tree_sitter_javascript::LANGUAGE.into(),

```

## Security Policy

This tool is designed to protect local environments. It does not provide network-level sandbox isolation. Do not treat this as a replacement for containerization (e.g., Docker) for untrusted code execution.

## License

MIT