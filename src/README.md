# Tetris Real-time Perfect Clear Solver Algorithm

The goal of this solver is to help players develop their perfect clear vision and consistently achieve perfect clears in any Tetris game.

## Implementation

The solver is modelled as a state machine with four stages: inactive piece stage, active piece stage, final stage, and decision stage.

### Inactive stage

The inactive piece stage describe states where actions do not move any pieces but instead determine the solver's next strategy e.g.

- consume the next piece in queue
- guess the next piece in queue
- continue solving

An inactive stage is reduced to an active stage when an active piece exists and the "continue solving" action is taken.

### Active stage

The active piece stage describe states where a piece is being moved into position for a new board state e.g.

- move / rotate / hold active piece

An active stage produces another active stage when reduced with a move action. Multiple sequences of actions may produce the same state. Therefore, only the most optimal sequence of actions should be saved to reach a specific position and orientation. To accomplish this, each potential piece position and orientation in the dynamic table will save an optional value containing the previous position and orientation, and the reducing action to reach the position and orientation.

- We cannot store the accumulated number of actions up to this point because it would break dynamic updating of the table.

An active stage is reduced to its next stage when the "place" action is taken, upon which any filled lines will be cleared and the solver will check for final stages. If no valid final stage exist for the branch, the active stage will be reduced to an inactive stage again.

### Final stage

The final stage are the states accumulated and compared during the decision stage. Final states include either:

- perfect clear achieved
- perfect clear failed

### Decision stage

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

## Active stage memoization

For each active state, given a board and a piece kind, we need to track whether the position and orientation has been visited before. To allow backtracking, we can store the previous state for any given state and trace back through the table if necessary.

Since the board and piece kind is constant during a given active stage, the only key required for memoization is the position and orientation of the piece.

## Solver strategy

Currently, the solver explores the entire problem space given the constraints and keeps track of the progress so far as `in_progress` to allow for tracing of the actions taken. However, this approach is running into performance issues. Memoization is also difficult because each state is sufficiently complex that a simple memoization strategy is not effective.

### Solver benchmarks

Testing with uniform distribution of piece probabilities.

| Strategy                                                           |           Time to first result |
| ------------------------------------------------------------------ | -----------------------------: |
| `Vec<State>` to store in progress paths                            |                           389s |
| `[Option<State>; 10]` to store in progress paths with `Copy` trait |     DNF (out-of-bounds access) |
| Graph implemented with `HashSet`                                   |    DNF (invalid memory access) |
| `petgraph::graph::Graph` track probabilities and boards            |                 258s, 45s, DNF |
| `petgraph::graph::Graph` deduplicate boards                        | ~420s (4 line), ~660s (2 line) |

It takes roughly 5 minutes to calculate a single perfect clear result which may not even be applicable to the game situation.

### Memoizing probabilities

The optimal position and orientation to place a piece depends on what's available in the queue and what will appear based on the seen pieces in the bag, which can be summarized with the probability of seeing next pieces.

Given a board, an active piece, and a probability distribution for the next piece, we want to find the next board that will provide the highest probability of achieving a perfect clear. A board with 2 or 4 fully filled lines will have 100% probability, and each board before will have decreasing probabilities based on the probability of seeing that piece.

Ultimately, this approach does not seem to be possible as the input domain is far too large when accounting for all probabilities and queues.

### Graph-first approach

Instead of memoizing probabilities, we should save board fills as nodes in a directional graph with piece kinds as edges between nodes. The graph can be lazily built by exploring the piece kinds in the queue or probabilistically.

- For implementation, we can store a forward reference table as `HashMap<(Board, PieceKind), HashSet<Board>>`.
- Note that the memoization table only tracks the graph of board fills against piece kind edges without any probabilities.

However, there isn't much use for that memoization table

### Two-pass approach

We first fill in a backtracking table with transition probabilities on a forward pass, then traverse backwards from the perfect clear board to the start.

Given a probability of each piece kind, applying the piece kind to a blank board will produce a list of possible boards with a transition probability attached i.e. `Vec<(Board, f32)>`. At each expansion step, we repeat the process for each existing board and flatten the results.

Each expansion step is stored in a backtracking table where for a given board and piece, we store the previous board with the highest probability i.e. `HashMap<(Board, PieceKind), (Board, f32)>`.

- The transition probability backwards is equal to the transition probability forwards.
- This method is mathematically flawed, as a board with high transition probability early on could greedily lead the backwards pass through a suboptimal path.
- The expansion step will also require an exhaustive search which defeats the purpose.

It may be beneficial to filter out boards below a threshold probability or the top probable boards. However

- It may be unfeasible to filter for only the top probable boards as there would be no deterministic way to resolve ties.
- Filtering for a threshold may be difficult as the probabilities will shrink exponentially at each expansion stage.
