use twilight_model::channel::message::{
    Component as TwilightComponent, 
    component::{
        MediaGallery as TwilightMediaGallery,
        MediaGalleryItem as TwilightMediaGalleryItem, 
        UnfurledMediaItem
    }
};

use crate::traits::component::IntoTwilight;

pub struct MediaGalleryItem(TwilightMediaGalleryItem);

impl MediaGalleryItem {
    pub fn new(url: impl Into<String>) -> Self {
        Self(TwilightMediaGalleryItem {
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
        self.0.description = Some(description.into());
        self
    }

    pub fn spoiler(mut self, spoiler: bool) -> Self {
        self.0.spoiler = Some(spoiler);
        self
    }
}

impl MediaGallery {
    pub fn new() -> MediaGalleryState<0> {
        MediaGalleryState(Vec::new())
    }
}

pub struct MediaGalleryState<const N: usize>(Vec<TwilightMediaGalleryItem>);

impl MediaGalleryState<0> {
    pub fn add_item(mut self, item: MediaGalleryItem) -> MediaGalleryState<1> {
        self.0.push(item.into_twilight());
        MediaGalleryState(self.0)
    }
}

macro_rules! define_media_gallery {
    ($( ($variant:ident($n:expr), $next:expr) ),*) => {
        
        pub enum MediaGallery {
            $( $variant(MediaGalleryState<$n>), )*
            Has10(MediaGalleryState<10>)
        }

        $(
            impl MediaGalleryState<$n> {
                pub fn build(self) -> MediaGallery {
                    MediaGallery::$variant(self)
                }

                pub fn add_item(mut self, item: MediaGalleryItem) -> MediaGalleryState<$next> {
                    self.0.push(item.into_twilight());
                    MediaGalleryState(self.0)
                }
            }

            impl From<MediaGalleryState<$n>> for MediaGallery {
                fn from(state: MediaGalleryState<$n>) -> Self {
                    MediaGallery::$variant(state)
                }
            }
        )*
    };
}

define_media_gallery!(
    (Has1(1), 2),
    (Has2(2), 3),
    (Has3(3), 4),
    (Has4(4), 5),
    (Has5(5), 6),
    (Has6(6), 7),
    (Has7(7), 8),
    (Has8(8), 9),
    (Has9(9), 10)
);

impl MediaGalleryState<10> {
    pub fn build(self) -> MediaGallery {
        MediaGallery::Has10(self)
    }
}

impl IntoTwilight<TwilightMediaGalleryItem> for MediaGalleryItem {
    fn into_twilight(self) -> TwilightMediaGalleryItem {
        TwilightMediaGalleryItem {
            media: self.0.media,
            description: self.0.description,
            spoiler: self.0.spoiler
        }
    }
}

impl IntoTwilight<TwilightComponent> for MediaGallery {
    fn into_twilight(self) -> TwilightComponent {
        let items = match self {
            Self::Has1(mg) => mg.0,
            Self::Has2(mg) => mg.0,
            Self::Has3(mg) => mg.0,
            Self::Has4(mg) => mg.0,
            Self::Has5(mg) => mg.0,
            Self::Has6(mg) => mg.0,
            Self::Has7(mg) => mg.0,
            Self::Has8(mg) => mg.0,
            Self::Has9(mg) => mg.0,
            Self::Has10(mg) => mg.0,
        };

        TwilightComponent::MediaGallery(TwilightMediaGallery {
            id: None,
            items: items
        })
    }
}