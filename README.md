# Plushy

[![Crates.io](https://img.shields.io/crates/v/plushy.svg)](https://crates.io/crates/plushy)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/darthdeus/plushy#license)
[![Crates.io](https://img.shields.io/crates/d/plushy.svg)](https://crates.io/crates/plushy)
[![Rust](https://github.com/darthdeus/plushy/workflows/CI/badge.svg)](https://github.com/darthdeus/plushy/actions)
[![Discord](https://img.shields.io/discord/720719762031771680.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/M8hySjuG48)

Plushy is a comfy generational arena for arbitrary types. You can think of it
as _[`thunderdome`](https://docs.rs/thunderdome) for all types at once_, or as
an ECS where you can only have one component at a time.

Plushy is a relatively simple crate that builds on top of thunderdome, but adds a few nice things, specificaly:

- Strongly typed wrappers around `Index`, meaning you can't accidentally mix up your entity ids. If you insert a `Player`, the corresponding id is `Id<Player>`.
- You only need one `Store` for all of your types. With `thunderdome` you'd need to create a separate `Arena<T>` for every type `T` you want to store.

```rust
let mut store = Store::new();

struct Enemy {
    pub x: i32,
}

struct Player {
    pub health: f32,
}

// New entities can just be spawned, we don't need to register
// the types anywhere.
store.spawn(Enemy { x: 1 });
store.spawn(Enemy { x: 2 });

// Store the player's ID for later
let player = store.spawn(Player { health: 100.0 });

assert_eq!(
    &[1, 2],
    store
        .iter::<Enemy>()
        .map(|t| t.1.x)
        .collect::<Vec<_>>()
        .as_slice()
);

// Fetch the player based on the ID. Note we don't need to write
// `store.get::<Player>(player)`, the type is inferred from the
// strongly typed ID.
assert_eq!(100.0, store.get(player).unwrap().health);

// Change player health
store.get_mut(player).unwrap().health = 200.0;

// Fetch it again and verify the change. Note we can also just directly
// index. This will panic if the `Id` is invalid.
assert_eq!(200.0, store[player].health);
```

# License

Plushy is free and open source and dual licensed under MIT and Apache 2.0 licenses.
