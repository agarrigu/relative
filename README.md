# rELaTivE

## A game about music, built with Rust and Simple DirectMedia Layer

At the moment a Linux Exclusive Title, but feel free to fork and create a port!

### Setting up the Development Enviroment:

This has been tested only in Debian Testing (Bookworm), though it should work
most distros.

- Install rust:
  ```sh
  $ sudo apt-get install rust-all
  ```

- Install SDL2:
  ```sh
  $ sudo apt-get install libsdl2-dev libsdl2-ttf-dev
  ```

- Clone the repository and change directory:
  ```sh
  $ git clone https://github.com/agarrigu/relative
  $ cd relative
  ```

- Build Cargo packages:
  ```sh
  $ cargo build
  ```

- Run Game:
  ```sh
  $ cargo run
  ```
