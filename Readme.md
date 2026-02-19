# beam_node

Rust library for connecting to Erlang nodes via EI (Erlang Interface).

## Overview

beam_node provides a Rust interface to Erlang's EI library, enabling Rust applications to communicate with Erlang nodes. It allows sending and receiving Erlang terms, connecting to remote nodes, and working with Erlang processes.

## Usage

```rust
use beam_node::{Node, Term};

let mut node = Node::new("mynode@hostname", "cookie")?;
let mut conn = node.connect("server@hostname")?;

conn.send(&pid, "hello")?;

let msg: Term = conn.receive()?;
```

## Features

- Connect to Erlang nodes
- Send and receive Erlang terms
- Support for PID handling
- Configurable timeouts for receive operations

## Requirements

- Erlang/OTP development libraries (ei)
- Rust 2021 edition

## Installation

Ensure you have Erlang installed, then build the project:

```bash
cargo build
```

## License

MIT
