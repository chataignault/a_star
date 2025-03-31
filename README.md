# A* algorithm

A*-like algorithms with a focus on parameter-tuning.

Motivation :
- Use Python bindings with Rust with build-system [Maturin](https://github.com/PyO3/maturin)
- Environment configuration, library packaging with [uv](https://docs.astral.sh/uv/) 


Build and install in python environment :
```shell
maturin develop --uv --bindings pyo3
```

TODO :
- [x] implement A* from [pseudo-code](https://en.wikipedia.org/wiki/A*_search_algorithm)
- [x] project structure as detailed in [maturin guide](https://www.maturin.rs/project_layout.html)
- [x] example python binding from end to end and verify build
- [ ] adapt Python parameter to be a n-tuple
- [ ] identify practical use-cases : optimisation grid (the goal is some heuristic on the loss as opposed to a location on the grid)
- [ ] detail the example in Python
- [ ] add CI build check
