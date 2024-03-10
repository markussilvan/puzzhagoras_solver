# Puzzhagoras Solver

A brute force DFS solver for the Puzzhagoras: Pythagoras Challenge
jigzaw puzzle.

## Potential Optimizations

  - instead of moving pieces around have "two jumplists"
    (pieces have position and board position has index to piece list)
    - this also fixes "the removing problem"
  - when going through connector, if connector matches, but offset is
    different, then flipping would make it fit (on that side)
  - make edges first (this might make a big difference)
  - use VecDeque instead of Vec for connectors
