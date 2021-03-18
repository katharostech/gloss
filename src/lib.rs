//! A thin layer over OpenGL that makes it just a little nicer.
//!
//! Gloss was made to help make it a little easier to do common OpenGL tasks while hopefully redcing
//! how easy it is to miss something trivial that will result in doing something you did not mean
//! to.
//!
//! The intent is not to provide a `safe` Rust API, but to make it simpler to use the rather error
//! prone OpenGL API.
//!
//! This crate is built on [`glow`] and can therefore be used on OpenGL, OpenGL ES, and WebGL.
//!
//! [`glow`]: https://github.com/grovesNL/glow
