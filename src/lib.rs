#![allow(clippy::new_without_default, clippy::needless_pass_by_value)]

#[macro_use]
extern crate tokio_trace;
#[macro_use]
extern crate prost_derive;
#[macro_use]
extern crate derivative;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

pub mod buffers;
pub mod event;
pub mod metrics;
pub mod region;
pub mod sinks;
pub mod sources;
pub mod test_util;
pub mod topology;
pub mod transforms;
pub mod types;

pub use event::Event;
