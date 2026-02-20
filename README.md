# Neflo

Neflo is a simple, lightweight CLI application for macOS that tracks your focus and idle time. It helps you understand your productivity patterns by monitoring system activity and generating reports on your daily and weekly "flow" sessions versus interruptions.

## Features

- **Activity Tracking**: Monitors system-wide keyboard and mouse activity to detect when you are actively working.
- **Smart Idle Detection**: Automatically transitions to "Idle" state after a configurable period of inactivity (default: 5 minutes).
- **Persistence**: Automatically stores all tracking data locally in JSON format.
- **Reporting**: Generates detailed daily and weekly summaries, including:
    - Total focus and idle time.
    - Number of interruptions.
    - Average length of focus and idle sessions.
- **Configurable**: Easily adjust the idle threshold via command-line flags or a configuration file.

## Documentation

Comprehensive documentation is available in the [doc/](doc/index.md) folder:

- [**Introduction**](doc/introduction.md): Overview of Neflo and its features.
- [**Setup**](doc/setup.md): System requirements and installation instructions.
- [**Usage**](doc/usage.md): Guide on how to use the CLI and the TUI dashboard.
- [**Architecture**](doc/architecture.md): Technical details about the project structure and implementation.
- [**Development**](doc/development.md): How to build, test, and contribute to the project.
- [**Publishing**](doc/publishing.md): Information on the release and publishing process.

## Requirements

- **Operating System**: macOS (for system-wide activity detection).
- **Rust**: Version 1.85 or later is recommended.

## Installation

To build Neflo from source:

```bash
cargo build --release
```

The binary will be available at `target/release/neflo`. You can move it to your `/usr/local/bin` or add the directory to your PATH.

## Usage

### Start Tracking

To start the focus tracker in the foreground:

```bash
neflo start
```

You can specify a custom idle threshold (in minutes) using the `--threshold` flag:

```bash
neflo start --threshold 10
```

To stop tracking, simply press `Ctrl+C`.

### View Reports

To see your focus and idle statistics for the current day and week:

```bash
neflo report
```

Neflo calculates the "current week" starting from Monday.

## Configuration

Neflo stores its data and configuration in the `~/.neflo` directory:

- `~/.neflo/db.json`: Contains the recorded focus and idle intervals.
- `~/.neflo/config.json`: Stores default settings.

Example `config.json`:
```json
{
  "default_threshold_mins": 5
}
```

## Development and Contribution

### Project Structure

- `src/main.rs`: Entry point and CLI argument parsing.
- `src/tracker.rs`: Core tracking loop and state transition logic.
- `src/report.rs`: Logic for aggregating data and displaying reports.
- `src/system.rs`: macOS-specific FFI calls for system idle time.
- `src/storage.rs`: JSON persistence layer.
- `src/models.rs`: Data structures for intervals and the database.
- `src/config.rs`: Configuration management.

### Running Tests

To run the unit tests:

```bash
cargo test
```

### Contributing

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Ensure your code follows Rust best practices and passes `cargo check`.
4. Submit a pull request with a clear description of your changes.

## License

MIT (or your preferred license)
