# Gloss

[![Crates.io](https://img.shields.io/crates/v/gloss.svg)](https://crates.io/crates/gloss)
[![Docs.rs](https://docs.rs/gloss/badge.svg)](https://docs.rs/gloss)
[![Katharos License](https://img.shields.io/badge/License-Katharos-blue)](https://github.com/katharostech/katharos-license)

A thin layer over OpenGL that makes it just a little nicer.

Gloss was made to help make it a little easier to do common OpenGL tasks while hopefully redcing
how easy it is to miss something trivial that will result in doing something you did not mean
to.

The intent is not to provide a `safe` Rust API, but to make it simpler to use the rather error
prone OpenGL API.

This crate is built on [`glow`] and can therefore be used on OpenGL, OpenGL ES, and WebGL.

[`glow`]: https://github.com/grovesNL/glow
