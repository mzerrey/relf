# Relf

A personal data management application built with Rust (Yew framework) and WebAssembly.

## Live Demo

https://relf-app.fly.dev

## Usage

### Daily Usage
1. Use Relf as your daily data management tool
2. At the end of each session, **don't forget to save your data**
3. Export your data regularly for backup

### LLM Workflow
Relf supports JSON import/export for LLM workflows:

#### Method 1: LLM → JSON → Relf
1. Use LLM to generate structured data in JSON format
2. Save the generated JSON file
3. Import the JSON file into Relf

#### Method 2: Relf → JSON → LLM
1. Export your data from Relf as JSON
2. Copy the JSON content
3. Paste into LLM for analysis or processing
4. Import the processed JSON back to Relf

#### Data Options
- **All data copy**: Copy all data to clipboard
- **All data paste**: Paste all data from clipboard
- **Inside only paste**: Paste data for inside content only
- **Outside only paste**: Paste data for outside content only
- **Export as file**: Export data as JSON file
- **Import as file**: Import data from JSON file

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
