# hactar
__hactar__ is a simple chessengine written in rust.

## todo
### perft
- [x] in/out functions
- [x] zobrist hashing
- [x] move generator
- [ ] move generator performance improvements
- [x] make-move
- [x] perft

### uci
- [x] basic uci framwork
- [x] mutlithreading for all time input
- [ ] setoptions for different search types

### alpha-beta
- [x] move sorting
- [x] quiescence search
- [ ] iterativ deepening
- [ ] transposition table
- [ ] performance improvements
- [ ] further pruning and extensions
- [ ] performance improvements

### monte-carlo
- [ ] monte-carlo tree search with alpha-beta search instead of rollout
- [ ] monte-carlo tree search as evaluation at leaf nodes of highly parallel min-max search:
  - [ ] on CPU
  - [ ] on GPU

### machine learning / reinforcement learning
_???_

## license
Copyright 2018 Tsoj Tsoj

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
