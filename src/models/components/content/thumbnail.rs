use twilight_model::channel::message::component::{Thumbnail as TwilightThumbnail, UnfurledMediaItem};

use crate::traits::component::IntoTwilight;

pub struct Thumbnail(TwilightThumbnail);

impl Thumbnail {
    /// Supports arbitrary urls and `attachment://<filename>` references.
    pub fn new(url: impl Into<String>) -> Self {
        Self(TwilightThumbnail {
            id: None,
            media: UnfurledMediaItem {
                url: url.into(),
                proxy_url: None,
                height: None,
                width: None,
                content_type: None
            },
            description: None,
            spoiler: None
        })
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.0.description = Some(Some(description.into()));
        self
    }

    pub fn spoiler(mut self, spoiler: bool) -> Self {
        self.0.spoiler = Some(spoiler);
        self
    }
}

impl IntoTwilight<TwilightThumbnail> for Thumbnail {
    fn into_twilight(self) -> TwilightThumbnail {
        self.0
    }
}