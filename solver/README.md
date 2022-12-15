# Tetris Real-time Perfect Clear Solver Algorithm

The goal of this solver is to help players develop their perfect clear vision and consistently achieve perfect clears in any Tetris game.

## Implementation

The solver is modelled as a state machine with four types of state: inactive piece states, active piece states, final states, and decision states.

### Inactive states

Inactive piece states describe states where actions do not move any pieces but instead determine the solver's next strategy e.g.

- consume the next piece in queue
- guess the next piece in queue
- continue solving

An inactive state is reduced to an active state when an active piece exists and the "continue solving" action is taken.

### Active states

Active piece states describe states where a piece is being moved into position for a new board state e.g.

- move / rotate / hold active piece

An active state produces another active state when reduced with a move action. Multiple sequences of actions may produce the same state. Therefore, only the most optimal sequence of actions should be saved to reach a specific position and orientation. To accomplish this, each potential piece position and orientation in the dynamic table will save an optional value containing the previous position and orientation, and the reducing action to reach the position and orientation.

- We cannot store the accumulated number of actions up to this point because it would break dynamic updating of the table.

An active state is reduced to its next state when the "place" action is taken, upon which any filled lines will be cleared and the solver will check for final states. If no valid final states exist for the branch, the active state will be reduced to an inactive state again.

### Final states

Final states are the states accumulated and compared during the decision phase. Final states include either:

- perfect clear achieved
- perfect clear failed

### Decision states

When solving, the game will be simulated to create multiple branches of possibilities. Once all decision branches have terminated in a final state, the most optimal perfect clear state will be selected from the collection of final states.

The full state machine can be summarized as:

![state_diagram](https://user-images.githubusercontent.com/42545742/201823624-2f66efe7-a096-44f6-90ad-1943b677a92a.png)

## State

At any given time, state comprises:

- board fill state
  - 24 x 10 cells represented by 4 64-bit bitfields
- piece state
  - position, piece kind, and orientation
  - position is determined with the bottom-left corner of the bounding box
- hold piece kind
- is hold used
- queue piece kinds
- last 14 seen piece kinds
- number of pieces placed since PC
- current probability of branch
  - set by multiplying the previous probability of branch with the probability of the current active piece kind

### Derived state

Several statistics can be derived from state:

- probability of seeing each piece kind next
  - dependent on bag type and last 14 seen piece kinds

## Actions

### Move actions

The move actions available depend on the solver's configuration.

- rotate left / right / 180 (check kick table for new position)
- move left / right
- slow drop (move down one line)
- instant drop (move to lowest point)

Certain combination of moves will result in the same state, therefore this must be memoized.

## Configuration

Different implementations of Tetris have slightly different rulesets which affect the solver's behavior.

Some possible configuration options include:

- next piece generation: random, 7-bag, 14-bag
- kick table: SRS, SRS+, etc. This can be matched to specific Tetris games.
- is slow drop allowed

The solver should reset its cache whenever these options change.

# Implementation notes

## Rotation

For simplicity, we'll start with SRS as the base implementation.

Pieces can either be in a 4x4 or 3x3 bounding box. Rotations are then based on symmetrically rotating the bounding box with its contents. This solves all issues with "half"-point piece centers.
