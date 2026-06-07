//! Types for extensible bookmark events ([MSC4482]).
//!
//! [MSC4482]: https://github.com/matrix-org/matrix-spec-proposals/pull/4482

use ruma_common::{OwnedEventId, OwnedRoomId, OwnedServerName};
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

use crate::message::TextContentBlock;

/// The payload for an extensible bookmark event.
///
/// This is a new kind of event introduced in [MSC4482] and should only be sent in rooms
/// with type `m.bookmarks` (unstable prefix `it.refs.msc4482.bookmarks`).
/// This event contains a new `m.pointer` block that references an event sitting in
/// another room.
/// [MSC4482]: https://github.com/matrix-org/matrix-spec-proposals/pull/4482
#[derive(Clone, Debug, Serialize, Deserialize, EventContent)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
#[ruma_event(type = "it.refs.msc4482.bookmark", kind = MessageLike, alias = "m.bookmark")]
pub struct BookmarkEventContent {
    /// The text representation of the message.
    #[serde(rename = "org.matrix.msc1767.text")]
    pub text: TextContentBlock,
    /// A reference to the bookmarked event
    #[serde(rename = "it.refs.msc4482.pointer", alias = "m.pointer")]
    pub pointer: PointerContentBlock,
}

/// A block for pointing to another event.
/// This reference to the event may not be valid or
/// reachable by the user.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct PointerContentBlock {
    /// The room_id where the event is
    pub room_id: OwnedRoomId,
    /// The event_id of the referenced event
    pub event_id: OwnedEventId,
    /// The server names used to route the request if necessary
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    via: Vec<OwnedServerName>,
}
