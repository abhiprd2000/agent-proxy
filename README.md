# AgentProxy

A firewall between AI coding agents and your terminal.

AgentProxy sits between an AI agent and a shell, intercepting commands before execution.

It allows developers to:

* Block destructive commands
* Review risky actions
* Audit agent behavior
* Enforce terminal policies

## Why?

AI coding agents can execute shell commands.

Most terminal environments provide little visibility into what an agent is about to do.

AgentProxy adds a policy layer between the agent and the operating system.

## Current Features

### Command Interception

AgentProxy runs a shell through a PTY proxy and inspects commands before execution.

Examples:

* `rm -rf`
* `git push --force`
* custom blocked commands

### AST Compression

Built-in Tree-sitter support for:

* Rust
* Python

Useful for token reduction and code analysis workflows.

### Codebase Visualization

Generate an interactive graph of a repository structure.

## Architecture

AI Agent

↓

AgentProxy

↓

Shell (bash)

↓

Operating System

## Installation

```bash
git clone https://github.com/abhiprd2000/agent-proxy.git
cd agent-proxy
cargo build --release
```

## Roadmap

* [ ] Policy engine
* [ ] YAML configuration
* [ ] Risk scoring
* [ ] Approval workflows
* [ ] Audit logs
* [ ] Claude Code integration
* [ ] Codex integration
* [ ] OpenHands integration

## Disclaimer

AgentProxy is not a sandbox and should not be considered a security boundary.

It is a command interception and policy enforcement layer.
