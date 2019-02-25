# quantumrandom
**A Rust quantum-random number generator**

All credit goes to qrng.anu.edu.au for the source of quantum-random data

The ANU rng is a nearly perfect random number generator. I take the data and mix it with the rand rng library to help increase entropy in the case that the HTTPS download stream is comprimised remotely; in that case, the data would be modified based on local parameters anyways, and as such, wouldn't matter if the data is comprimised remotely.

The ANU API limits requests to 1024 random numbers at a time per connection*. This program allows you to retrieve more than that.

```
extern crate QuantumRandom;
let u128s : Vec<u128>  = *QuantumRandom::random::next_u128s(40); //40 = number of rands you want to fetch
let mut idx = 0;
for val in u128s.iter(){
    println!("{}: {}", idx, val);
    idx+=1;
}
```
