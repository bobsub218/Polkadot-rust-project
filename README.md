# Polkadot-rust-project
Example of building a parachain


Ogni parachain è compilato in due eseguibili:

da un linguaggio compilato nativamente (Rust, JavaScript, C++)

BLOB del codice Wasm

WebAssembly (Wasm) è un formato di istruzione binario per una macchina virtuale basata su stack. 
È stato progettato per essere portatile e per essere eseguito quasi ovunque, in quanto è abbastanza vicino al codice nativo. Per ora, Wasm è supportato da Google, Mozilla, Apple e Microsoft.


Preparare l'ambiente:
Il set minimo di strumenti utilizzati da Polkadot:

ruggine
Compilatore WebAssembly
Codice sorgente Polkadot

Il primo passaggio consiste nell'installare il compilatore del linguaggio Rust e alcuni strumenti aggiuntivi come il compilatore WebAssembly e il generatore di progetti. Polkadot è attualmente scritto in Rust.


Installare Rust e strumenti aggiuntivi:

curl https://sh.rustup.rs -sSf | sh
rustup update nightly
rustup default nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

Dopo che Rust è stato installato con il supporto Wasm, è il momento di installare Wasm stesso. 
Lo strumento per questo - cargo - è un costruttore di progetti Rust predefinito.

cargo install --git https://github.com/alexcrichton/wasm-gc
Ora abbiamo bisogno del codice sorgente Polkadot, in quanto contiene l'API necessaria per il nostro parachain e collator. 

Dopo l'installazione,testare i componenti per assicurarsi che la compilazione sia stabile:

git clone https://github.com/paritytech/polkadot.git
cd polkadot
./scripts/build.sh  # Builds the WebAssembly binaries
cargo build # Builds all native code
cargo test --all





Struttura e componenti:
La struttura del parachain nel progetto è molto semplice. Può essere descritto in tre cartelle.

P1) Collator

Questa cartella contiene un main.rs file con il codice per l'agente di confronto completo eseguito dal client. 
Può anche contenere moduli personalizzati, un file di compilazione cargo con dipendenze e altri file di supporto.

P2) src/lib.rs

Questo contiene la logica della paracacina stessa. 
Può anche contenere moduli personalizzati, file di compilazione del carico, ecc.

P3) wasm/lib.rs

Questo file contiene l'API, che sarà visibile ai validatori della catena di inoltro. 
Più spesso, contiene wrapper intorno alle funzioni dalla cartella src o anche un'importazione di una singola riga di tutte le funzioni / interfacce pubbliche 
da quella cartella.

Il progetto contiene anche un file Cargo.toml (possibilmente per ogni cartella), che contiene le dipendenze utilizzate nel codice: 
l'API per accedere al parachain, l'API per l'esecuzione di fascicolatori, tipi Polkadot standard, ecc. 
Il file Cargo.toml può essere simile al seguente:

[package]
name = "custom_parachain"
version = "0.1.0"
[dependencies]
polkadot-parachain = { path = "<path to polkadot/parachain>" }
polkadot-collator = { path = "<path to polkadot/collator>" }
polkadot-primitives = { path = "<path to polkadot/primitives>" }


