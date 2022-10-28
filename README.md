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
