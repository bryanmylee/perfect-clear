# Tetris Perfect Clear Solver Algorithm

We model the game as a state machine with discrete states and actions.

## State

At any given time, state comprises:

- board fill state
  - 24 x 10 cells represented by 4 64-bit bitfields
- piece state
  - center point, piece variant, and rotation
- hold piece variant
- is hold used
- queue pieces
- seen pieces
- probability of seeing each piece next
  - dependent on bag type and seen pieces
- probability of PC
  - given any path to PC, the probability of PC is equal to the probability of seeing the pieces required to traverse that path
  - the total probability of PC is the sum of probabilities of paths to PC
- probability of next PC
  - traverse the path further until the next PC

## Actions

When applied to a given state, actions produce a new state. There actions available are:

- rotate left (check kick table for new position)
- rotate right (check kick table for new position)
- move left
- move right
- soft drop (move down one line)
- hard drop
- use hold
