# Plus codes

Plus codes is an implementation of [plus codes](https://plus.codes) in Rust. It can be used either as a command line tool or as a crate.

# As command line tool

- Install and setup Rust and Cargo
- `cargo install pluscodes`

Try it out with:

```
pluscodes help
pluscodes encode 59.335938,18.077813
pluscodes encode 59.335938,18.077813 --length 6
pluscodes decode 9FFW83PH+94
```

# As a Rust crate

```
cargo add pluscodes

let coord = pluscodes::Coord {
    latitude: 59.335938,
    longitude: 18.077813,
};
pluscodes::encode(&coord, 10) // => "9FFW83PH+94"
pluscodes::encode(&coord, 6) // => "9FFW8300+"
pluscodes::decode("9FFW83PH+94") // => pluscodes::Coord {...}

```
