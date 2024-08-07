//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{InputSticker, True, UserId};

impl_payload! {
    @[multipart = sticker]
    /// Use this method to add a new sticker to a set created by the bot. Emoji sticker sets can have up to 200 stickers. Other sticker sets can have up to 120 stickers. Returns True on success.
    #[derive(Debug, Clone, Serialize)]
    pub AddStickerToSet (AddStickerToSetSetters) => True {
        required {
            /// User identifier of sticker file owner
            pub user_id: UserId,
            /// Sticker set name
            pub name: String [into],
            /// A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set isn't changed.
            pub sticker: InputSticker,
        }
    }
}
