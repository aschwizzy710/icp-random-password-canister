# ICP Rust Password Generator

This canister generates a random strong password of specified length using Rust on the Internet Computer. Created for the ICP Rust Smart Contract Challenge 101 on the Dacade platform.

## Prerequisites

- Rust
- DFINITY SDK
- Cargo

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/aschwizzy710/icp-random-password-canister.git
   cd icp-random-password-canister

2. Build the canister:
   cargo build --target wasm32-unknown-unknown --release -p my_rust_canister_backend

3. Start the local Internet Computer:
   dfx start --background

4. Create and deploy the canister:
   dfx canister create my_rust_canister_backend
   dfx build my_rust_canister_backend
   dfx canister install my_rust_canister_backend --mode=reinstall

## Usage

1. Call the generate_password function with the desired password length:
   dfx canister call my_rust_canister_backend generate_password '(12: nat64)'

## License

MIT License

Copyright (c) 2024 Aaron Schwartz

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.