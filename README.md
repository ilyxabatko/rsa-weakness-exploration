# RSA cracker written in Rust

_This code isn't optimized and cannot be used in real life examples. This example is for training purposes only._

Rust representation of the RSA cracker where `d(private_key)` is derived from `e(usually 65537)` and `n(multiplication of 2 big prime numbers)`, and prime factors of `n` are weakly generated (relatively close to each other) so that we can apply Fermat's factorization method to calculate `n`. Refer to this video for a detailed explanation: [Computerphile Video](https://www.youtube.com/watch?v=-ShwJqAalOk&ab_channel=Computerphile).

## Run

- Build with `cargo build`
- Run the binary with `cargo run`
