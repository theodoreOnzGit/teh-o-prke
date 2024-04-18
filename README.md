# teh-o-prke
Point Reactor Kinetics Equations Module for the Teh-O package

Teh-O is the Transport, Eigenvalue and Hybrid Open Source Solver. It is meant to 
sound like Teh-O from Southeast Asia (Singapore Specifically).


## zero power PRKE
The zero power PRKE is split into two parts, the server and client.
Here, we don't have temperature feedback or whatsoever
To run the server:

```sh
cargo run --example zero_power_server --release
```

To run the client:

```sh
cargo run --example zero_power_client --release
```

Please remember to run the client AFTER the server.

# prerequisites

You'll need openblas to run this on linux.


