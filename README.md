# Multi-Threaded Downloader

A high-performance, multi-threaded file downloader written in Rust. This project implements parallel chunk downloading to maximize download speeds by utilizing multiple threads to download different parts of a file simultaneously.

## Features

- **Multi-threaded Downloads**: Downloads files in parallel chunks using multiple threads
- **Range Request Support**: Automatically detects and uses HTTP range requests when supported by the server
- **Chunk-based Architecture**: Splits large files into manageable chunks for efficient parallel processing
- **Automatic Merging**: Seamlessly combines downloaded chunks into the final file
- **Progress Tracking**: (Planned) Real-time download progress and speed monitoring
- **Resume Capability**: (Planned) Ability to resume interrupted downloads

## Requirements

- Rust 1.70 or later
- Cargo (Rust's package manager)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd downloader
```

2. Build the project:
```bash
cargo build --release
```

3. Run the downloader:
```bash
cargo run --release
```

## Usage

Currently, the downloader is configured to download a specific file. You can modify the URL in `src/main.rs` to download different files.

### Example

The current implementation downloads the Linux kernel source code:
```rust
let url = "https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.18.8.tar.xz";
```

### How It Works

1. **Header Request**: Sends a HEAD request to check:
   - File size (`content-length`)
   - Content type
   - Range request support (`accept-ranges`)

2. **Chunk Division**: Splits the file into 8 equal chunks (configurable)

3. **Parallel Download**: Downloads each chunk in parallel using range requests

4. **File Merging**: Combines all chunks into the final output file

## Project Status

🚧 **Work in Progress**

The project is currently under active development. The basic multi-threaded download functionality is implemented, with plans for:

- [ ] Async/await implementation using Tokio
- [ ] Configurable thread count
- [ ] Progress bars and download statistics
- [ ] Resume functionality for interrupted downloads
- [ ] Command-line interface (CLI) for easy usage
- [ ] Support for multiple file downloads
- [ ] Error handling and retry logic
- [ ] Download speed limiting/throttling

## Dependencies

- **reqwest**: HTTP client library for making requests
- **tokio**: Async runtime for Rust (prepared for future async implementation)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

See [LICENCE](LICENCE) file for details.

## Future Enhancements

- Async/await implementation for better performance
- CLI with command-line arguments
- Configuration file support
- Download queue management
- Bandwidth throttling
- Proxy support
- SSL/TLS certificate validation options
