<img src="./public/logo.png" width="100">

# Strand

[![GitHub Release](https://img.shields.io/github/v/release/GRA0007/strand?label=Version)](https://github.com/GRA0007/strand/releases)
[![checks](https://img.shields.io/github/check-runs/GRA0007/strand/main?label=Checks)](https://github.com/GRA0007/strand/actions)
[![Tauri v2](https://img.shields.io/badge/Tauri-v2-FFC131?logo=tauri&logoColor=FFC131)](https://tauri.app/)

> A strand-like git GUI

Strand intends to be a simple but powerful graphical user interface for [Git](https://www.git-scm.com/).

## Principles

- Strand is designed to use only native git cli commands to interface with your repositories. This has the benefit of being able to support every git feature, as well as show a log of commands it has run.
- As a git GUI, Strand attempts to simplify complex git functionality, with the aid of the visuals. This includes functionality like auto-fetch, undo/redo, and the git graph.
- During early development, Strand is very opinionated, and minimal customisation will be available. This may change in the future.

## Download

> [!WARNING]
> Strand is currently in alpha, so many features will be missing/broken and there is no documentation. Feel free to try it out at your own risk.

You can download the latest release for your device from the [GitHub releases](https://github.com/GRA0007/strand/releases).

## Contributing

Strand is still in the early stages of design and development, so any contributions are likely to be rejected. If you'd like to get it running locally however, follow the steps below.

### Local Development

1. You'll need [Rust](https://www.rust-lang.org/) and [Node.js](https://nodejs.org/en) ([fnm](https://github.com/Schniz/fnm) or similar recommended) installed to begin. Make sure you install the version of Node specified in the `.nvmrc` file.
2. Install [pnpm](https://pnpm.io/installation).
3. Clone the repository locally with git.
4. Run `pnpm i` in the root of the repository to install the JavaScript dependencies.
5. Run `pnpm tauri dev` to start the dev server.

The UI is built with [React](https://react.dev/), and the code can be found in the `src` folder.

The `src-tauri` folder contains the Rust code. If you want to take advantage of the [`sqlx`](https://github.com/launchbadge/sqlx) checked queries, you'll need to create a `.env` file in this folder with the `DATABASE_URL` set to the location of the Sqlite file, e.g. `sqlite:/Users/you/Library/Application Support/dev.bengrant.strand/data.db`.
