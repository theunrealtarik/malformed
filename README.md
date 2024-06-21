# Malformed

![malformed banner](https://github.com/theunrealtarik/malformed/assets/58333332/2f277cf7-32a9-4263-b81b-e0302dc5fd8d)

> [!WARNING]
> This game is a piece of software that will entirely crash your computer whenever you lose which can lead to hardware-level damage and loss of any progress of any ongoing work.

> [!WARNING]
> I'm not responsible for any mis use of the provided code in this repository.

## Download
If you just want to try what the game is all about (nothing new really) please download the **regular** version as it doesn't crash your whole system, instead your player will respawn and restart the level, unlike the **BSoD** version.
> **BSoD**: *"blue screen of death"*


## Controls
- `Space` to jump, the longer you hold the longer your jump gets.
- `R` to restart the level after death. 

## Development
For starters read [bevy](https://bevyengine.org/learn/quick-start/getting-started/setup/)'s getting started guide and make sure you have [Rust](https://rustup.rs/) installed on your system then install [cargo-make](https://github.com/sagiegurari/cargo-make). (idk why am explaining this LOL)
```bash
// debug mode with dynamic linking
cargo make build-debug
cargo make run

// release mode with static linking
cargo make --profile production build-release
```
Regarding contributions, yes (please).