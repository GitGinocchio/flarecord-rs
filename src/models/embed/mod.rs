use twilight_model::{channel::message::{Embed as TwilightEmbed, embed::EmbedField}, util::Timestamp};

use crate::{models::{color::Color, embed::{author::EmbedAuthor, footer::EmbedFooter, image::EmbedImage, provider::EmbedProvider, thumbnail::EmbedThumbnail, video::EmbedVideo}}, traits::component::IntoTwilight};

pub mod thumbnail;
pub mod provider;
pub mod author;
pub mod footer;
pub mod image;
pub mod video;

pub struct Embed(TwilightEmbed);

impl Embed {
    pub fn new() -> Self {
        Self(TwilightEmbed {
            author: None,
            color: None,
            description: None,
            fields: vec![],
            footer: None,
            image: None,
            kind: "rich".into(),
            provider: None,
            thumbnail: None,
            timestamp: None,
            title: None,
            url: None,
            video: None
        })
    }

    pub fn set_author(&mut self, author: Option<EmbedAuthor>) {
        self.0.author = author.map(|a| a.into())
    }

    pub fn set_color(&mut self, color: Option<Color>) {
        self.0.color = color.map(|c| c.into())
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.0.title = title.map(|s| s.into())
    }

    pub fn set_url(&mut self, url: Option<String>) {
        self.0.url = url.map(|s| s.into())
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.0.description = description.map(|s| s.into())
    }

    pub fn set_provider(&mut self, provider: Option<EmbedProvider>) {
        self.0.provider = provider.map(|p| p.into())
    }

    pub fn set_footer(&mut self, footer: Option<EmbedFooter>) {
        self.0.footer = footer.map(|f| f.into());
    }

    pub fn set_thumbnail(&mut self, thumbnail: Option<EmbedThumbnail>) {
        self.0.thumbnail = thumbnail.map(|i| i.into())
    }

    pub fn set_image(&mut self, image: Option<EmbedImage>) {
        self.0.image = image.map(|i| i.into())
    }

    pub fn set_video(&mut self, video: Option<EmbedVideo>) {
        self.0.video = video.map(|i| i.into())
    }

    pub fn set_timestamp(&mut self, timestamp: Option<Timestamp>) {
        self.0.timestamp = timestamp.into();
    }

    pub fn add_field(&mut self, field: EmbedField) {
        self.0.fields.push(field);
    }
}

impl From<TwilightEmbed> for Embed {
    fn from(value: TwilightEmbed) -> Self {
        Self(value)
    }
}

impl IntoTwilight<TwilightEmbed> for Embed {
    fn into_twilight(self) -> TwilightEmbed {
        self.0
    }
}