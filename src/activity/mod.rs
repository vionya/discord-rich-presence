//! Provides an interface for building activities to send
//! to Discord via [`DiscordIpc::set_activity`](crate::DiscordIpc::set_activity).
//! 
//! This interface is primarily accessed via the structs provded by the [`builders`]
//! submodule.
//! 
//! A typical usage might look like:
//! ```
//! // Boilerplate elided
//! let mut activity = activity::ActivityBuilder::default()
//!     .state("State")
//!     .details("Details")
//!     .assets(activity::AssetsBuilder::default()
//!         .large_image("large-image")
//!         .small_image("small-image")
//!         .build()
//!     )
//!     .build();
//! 
//! client.set_activity(activity)?;
//! ```
//! 
//! Some structs in the [`models`] also provide shorthand `new()` functions to construct
//! them more concisely, such as [`models::Timestamps`].
pub mod builders;
pub mod models;

#[doc(inline)]
pub use builders::*;
pub use models::*;