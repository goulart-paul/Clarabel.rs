# Changelog

Changes for the Julia version of Clarabel are documented in this file.   For the Rust version, see [here](https://github.com/oxfordcontrol/clarabel.rs/CHANGELOG.md).

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

Version numbering in this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).  We aim to keep the core solver functionality and minor releases in sync between the Rust/Python and Julia implementations.   Small fixes that affect one implementation only may result in the patch release versions differing.

## [0.4.1] - 2023-08-03

### Changed 

Added optional feature to remove inequality constraints with very large upper bounds.   This feature is enabled by default but can be turned off using the `presolve_enable` setting.  

Bug fix in equilibration for NN and zero cones.
### Rust/Python specific changes

Rust algebra module modified to allow chaining of elementwise vector operations.

Added Rust matrix format checking utility in CscMatrix::check_format.  NB: CscMatrix 
integrity is assumed by the solver and is not checked internall.
## [0.4.0] - 2023-25-02

### Changed 

- Internal fixes relating to initialization of iterates in symmetric cone problems.

- Numerical stability improvements for second order cone constraints. 

### Rust/Python specific changes

- Modification of the internal calls to the Rust qdldl to allow for direct assignment of parameters in AMD ordering.   

- Added release of binaries for arm64 Linux [#9](https://github.com/oxfordcontrol/Clarabel.rs/issues/9).   Thanks to @nrontsis.

- Fixed a bug using `==` for SolverStatus objects in Python.  Fixes [#10](https://github.com/oxfordcontrol/Clarabel.rs/issues/10).

- Python now reports the Clarabel version using `__version__` instead of `__version__()`.

- Added additional unit tests for Rust implementation.   NB: Rust implementation is also tested
offline against the Julia-based benchmark problem suite, but this will not appear in coverage reporting.  


## [0.3.0] - 2022-09-13

### Changed 

- Implements support for exponential and power cones

- Numerical stability improvements

- Various bug fixes

## [0.2.0] - 2022-07-31

- Rust/python implementation released starting from this version.

- Ported all documentation to the common site [here](https://github.com/oxfordcontrol/ClarabelDocs)


[0.4.1]: https://github.com/oxfordcontrol/Clarabel.rs/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/oxfordcontrol/Clarabel.rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/oxfordcontrol/Clarabel.rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/oxfordcontrol/Clarabel.rs/tree/v0.2.0
