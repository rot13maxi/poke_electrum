# Wut?

I wanted to play with the electrum API. So this is a little rust program that pokes an electrum server and gets history for an address.

Feel free to change this line:
```rust
    let client = Client::new("tcp://kirsche.emzy.de:50001")?;
```
to point at your electrum server. Change the next line for the address you want to examine. Maybe I'll make it a CLI argument. Maybe not. This is a *very* low-effort block of hacks for me. PRs welcome!

Once you have those two values dialed in, just do `cargo run` and it'll compile and run.

If you don't have `cargo`, go install rustup and follow the directions. 