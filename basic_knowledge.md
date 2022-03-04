
<!-- concepts -->

 standard library:

 The panic_handler attribute defines the function that the compiler should invoke when a panic occurs

runtime system: is responsible for things such as garbage collection (e.g. in Java) or software threads (e.g. goroutines in Go). This runtime needs to be called before main, since it needs to initialize itself.

In a typical Rust binary that links the standard library, execution starts in a C runtime library called crt0 (“C runtime zero”), which sets up the environment for a C application.

Rust only has a very minimal runtime, which takes care of some small things such as setting up stack overflow guards or printing a backtrace on panic. The runtime then finally calls the main function.

1. Minimal freestanding Rust binary
2. A Minimal Rust Kernel


we create a minimal 64-bit Rust kernel for the x86 architecture. We build upon the freestanding Rust binary from the previous post to create a bootable disk image, that prints something to the screen.


errors:
cargo build --target x86_64-mini_os.json
error: "/Users/hq/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/Cargo.lock" does not exist, unable to build with the standard library, try:
        rustup component add rust-src --toolchain nightly-x86_64-apple-darwin


download qemu use: qemu-system-x86_64 -drive format=raw,file=target/x86_64-mini_os/debug/bootimage-mini_os.bin
to start the image.