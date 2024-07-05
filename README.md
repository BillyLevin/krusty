# Krusty

chess engine written in rust

## todos

- [x] add visual feedback for perft tests
- [x] zobrist hashing
- [x] speed up perft (transposition table)
- [x] set up basic cli
- [x] add `help` command
- [x] add debugging commands (load FEN, perft on specific FEN, print board)
- [x] add command to make moves
- [ ] consistent error handling (and panic where it's a bug in program implementation)
- [x] implement UCI protocol (at least the important parts)
- [x] basic search
- [x] basic material evaluation
- [x] time management
- [x] iterative deepening
- [x] integrate transposition table with search
- [x] alpha-beta pruning
- [x] move ordering
- [x] piece-square tables
- [x] quiescence search
- [x] piece mobility
- [ ] opening book
- [x] principal variation search
- [x] make transposition table work with alpha-beta
- [ ] research and experiment with fail-soft alpha-beta
- [x] endgame piece-square tables
- [x] try out tapered evaluation
- [ ] tuning
- [x] check extension
- [x] draw detection (repetitions and 50-move rule)
- [x] null move pruning
- [x] killer move heuristic
- [x] history heuristic
- [ ] aspiration window
- [ ] weighted mobility based on piece type and game phase
