//! Provides an interface for building activities to send
//! to Discord via [`DiscordIpc::set_activity`](crate::DiscordIpc::set_activity).
//!
//! This interface is primarily accessed via the structs provded by the [`builders`]
//! submodule.
//!
//! A typical usage might look like:
//! ```no_run
//! // Boilerplate elided
//! let mut activity = activity::ActivityBuilder::default()
//!     .state("State")
//!     .details("Details")
//!     .assets(activity::AssetsBuilder::default()
//!         .large_image("large-image")
//!         .small_image("small-image")
//!         .build()
//!     )
//!     .timestamps(activity::Timestamps::new(Some(0), None))
//!     .build();
//!
//! client.set_activity(activity)?;
//! ```
//!
//! Some structs in [`models`] also provide shorthand `new()` methods to construct
//! them more concisely, such as [`Timestamps`].
//!
//! This module re-exports both [`models`] and [`builders`]. The primary way to create
//! models is via the structs in [`builders`].
pub mod builders;
pub mod models;

#[doc(inline)]
pub use builders::*;
pub use models::*;
