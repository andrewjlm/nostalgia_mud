# nostalgia_mud

## Motivation

When I was in middle school, I spent countless hours in front of a Telnet terminal playing MUDs. This project is a love-letter to this style of game, in particular the [Diku](https://en.wikipedia.org/wiki/DikuMUD)/[Merc](https://github.com/alexmchale/merc-mud)/[ROM](https://github.com/avinson/rom24-quickmud) style MUDs such as [Fatal Dimensions](https://www.fataldimensions.nl/).

The core of the server uses [tokio](https://tokio.rs/) to handle Telnet connections and the game loop.

## Features

Current implemented features include:

- Handling of multiple simultaneous Telnet connections
- A partial implementation of a ROM area file format parser. Currently, only rooms, mobiles and mobile reset commands are implemented.
- Chat via `gossip` and `say` commands.
- Colored output.
- Movement throughout the world with `NSEWUD` and `look` commands.

There is still a ton of work to do such as implementing deeper interaction with the world, objects, combat, etc.

## Getting Started

If you want to try this out:

1. Clone the report.
2. Write a settings file in TOML. For example, you could create a file `settings.toml` using the following template:

```TOML
port = 4073
# Some sample areas sourced from the classic ROM distribution
areas = ["areas/midgaard.are", "areas/school.are"]
recall_vnum = 3001
```

3. Run the server with the configuration file using `cargo run -- --config-file settings.toml`. This will start a server locally on port 4073.
4. Connect to the server and explore using `telnet localhost 4073`.
