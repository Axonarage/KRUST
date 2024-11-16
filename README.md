# Krust

## Installation de la version adéquate de rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup install nightly

rustup default nightly

## Installation de la cible
rustup target add thumbv7em-none-eabihf

## Simple cargo build
cargo build

## Divers
La target qemu utilisée pour les test sur Cortex-M4 est la netduinoplus2 (microcontrolleur STM32F405RGT6)
