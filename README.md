# rust-ecs [![Rust](https://github.com/andreban/rust-ecs/actions/workflows/rust.yml/badge.svg)](https://github.com/andreban/rust-ecs/actions/workflows/rust.yml)

A simple ECS.

## Goals

The goal of this project is to be a learning exercise for the author, to understand the [Entity Component System][1](ECS) architecture and how to implement it with Rust.

The architecture is heavily inspired by the [C++ Game Engine Programming][2] course, but adapted to (try) being more idiomatic to Rust.

## Running the Demo

The demo project uses the assets from the [C++ Game Engine Programming][2] course. In order to run it, downloads the assets from the original course and move the `assets` folder into the `demo` folder of this project.

To run the demo, cd into the `demo` directory and run `cargo run`.

[1]: https://en.wikipedia.org/wiki/Entity_component_system
[2]: https://pikuma.com/courses/cpp-2d-game-engine-development
