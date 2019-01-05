
```bash
$ cargo install cargo-xbuild
$ git clone https://github.com/osdev-rs/minimal-kernel-rpi2.git
$ cd minimal_kernel_rpi2
$ cargo xbuild --target arm-none-eabihf.json
$ qemu-system-arm -m 256M -M raspi2 -kernel target/arm-none-eabihf/debug/minimal-kernel-rpi2 -serial stdio
```
