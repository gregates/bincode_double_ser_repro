`cargo run` this repo to see that bincode calls custom serialize function twice, while serde_json does so only once.

The output does not contain two copies of the serialized data, but nevertheless the function is invoked twice.
