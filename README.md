# Azalea

[![docs.rs](https://img.shields.io/docsrs/azalea)](https://docs.rs/azalea)

A collection of Rust crates for making Minecraft bots, clients, and tools.

<p align="center">
    <img src="https://github.com/azalea-rs/azalea/assets/27899617/b98a42df-5cf0-4d1f-ae7c-ecca333e3cab" alt="Azalea" height="200">
</p>


<!-- The line below is automatically read and updated by the migrate script, so don't change it manually. -->

_Currently supported Minecraft version: `25w03a`._

> [!WARNING]
> Azalea is still unfinished, though most crates are in a useable state

## Features

-   [Accurate physics](https://github.com/azalea-rs/azalea/blob/main/azalea-physics/src/lib.rs) (but some features like entity collisions and elytras aren't yet implemented)
-   [Pathfinder](https://azalea.matdoes.dev/azalea/pathfinder/index.html)
-   [Swarms](https://azalea.matdoes.dev/azalea/swarm/index.html)
-   [Breaking blocks](https://azalea.matdoes.dev/azalea/struct.Client.html#method.mine)
-   [Block interactions & building](https://azalea.matdoes.dev/azalea/struct.Client.html#method.block_interact) (this doesn't predict the block interactions/placement on the client yet but it's usually fine)
-   [Inventories](https://azalea.matdoes.dev/azalea/struct.Client.html#impl-ContainerClientExt-for-Client)
-   [Attacking entities](https://azalea.matdoes.dev/azalea/struct.Client.html#method.attack) (but you can't get the entity at the crosshair yet)

## Docs

The "stable" documentation is available at [docs.rs/azalea](https://docs.rs/azalea) and the unstable docs are at [azalea.matdoes.dev](https://azalea.matdoes.dev) (recommended).

## Matrix/Discord

If you'd like to chat about Azalea, you can join the Matrix space at [#azalea:matdoes.dev](https://matrix.to/#/#azalea:matdoes.dev) (recommended) or the Discord server at [discord.gg/FaRey6ytmC](https://discord.gg/FaRey6ytmC) (they're bridged so you don't need to join both).

## Goals

-   Do everything a vanilla client can do.
-   Be intuitive and easy to use.
-   Make it easy to have many bots working at the same time.
-   Don't trigger anticheats.
-   Support the latest Minecraft version.
-   Be fast and memory efficient.

## Non-goals

-   Supporting several versions of Minecraft on the same branch[\*](https://github.com/azalea-rs/azalea-viaversion).
-   Bedrock edition.
-   Graphics.

## Branches

There are several branches in the Azalea repository that target older Minecraft versions.
Most of them are severely outdated compared to the latest version of Azalea.
If you'd like to update them or add more, please open a PR.

-   [1.21.2-1.21.3](https://github.com/azalea-rs/azalea/tree/1.21.3)
-   [1.21-1.21.1](https://github.com/azalea-rs/azalea/tree/1.21.1)
-   [1.20.5-1.20.6](https://github.com/azalea-rs/azalea/tree/1.20.6)
-   [1.20.4](https://github.com/azalea-rs/azalea/tree/1.20.4)
-   [1.20.2](https://github.com/azalea-rs/azalea/tree/1.20.2)
-   [1.20-1.20.1](https://github.com/azalea-rs/azalea/tree/1.20.1)
-   [1.19.4](https://github.com/azalea-rs/azalea/tree/1.19.4)
-   [1.19.3](https://github.com/azalea-rs/azalea/tree/1.19.3)
-   [1.19.2](https://github.com/azalea-rs/azalea/tree/1.19.2)
