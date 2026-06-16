use twilight_model::channel::message::{
    Component as TwilightComponent, 
    component::Container as TwilightContainer
};

use crate::{models::{color::Color, components::{content::media_gallery::MediaGallery, id::ID_GEN, layout::{action_row::ActionRow, section::Section, separator::Separator}}}, traits::component::IntoTwilight};

pub enum ContainerChild {
    ActionRow(ActionRow),
    Section(Section),
    Separator(Separator),
    MediaGallery(MediaGallery)
}

impl ContainerChild {
    pub (crate) fn get_id(&mut self) -> i32 {
        match self {
            Self::ActionRow(action_row) => action_row.get_id(),
            Self::Section(section) => section.get_id(),
            Self::Separator(_) => 0,
            Self::MediaGallery(_) => 0
        }
    }
}

impl From<Section> for ContainerChild {
    fn from(value: Section) -> Self {
        Self::Section(value)
    }
}

impl From<ActionRow> for ContainerChild {
    fn from(value: ActionRow) -> Self {
        Self::ActionRow(value)
    }
}

impl From<Separator> for ContainerChild {
    fn from(value: Separator) -> Self {
        Self::Separator(value)
    }
}

impl From<MediaGallery> for ContainerChild {
    fn from(value: MediaGallery) -> Self {
        Self::MediaGallery(value)
    }
}

pub struct Container {
    children: Vec<ContainerChild>,
    accent_color: Option<Color>,
    spoiler: Option<bool>,
    id: i32
}

impl Container {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            accent_color: None,
            spoiler: None,
            id: ID_GEN.next_i32()
        }
    }

    pub fn accent_color(mut self, color: Color) -> Self {
        self.accent_color = Some(color);
        self
    }

    pub fn add(mut self, child: impl Into<ContainerChild>) -> Self {
        let child = child.into();
        self.children.push(child);
        self
    }

    pub (crate) fn get_id(&self) -> i32 {
        self.id
    }
}

impl IntoTwilight<TwilightComponent> for ContainerChild {
    fn into_twilight(self) -> TwilightComponent {
        match self {
            Self::ActionRow(action_row) => action_row.into_twilight(),
            Self::Section(section) => section.into_twilight(),
            Self::Separator(separator) => separator.into_twilight(),
            Self::MediaGallery(media_gallery) => media_gallery.into_twilight()
        }
    }
}

impl IntoTwilight<TwilightContainer> for Container {
    fn into_twilight(self) -> TwilightContainer {
        TwilightContainer {
            id: Some(self.id),
            spoiler: self.spoiler,
            accent_color: Some(self.accent_color.map(|v| v.into())),
            components: self.children
                .into_iter()
                .map(|c| c.into_twilight())
                .collect()
        }
    }
}

impl IntoTwilight<TwilightComponent> for Container {
    fn into_twilight(self) -> TwilightComponent {
        TwilightComponent::Container(self.into_twilight())
    }
}