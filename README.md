# tetr::

A ✨__**modern**__✨ Tetris game made in *OpenGL* and *Rust*

## Gameplay

**TODO**: add gameplay gif when the game looks somewhat good

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

## Playing the game

To build and launch the game, make sure you have *the Rust programming language* installed and type:

```sh
cargo run -r
```

To compile the game only, use `cargo build -r`. The game should be contained in a single portable executable, located in `target/release/`

### Keybinds

The single-player keybinds conform to the [Tetris guideline](https://tetris.wiki/Tetris_Guideline).

The two-player keybinds are as follows:

#### Left player

 * WASD - rotate CW, soft drop, left, right
 * LShift - swap
 * LCtrl - rotate CCW
 * Space - hard drop

#### Right player

 * Up, Down, Left, Right - rotate CW, soft drop, left, right
 * RShift - swap
 * RCtrl - rotate CCW
 * Return - hard drop
