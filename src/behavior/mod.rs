//! Provides support for behaviors as defined in the DOM standards. Because the standards
//! refer extensively to classes and mixins, and because Rust does not support either one,
//! this module provides several structures that provide the same behavior but in a Rust-
//! friendly way (using composition instead of inheritance).
//!
//! Every behaviour is implemented using traits, which can be dependent on one another.
//! Every behaviour has a **BehaviourName**Behaviour and **BehaviourName**BehaviourStorage and
//! implementing macro.

pub mod node;
pub mod sandbox_member;
