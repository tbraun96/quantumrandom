# quantumrandom
**A Rust quantum-random number generator**

All credit goes to qrng.anu.edu.au for the source of quantum-random data

The ANU rng is a nearly perfect random number generator. I take the data and mix it with the rand rng library to help increase entropy in the case that the HTTPS download stream is comprimised remotely; in that case, the data would be modified based on local parameters anyways, and as such, wouldn't matter if the data is comprimised remotely.

The ANU API limits requests to 1024 random numbers at a time per connection*. This program allows you to retrieve more than that.

```
extern crate QuantumRandom;

    let mut idx = 0;
    let loop_display = match QuantumRandom::random::next_u128s(40) {
        Some(T) => T,
        _ => panic!("Please check your internet connection. Halting execution")
    }.into_iter().for_each( |value| {
        println!("[{}] {}", idx, value);
        idx = idx+1;
    });
```

You may view my website here: https://thomaspbraun.com/
