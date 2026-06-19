//! Types for extensible bookmark events ([MSC4482]).
//!
//! [MSC4482]: https://github.com/matrix-org/matrix-spec-proposals/pull/4482

use ruma_common::{MatrixUri, OwnedEventId, OwnedRoomId, OwnedServerName, matrix_uri::MatrixId};
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

use crate::{message::TextContentBlock, room::message::Relation};

/// The payload for an extensible bookmark event.
///
/// This is a new kind of event introduced in [MSC4482] and should only be sent in rooms
/// with type `m.bookmarks` (unstable prefix `it.refs.msc4482.bookmarks`).
/// This event contains a new `m.pointer` block that references an event sitting in
/// another room.
/// [MSC4482]: https://github.com/matrix-org/matrix-spec-proposals/pull/4482
#[derive(Clone, Debug, Serialize, Deserialize, EventContent)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
#[ruma_event(type = "it.refs.msc4482.bookmark", kind = MessageLike, without_relation)]
pub struct BookmarkEventContent {
    /// The text representation of the message.
    #[serde(rename = "org.matrix.msc1767.text")]
    pub text: TextContentBlock,

    /// A reference to the bookmarked event
    #[serde(rename = "it.refs.msc4482.pointer", alias = "m.pointer")]
    pub pointer: PointerContentBlock,
    /// Information about related messages.
    #[serde(
        flatten,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::room::message::relation_serde::deserialize_relation"
    )]
    pub relates_to: Option<Relation<BookmarkEventContentWithoutRelation>>,
}

impl BookmarkEventContent {
    /// Create a BookmarkEventContent with the default fallback text block
    pub fn new(
        pointer: PointerContentBlock,
        sender_display_name: &str,
        room_display_name: &str,
    ) -> Self {
        let event_uri: MatrixUri = pointer.clone().into();
        let text = TextContentBlock::html(
            format!(
                "You bookmarked a message sent by {sender_display_name} in {room_display_name}."
            ),
            format!(
                "You bookmarked this <a href=\"{event_uri}\">message</a> sent by {sender_display_name} in {room_display_name}."
            ),
        );
        Self { text, pointer, relates_to: None }
    }

    /// Create a BookmarkEventContent with a custom fallback text block
    pub fn new_with_custom_fallback(pointer: PointerContentBlock, text: TextContentBlock) -> Self {
        Self { text, pointer, relates_to: None }
    }
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
    /// This should always point to the first version of
    /// an event, i.e. in case the event has a `m.relates_to`
    /// relation of type `m.replace`, you should point instead
    /// to the original root event_id. The client then have the
    /// responsibility to resolve the content of the bookmark
    /// to its latest version.
    pub event_id: OwnedEventId,
    /// The server names used to route the request if necessary
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub via: Vec<OwnedServerName>,
}

impl PointerContentBlock {
    /// Create a new [`PointerContentBlock`]
    pub fn new(room_id: OwnedRoomId, event_id: OwnedEventId, via: Vec<OwnedServerName>) -> Self {
        Self { room_id, event_id, via }
    }
}

impl From<PointerContentBlock> for MatrixUri {
    fn from(value: PointerContentBlock) -> Self {
        let id: MatrixId = (value.room_id, value.event_id).into();
        MatrixUri::new(id, value.via, None)
    }
}
