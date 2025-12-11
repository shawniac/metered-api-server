# Metered API Server

A metered API server built with Warp and SQLx in Rust.

## Description

This project provides a simple API server with metering capabilities, allowing you to track and limit API usage per client.

## Features

- Asynchronous HTTP server using Warp
- Database integration with SQLx (SQLite)
- In-memory rate limiting and usage metering
- Structured logging with tracing
- Environment-based configuration

## Installation

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- SQLite (for database, or configure for other DB)

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/metered-api-server.git
   cd metered-api-server
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

## Usage

1. Create a `.env` file in the root directory:
   ```
   DATABASE_URL=sqlite://database.db
   SERVER_PORT=3030
   RUST_LOG=info
   ```

2. Run the server:
   ```bash
   cargo run
   ```

The server will start on `http://localhost:3030`.

## API Endpoints

- `GET /health` - Health check endpoint
- `POST /api/data` - Example metered endpoint (requires API key)
- `GET /metrics` - Usage metrics (admin endpoint)

## Configuration

Environment variables:

- `DATABASE_URL`: Database connection string
- `SERVER_PORT`: Server port (default: 3030)
- `RUST_LOG`: Log level (default: info)

## Development

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

### Documentation

```bash
cargo doc --open
```

## License

MIT