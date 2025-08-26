# Relf

A personal data management application built with Rust and WebAssembly.

## Demo Site

🚀 **Live Demo**: https://relf-app.fly.dev

## Quick Start

### Prerequisites

- Rust (latest stable version)
- Trunk: `cargo install trunk`

### Option 1: Local Development with Trunk (Recommended)
```bash
# Install Trunk if not already installed
cargo install trunk

# Run development server with hot reload
trunk serve

# Access at http://localhost:8080
```

### Option 2: Production Build
```bash
# Build for production
trunk build --release

# Run the server
cargo run --release --bin server --features server

# Access at http://localhost:5000
```

## Features

- **Offline-First**: Browser local storage
- **Responsive**: Desktop and mobile support

### Option 3: Docker Deployment

```bash
# Build and run with Docker
./docker-run.sh run

# View logs
./docker-run.sh logs

# Stop application
./docker-run.sh stop
```


## License

MIT