# Relf

A personal data management application built with Rust (Yew framework) and WebAssembly.

## Live Demo

https://relf-app.fly.dev

## Relf Format

### Outside
External resources and references:
- **Name**: Title or identifier of the resource
- **Context**: Description or notes about the resource
- **URL**: Web address or link
- **Percentage**: Score or progress indicator, sortable for ordering

### Inside
Internal notes or thoughts with timestamps:
- **Date**: Timestamp of the entry, sortable for ordering
- **Context**: notes or thoughts

```json
{
  "outside": [
    {
      "name": "Rust Programming Language",
      "context": "A systems programming language focused on safety, speed, and concurrency.",
      "url": "https://www.rust-lang.org/",
      "percentage": 90
    }
  ],
  "inside": [
    {
      "date": "2024-01-01 10:00:00",
      "context": "Finally learned how to use cargo! Running 'cargo new my_project' creates such a clean project structure."
    }
  ]
}
```

This format is also available in [github.com/rlelf/revw](https://github.com/rlelf/revw)

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

### Option 1: Local Development with Trunk
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
