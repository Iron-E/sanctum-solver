# Sanctum Solver

`sanctum-solver` is a [longest path](https://en.wikipedia.org/wiki/Longest_path_problem) calculator designed for use with [Sanctum](https://store.steampowered.com/app/91600/Sanctum/) and [Sanctum 2](https://store.steampowered.com/app/210770/Sanctum_2/) (although it may have other applications).

## Installation

Requirements:

* [`cargo`](https://github.com/rust-lang/cargo)

```sh
cargo install --git https://github.com/Iron-E/sanctum-solver --root=<desired install folder>
```

## Usage

> **Note**
>
> See `sanctum-solver --help` for more options.

`sanctum-solver` uses JSON files to load map information. A `sanctum-solver` JSON file has the following fields:

| Field  | Type       |
|:-------|:-----------|
| `name` | String     |
| `grid` | `Tile[][]` |

…where a `Tile` is:

| Tile       | Description                                                                              |
|:-----------|:-----------------------------------------------------------------------------------------|
| `"Block"`  | A tile which used to be `"Empty"`, but is now obstructed.                                |
| `"Core"`   | An object to protect. The "longest path" is created with respect to these tiles.         |
| `"Empty"`  | A tile which can be moved through freely, but can have `Block`s placed or removed there. |
| `"Impass"` | A tile which cannot be moved through, nor have its obstruction cleared.                  |
| `"Pass"`   | A tile which can be moved through freely, but no `Block`s may be placed there.           |
| `"Spawn"`  | A tile which entities that the `Core` needs protecting from enter into the map.          |

[Here](./park.json) is an example of the map [Park](https://sanctum.fandom.com/wiki/Park).

## Limitations

The output is not guaranteed to be *the* longest path, as some shortcuts have been taken in order to prioritize speed. However, the output is guaranteed to be *a* long path which is fairly efficient given the parameters.
