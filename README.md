# tetr::

A ✨*modern*✨ Tetris game made in *OpenGL* and *Rust*

## Gameplay

![](gameplay.gif)

`tetr::` is an implementaion of **modern** Tetris, and essentially a clone of [tetr.io](https://tetr.io). This means:

 * the game provides a [hold slot](https://tetris.wiki/Hold_piece)
 * the game previews 5 next pieces and uses a [7-bag generator](https://tetris.wiki/Random_Generator) for improved gae stability
 * the games uses the [super rotation system](https://tetris.wiki/Super_Rotation_System) allowing for some cool tricks
 * you should use [hard drop](https://tetris.wiki/Drop#Hard_drop) for eveything
 * there is a [ghost piece](https://tetris.wiki/Ghost_piece) previewing where the block will fall

All these features together make the game almost impossible to lose in single-
player mode. The fun is in *multiplayer mode*, which is incredibly good on
tetr.io, except that tetr.io doesn't provide the classic split-screen game.

This game doesn't provide online multiplayer *yet*, however, it is on the list
of features to be added once the game is playable enough.

## Non-gameplay

The game provides the following features (this is a TODO list):

 * [x] Single-player mode
 * [x] Split screen mode for 2 players
 * [ ] Split screen mode for `n` players
 * [ ] Settings screen
 * [ ] Online multiplayer

## Why another tetris game?

 * [tetr.io](https://tetr.io) has ads
 * tetr.io lacks split-screen multiplayer
 * tetr.io is a web game, which impacts the performance and user experience
 * I wanted to have a take at implementing Tetris, as there is a surprising amount of technical challenges involved

## Playing the game

To build and launch the game, make sure you have [the Rust programming language](https://www.rust-lang.org/) installed and type:

```sh
cargo run -r
```

To compile the game only, use `cargo build -r`. The game should be contained in a single executable, located in `target/release/`

### Backgrounds

The game looks for a `backgrounds` folder in its working directory, so make
sure that you are in the root directory of the game (where this README file is
located), or you have the backgrounds directory copied to your CWD. **TODO**:
add option to install the directory somewhere else.

All of the backgrounds are taken from [DT's wallpaper collection](https://gitlab.com/dwt1/wallpapers), which uses pictures from [unsplash](https://unsplash.com/).

### Keybinds

The single-player keybinds conform to the [Tetris guideline](https://tetris.wiki/Tetris_Guideline).

The two-player keybinds are as follows:

#### Left player

 * A - Left
 * D - Right
 * W - RotateCW
 * LCtrl - RotateCCW
 * S - SoftDrop
 * Space - HardDrop
 * LShift - Swap

#### Right player

 * Numpad1 - Left
 * Numpad3 - Right
 * Numpad5 - RotateCW
 * NumpadComma - RotateCCW
 * Numpad2 - SoftDrop
 * Numpad0 - HardDrop
 * NumpadEnter - Swap
