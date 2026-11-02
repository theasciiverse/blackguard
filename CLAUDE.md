# Blackguard (Asciiverse packaging)

Elnur Badalov's Blackguard, Zach Gage and Kurt Bieg's solo card game
Scoundrel in Rust, Ratatui and tachyonfx, MIT; catalog id `blackguard`.
Not our code: keep changes small, list every one in PATCHES.md, and offer
fixes upstream.

- Remotes: `upstream` is github.com/ElnurBDa/blackguard (branch `main`),
  `origin` is github.com/theasciiverse/blackguard.
- Branch `asciiverse` = upstream main plus our commits.
- Release: tag `v<version>-av.<n>` (now `v0.1.1-av.1`) and push only that
  tag; the workflow runs on `*-av.*` tags and also makes .deb, .rpm, .apk
  and Arch packages (`asciiverse/nfpm.yaml`).
- Run it under a memory cap when testing:
  `systemd-run --user --scope -p MemoryMax=4G ...`.
- Session log: `RUST_LOG=debug blackguard --seed 7 2>blackguard.log`; the
  snapshots show the room, weapon and its kills, so a game can be planned
  move by move from them.
- Static build, in a copy of the tree: `docker run --rm --cpus 4 --memory
  4g -v "$PWD":/src -w /src rust:1-alpine sh -c 'apk add musl-dev && cargo
  build --release --locked -j4'`.
- Preview: from a folder holding the binary, `hub/tools/castrec.py
  asciiverse/blackguard.cast <folder> "w2.5;kright;w0.5;kright;w0.7;kenter;w1.3;kf;w1.4;kleft;w0.6;kenter;w1.8;kright;w0.6;kenter;w1.2;kleft;w0.6;kf;w1.2;kright;w0.6;kenter;w2;ka;w2;kright;w0.5;kright;w0.7;kf;w1.2;kf;w1.2;kleft;w0.6;kf;w2;kb;w1.6;kright;w0.6;kenter;w1.4;kf;w2.5;kq;w0.5"
  -- ./blackguard --seed 7`. Left wraps round the room, so one Left from
  the second card selects the first.
- 80x24 is the smallest size where everything fits.
