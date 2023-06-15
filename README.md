
# rELaTivE

<a href='http://www.recurse.com' title='Made with love at the Recurse Center'><img src='https://cloud.githubusercontent.com/assets/2883345/11325206/336ea5f4-9150-11e5-9e90-d86ad31993d8.png' height='20px'/></a>

## Intro

A ~game~ interactive audio visual square moving experience about music,
built with Rust and Simple DirectMedia Layer.
Used as an exuse to learn Rust, digital audio and SDL2 during my
[Recurse Center](https://www.recurse.com) batch,
the premise of the game is to solve puzzles using relative pitch,
as opposed to most music games, where rhythm is used.

There is no deffinite design yet, just a bunch of systems interacting with each
other making notes happen.
The only available level is won by moving the avatar to y position 300,
not very fun.


### Setting up the Development Enviroment:

This has been tested only in Debian Testing (Bookworm), though it should work
most distros.

Install dependencies:

```sh
$ sudo apt-get install libsdl2-dev libsdl2-ttf-dev rust-all
```

Clone the repository and change directory:

```sh
$ git clone https://github.com/agarrigu/relative
$ cd relative
```

Build Cargo packages:

```sh
$ cargo build
```

Run Game:

```sh
$ cargo run
```

### Play

Use WASD to move
