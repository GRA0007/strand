# Changelog

## \[0.5.0]

### New Features

- [`461d2cc`](https://www.github.com/GRA0007/strand/commit/461d2cc8948d14717908c0223bd1fd7ff2febe15) Add file diff viewer

### Bug Fixes

- [`74a9ffc`](https://www.github.com/GRA0007/strand/commit/74a9ffc6ddc106dba5f655aab3d03626279d54ea) Limit initials of a user avatar to 2 and correctly parse emojis and other unicode characters
- [`8917470`](https://www.github.com/GRA0007/strand/commit/8917470fb63ad83d6710f9fa462cb222fa119972) Show toasts when an error occurs in a query or mutation

## \[0.4.0]

### New Features

- [`7bcdbac`](https://www.github.com/GRA0007/strand/commit/7bcdbaca2a34a8e8fc82e7f108248763cadf2e04) Display commit info and changed files when a commit is selected

### Bug Fixes

- [`28412fb`](https://www.github.com/GRA0007/strand/commit/28412fbb76e115231bef778a8c6c7f80e6ae9cb4) Fix commit parsing if description contained newlines
- [`968bab9`](https://www.github.com/GRA0007/strand/commit/968bab9cfa776d86423a08a327f8c0030f4a4cf0) Only fetch requested git command log items from the database
- [`97496b8`](https://www.github.com/GRA0007/strand/commit/97496b88ad48200c8a80f9bfe669f152187c9fd8) Refetch the graph when opening a new repository, fetching, or when tabbing back onto Strand

## \[0.3.0]

### New Features

- [`d2bb9de`](https://www.github.com/GRA0007/strand/commit/d2bb9dea5c38d3c761e34ec28fbfbd7e93aef958) Load commits with `git log` and display them in the graph panel

## \[0.2.0]

### New Features

- [`bfbf0b3`](https://www.github.com/GRA0007/strand/commit/bfbf0b3a4dcb4236bb9cb00944dfb3db41add308) Disable unimplemented buttons in the UI
- [`bfbf0b3`](https://www.github.com/GRA0007/strand/commit/bfbf0b3a4dcb4236bb9cb00944dfb3db41add308) Store the full git command log history and make it viewable in the UI
- [`bfbf0b3`](https://www.github.com/GRA0007/strand/commit/bfbf0b3a4dcb4236bb9cb00944dfb3db41add308) Implement a basic toast system to show alerts and information

## \[0.1.0]

### New Features

- [`754d755`](https://www.github.com/GRA0007/strand/commit/754d755faeea008334ba1215e748e097ab8359d3) Run `git fetch --all` with the fetch button
- [`754d755`](https://www.github.com/GRA0007/strand/commit/754d755faeea008334ba1215e748e097ab8359d3) Load local and remote branches into the sidebar
- [`754d755`](https://www.github.com/GRA0007/strand/commit/754d755faeea008334ba1215e748e097ab8359d3) Open a local git repository with the system folder picker
