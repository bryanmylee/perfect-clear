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
