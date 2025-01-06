# Schnorr Signature

This project demonstrates a Schnorr signature scheme implementation using the Rust programming language. Schnorr signatures provide a simple, efficient, and secure way to sign messages using elliptic curve cryptography.

## Features

- Key generation (private and public keys)
- Message signing
- Signature verification

## Prerequisites

- Rust (v1.65 or higher recommended)

## Dependencies

This project uses the following Rust crates:

- [`curve25519-dalek`](https://crates.io/crates/curve25519-dalek): Provides elliptic curve operations on the Ristretto group.
- [`rand`](https://crates.io/crates/rand): Used for generating cryptographically secure random numbers.
- [`sha2`](https://crates.io/crates/sha2): Provides the SHA-512 hashing algorithm.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/schnorr_signature.git
   cd schnorr_signature
   ```

2. Ensure you have Rust installed. If not, install it using [rustup](https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. Build the project:
   ```bash
   cargo build
   ```

4. Run the example:
   ```bash
   cargo run
   ```

## Usage

The main components of the implementation include:

### Key Generation
Generate a private and public key pair:
```rust
let (private_key, public_key) = keygen();
```

### Signing
Sign a message with the private key:
```rust
let message = b"Hello, Schnorr!";
let signature = sign(message, private_key);
```

### Verification
Verify the signature using the public key:
```rust
let is_valid = verify(message, &signature, public_key);
println!("Signature valid: {}", is_valid);
```
## Example Output

When you run the program, it will:

1. Generate a key pair (private and public keys).
2. Sign the message `"Hello, Schnorr!"`.
3. Verify the signature and display whether it is valid.

Expected output:
```text
Signature valid: true
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [curve25519-dalek](https://github.com/dalek-cryptography/curve25519-dalek) for the Ristretto implementation.
- [Rust](https://www.rust-lang.org/) for being an amazing systems programming language.

---

