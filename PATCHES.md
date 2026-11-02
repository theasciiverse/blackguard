# Changes from upstream

This is the Asciiverse packaging of [Blackguard](https://github.com/ElnurBDa/blackguard)
by Elnur Badalov: Zach Gage and Kurt Bieg's solo card game Scoundrel in the
terminal (Rust, Ratatui, tachyonfx). Upstream is the `upstream` remote; the
`asciiverse` branch carries the changes below.

Built from upstream `main` at `294bfd3` (2026-08-01), tagged v0.1.1 there.

## Licence

MIT, "Copyright (c) 2026 Elnur Badalov" in LICENSE, and `license = "MIT"`
in Cargo.toml (checked 2026-11-02).

## Code

- **Session logging.** `RUST_LOG=debug blackguard 2>blackguard.log`
  records every key as it arrives, before it is handled, each deal (a new
  seed, or the same one again with r or t), a win or death, the help
  opening and closing, and a snapshot of the table (health, room, weapon
  and its kills, deck, discard, counts) at most once a second while it
  changes. `src/logging.rs`, the `log` and `env_logger` crates; off unless
  `RUST_LOG` is set, so nothing is ever written over the game.
- **Ctrl+C quits like q.** In raw mode Ctrl+C arrives as a key, and the
  game ignored it.

## Packaging

- Upstream's workflows (`ci.yml`, checks on every push, and `release.yml`,
  its own releases and a crates.io publish on `v*` tags) are removed.
- `.github/workflows/release.yml`: av archives `blackguard-<target>` for
  Linux (static musl, x86-64 and ARM64, built in Alpine), macOS (x86-64
  and ARM64) and Windows (x86-64 and ARM64, MSVC), with the binary,
  LICENSE, README and this file, plus SHA256SUMS; and .deb, .rpm, .apk and
  Arch packages of the static Linux binary, made by nfpm
  (`asciiverse/nfpm.yaml`), with no menu entry. Runs on our tags only
  (`*-av.*`).
- `asciiverse/blackguard.cast`, the catalog preview, recorded at 100x30
  with hub's `tools/castrec.py`: seed 7 played for five rooms (a weapon
  taken and swapped, the blade on 9♠, a potion, a room avoided, the blade
  dulling on 7♠, 5♠ and 4♣, the Q♠ fought bare-handed and a heal after).

av starts it as `blackguard` with no arguments: a random dungeon.

## Checked

- Linux, static musl build (Alpine, rustc 1.99.0): the 35 upstream tests
  pass, and `cargo clippy --all-targets -- -D warnings` is clean. In tmux,
  q, Esc and Ctrl+C quit with status 0; resized between 1x1 and 150x50
  without a crash, at 4 MB; 80x24 is the smallest size where everything
  fits. The four system packages build with nfpm 2.43.0.
- Windows and macOS: `cargo check` for x86_64-pc-windows-msvc and
  aarch64-apple-darwin passes; not run.

## Ours

- **How to play it, at the top of README.md.** A short note on installing
  and starting it with av; the original text follows unchanged.
