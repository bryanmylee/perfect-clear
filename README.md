# Tetris Perfect Clear Solver

An experiment to use WebAssembly for high performance in the solver algorithm and Web Workers for concurrency.

## UX

1. The user is presented with an empty board with empty placeholders in the queue and on top of the board.
2. The user clicks an empty placeholder to open a menu and select pieces (the menu could be arranged above or below the placeholder in an arc).
3. Once the queue is filled, the user presses start to begin solving the perfect clear.
4. Pieces will move on the screen with feedback on which actions are taken (move, rotate, etc).
5. When the solver reaches a point where it needs more input from the user (e.g. the next piece in the queue), it will pause execution.
6. The additional input needed will be highlighted or opened automatically.
7. If there are no valid solves, a message will appear at the bottom of the board indicating as such.
8. Otherwise, the pieces will move until the perfect clear is achieved.
9. The user can step back through the actions taken via a timeline below the board.

## Implementation

On starting the solving process, a serialized message can be sent from the UI to the solver worker describing the starting conditions.

Once given the starting conditions, the solver will find the next states possible given a set of possible actions and determine the possible paths towards a perfect clear. These paths will have a probability attached based on the probability of the next pieces which have yet to be seen.

- The worker should retain a cache of states to valid paths for performance.

The solver will return the path with the highest probability of reaching a perfect clear; this may be a single action or a series of actions. It will also return the next state given those actions, which will include blanks in the next queue.

The user will then be responsible for filling in the next pieces before continuing the solving process, upon which another serialized message will be sent from the UI to the solver worker describing the new starting conditions.

## Rust + WebAssembly Template

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
- `LICENSE-APACHE` and `LICENSE-MIT`: most Rust projects are licensed this way, so these are included for you

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
