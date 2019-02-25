# quantumrandom
A Rust quantum-random number generator
All credit goes to qrng.anu.edu.au for the source of quantum-random data

The ANU rng is a nearly perfect random number generator. I take the data and mix it with the rand rng library to help increase entropy in the case that the HTTPS download stream is comprimised remotely; in that case, the data would be modified based on local parameters anyways, and as such, wouldn't matter if the data is comprimised remotely.

```
extern crate QuantumRandom;
let bytes : Vec<u128>  = *QuantumRandom::random::next_u128s(40);
let mut idx = 0;
for byte in bytes.iter(){
    println!("{}: {}", idx, byte);
    idx+=1;
}
```
