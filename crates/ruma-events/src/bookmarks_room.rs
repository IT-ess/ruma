//! Types for the [`it.refs.msc4482.bookmarks_room`] account data event.
//!
//! [`it.refs.msc4482.bookmarks_room`]: https://github.com/matrix-org/matrix-spec-proposals/pull/4482

use ruma_common::OwnedRoomId;
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

/// The content of an [`it.refs.msc4482.bookmarks_room`] event.
///
/// [`it.refs.msc4482.bookmarks_room`]: https://github.com/matrix-org/matrix-spec-proposals/pull/4482
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
#[ruma_event(type = "it.refs.msc4482.bookmarks_room", kind = GlobalAccountData)]
pub struct BookmarksRoomEventContent {
    /// The current active bookmarks room
    pub active_bookmarks_room_id: OwnedRoomId,
}

impl BookmarksRoomEventContent {
    /// Creates a new `BookmarksRoomEventContent`.
    pub fn new(active_bookmarks_room_id: OwnedRoomId) -> Self {
        Self { active_bookmarks_room_id }
    }
}
