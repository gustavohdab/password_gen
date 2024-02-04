# Password Generator

This is a simple, yet powerful password generator written in Rust. It generates a random password of a given length, which is a mix of uppercase letters, lowercase letters, numbers, and symbols. The generated password is copied to the clipboard automatically and printed to the console.

## Features

- Generate a random password of a given length
- The password includes uppercase letters, lowercase letters, numbers, and symbols
- The generated password is copied to the clipboard automatically
- The generated password is printed to the console
- The length of the password can be configured via a command line argument
- The password is generated using a cryptographically secure random number generator

## Usage

To run the program, use the following command:

```bash
cargo run --package password_gen --bin password_gen
```

It will prompt you to enter the length of the password. After entering the length, the generated password will be copied to the clipboard.

## Future Improvements

Implement a command to view the history of all generated passwords
Allow the user to enter the command "/history" to see all generated passwords
Support the command "/history " to view a specific password
Prompt the user to provide an identifier for each generated password for easy identification and management

## Testing

The project includes a basic unit test for the generate_password function. To run the tests, use the following command:

```bash
cargo test --package password_gen
```

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
