# Polkadot-rust-project
Example of building a parachain.

#### Each parachain is compiled into two executables:

from a natively compiled language (Rust, JavaScript, C++)

Wasm Code BLOB

WebAssembly (Wasm) is a binary instruction format for a stack-based virtual machine. 
It is designed to be portable and run almost anywhere, as it is quite close to native code. For now, Wasm is supported by Google, Mozilla, Apple and Microsoft.

#### Preparing the environment:
The minimum set of tools used by Polkadot:

rust
WebAssembly Compiler
Polkadot source code

The first step is to install the Rust language compiler and some additional tools such as the WebAssembly compiler and the project generator. Polkadot is currently written in Rust.

#### Install Rust and additional tools:

curl https://sh.rustup.rs -sSf | sh
rustup update nightly
rustup default nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

After Rust has been installed with Wasm support, it is time to install Wasm itself. 
The tool for this – cargo – is a default Rust project builder.

cargo install --git https://github.com/alexcrichton/wasm-gc
Now we need the Polkadot source code, as it contains the API needed for our parachain and collator. 

After installation, test the components to make sure the build is stable:

git clone https://github.com/paritytech/polkadot.git
cd polkadot
./scripts/build.sh # Builds the WebAssembly binaries
cargo build # Builds all native code
cargo test --all

#### Structure and components:
The structure of the parachain in the project is very simple. It can be described in three folders.

P1) Collator

This folder contains a main.rs file with the code for the full comparison agent executed by the client. 
It can also contain custom modules, a cargo build file with dependencies, and other support files.

P2) src/lib.rs

This contains the logic of the parachin itself. 
It can also contain custom forms, load compilation files, etc.

P3) wasm/lib.rs

This file contains the API, which will be visible to the forwarding chain validators. 
Most often, it contains wrappers around functions from the src folder or even a single-line import of all functions/public interfaces 
from that file.

The project also contains a Cargo.toml file (possibly for each folder), which contains the dependencies used in the code: 
the API to access the parachain, the API to run binders, standard Polkadot types, etc. 
The Cargo.toml file may look like the following:

[package]
name = "custom_parachain"
version = "0.1.0"
[dependencies]
polkadot-parachain = { path = "<path to polkadot/parachain>" }
polkadot-collator = { path = "<path to polkadot/collator>" }
polkadot-primitives = { path = "<path to polkadot/primitives>" }


