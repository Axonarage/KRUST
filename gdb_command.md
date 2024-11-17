## Basic Commands:

### Start execution
```
continue
```

### Step through instructions

```
step   # Step into
next   # Step over
```

### Set a breakpoint

```
break main
``` 
Replace main with the desired function or memory address.

### Inspect registers

```
info registers
``` 

### Inspect memory

```
x/10x 0x20000000
```
(Example to view 10 words at address 0x20000000 in hexadecimal.)

## Advanced Commands

### Load symbols

``` 
file target/thumbv7em-none-eabihf/debug/krust
```

### View backtrace
```
backtrace
```

### Restart execution
```
monitor reset
```
Sends a reset signal to the emulator

### Get functions from address

```
info symbol 0x080001cb
```
out : `HardFaultHandler + 1 in section .text`

