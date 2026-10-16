#![allow(dead_code)]

// Single-cell helpers for SDS regressions. The two-cell harness depends on
// MLE snapshot APIs which are not part of the current base-station runtime.
#[path = "component_test.rs"]
pub mod component_test;
#[path = "default_stack.rs"]
pub mod default_stack;
#[path = "sink.rs"]
pub mod sink;

pub use component_test::ComponentTest;
