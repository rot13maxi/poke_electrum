# Wut?

I wanted to play with the electrum API. So this is a little rust program that pokes an electrum server and gets history for an address.

If you don't have `cargo`, go install rustup and follow the directions. 

Do `cargo build --release` to build it. Then do `./target/release/poke_electrum [your electrum server IP/hostname and port] [the address you want to lookup]`. For example, if your electrum server is at 192.168.1.50, you can do `./target/release/poke_electrum 192.168.1.50:50001 bc1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx` to look up the history for `bc1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx` with your electrum server