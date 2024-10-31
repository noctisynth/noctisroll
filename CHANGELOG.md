# Changelog

## \[0.1.5]

### Refactors

- [`9a8bf69`](https://github.com/noctisynth/noctisroll.git/commit/9a8bf69efb2269beed019ce00a0d63b88d46dbfd) ([#12](https://github.com/noctisynth/noctisroll.git/pull/12) by [@fu050409](https://github.com/noctisynth/noctisroll.git/../../fu050409)) Refactor `Dice` struct to use `u32` instead of `i32` for the `sides` field and it's result.
- [`82efd32`](https://github.com/noctisynth/noctisroll.git/commit/82efd32941379f934fb49b48b2b3872c9023a228) ([#10](https://github.com/noctisynth/noctisroll.git/pull/10) by [@fu050409](https://github.com/noctisynth/noctisroll.git/../../fu050409)) Refactored the grammar to make it more scalable, universal, and easier to maintain.

## \[0.1.4]

### Bug Fixes

- [`8995cd5`](https://github.com/noctisynth/noctisroll.git/commit/8995cd59cbd9e702818346cd0965830dcfe82a65) ([#8](https://github.com/noctisynth/noctisroll.git/pull/8) by [@fu050409](https://github.com/noctisynth/noctisroll.git/../../fu050409)) Fix `'static` lifetime issue in roll functions.

## \[0.1.3]

### New Features

- [`8135065`](https://github.com/noctisynth/noctisroll.git/commit/8135065de3ca01cc14d72e5343511149db04e90f) ([#6](https://github.com/noctisynth/noctisroll.git/pull/6) by [@fu050409](https://github.com/noctisynth/noctisroll.git/../../fu050409)) Support dice filter in grammar parsing.

  For example, `6d6k3` will roll 6 six-sided dice and keep the highest 3 results and `6d6q3` will roll 6 six-sided dice and keep the lowest 3 results.

## \[0.1.2]

### New Features

- [`3262824`](https://github.com/noctisynth/noctisroll.git/commit/3262824e5912240819638e2f3640f6e51c19ab7f) ([#3](https://github.com/noctisynth/noctisroll.git/pull/3) by [@fu050409](https://github.com/noctisynth/noctisroll.git/../../fu050409)) Support for some math functions in roll string.

  For example, `max(1, 2)` will return `2`.
- [`3262824`](https://github.com/noctisynth/noctisroll.git/commit/3262824e5912240819638e2f3640f6e51c19ab7f) ([#3](https://github.com/noctisynth/noctisroll.git/pull/3) by [@fu050409](https://github.com/noctisynth/noctisroll.git/../../fu050409)) Support for procedural roll string.
- [`3262824`](https://github.com/noctisynth/noctisroll.git/commit/3262824e5912240819638e2f3640f6e51c19ab7f) ([#3](https://github.com/noctisynth/noctisroll.git/pull/3) by [@fu050409](https://github.com/noctisynth/noctisroll.git/../../fu050409)) Support for `+|-` prefix in roll string parsing.

## \[0.1.1]

### New Features

- [`e039a38`](https://github.com/noctisynth/noctisroll.git/commit/e039a38e1c06909ecb0b5eee66f80c0ca53e74ee) ([#1](https://github.com/noctisynth/noctisroll.git/pull/1) by [@fu050409](https://github.com/noctisynth/noctisroll.git/../../fu050409)) Support for procedural roll string.
