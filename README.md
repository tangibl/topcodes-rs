# TopCodes in Rust

This is a direct reimplementation of TopCodes in Rust. The original source by
Michael Horn can be found [here](https://github.com/TIDAL-Lab/TopCodes).

## Plans

The goal of this package is to be as agnostic of the platform as possible. All
dependencies that are not explicitly required will be feature-gated to ensure
that the default dependencies of this project are as close to zero as possible.
Ideally, this version of the project should be able to run on most/all
platforms that are supported by Rust out of the box.

I plan to create a separate repository for providing a dynamic library from this
source, so that it can be pulled in from other languages, as well.
