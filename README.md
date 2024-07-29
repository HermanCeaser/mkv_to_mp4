# MKV to MP4 Converter

This Rust program converts `.mkv` files to `.mp4` files using `ffmpeg`. It also gives you the option to delete the original `.mkv` files after conversion.

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- `ffmpeg` installed on your system. The program will attempt to install `ffmpeg` if it is not found.

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/hermanceaser/mkv_to_mp4.git
    cd mkv_to_mp4
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

## Usage

1. Navigate to the directory containing the `.mkv` files you want to convert.

2. Run the program:
    ```sh
    cargo run --release
    ```

3. Follow the on-screen prompt to decide whether to delete the original `.mkv` files after conversion.

### Example

Assume you have the following files in your directory:
- video1.mkv
- video2.mkv


Running the program:
```sh
cargo run --release
```

You will see a prompt:
```
Do you wish original .mkv files to be deleted when finished? [y/N]:
```

- If you type `y` and press Enter, the `.mkv` files will be deleted after conversion.
- If you type `n` or press Enter without typing anything, the `.mkv` files will be retained.

After conversion, you should see the corresponding `.mp4` files in the same directory:
```
video1.mp4
video2.mp4
```

## Troubleshooting

- If `ffmpeg` is not installed on your system, the program will attempt to install it using `sudo apt install ffmpeg`.
- Ensure you have the necessary permissions to install packages and remove files when running the program.

## Contributing

1. Fork the repository.
2. Create a new branch: `git checkout -b feature-branch`.
3. Make your changes and commit them: `git commit -m 'Add some feature'`.
4. Push to the branch: `git push origin feature-branch`.
5. Open a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.