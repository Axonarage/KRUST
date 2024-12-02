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

## Debug with gdb

1. `cargo build`
2. Launch `./tools/krust_gdb`
3. `arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/krust`
4. In gdb, `target remote :3333`

### Install `arm-none-eabi-gdb`

```bash
apt install gdb-multiarch
ln -s /usr/bin/gdb-multiarch /usr/bin/arm-none-eabi-gdb
``` 
## Version qemu

`qemu-system-arm` : version 7.2.13
