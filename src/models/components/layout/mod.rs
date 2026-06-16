use twilight_model::channel::message::Component as TwilightComponent;

use crate::{models::components::{ComponentType, layout::{action_row::ActionRow, container::Container, section::Section, separator::Separator}}, traits::component::{IntoComponent, IntoTwilight}};


pub mod action_row;
pub mod container;
pub mod separator;
pub mod section;

pub struct RootComponent(pub (crate) Vec<LayoutComponent>);

impl RootComponent {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub (crate) fn require_components_v2(&self) -> bool {
        for comp in self.0.iter() {
            if comp.require_components_v2() {
                return true
            }
        }

        false
    }

    pub (crate) fn count(&self) -> usize {
        todo!()
    }

    pub fn add<C: Into<LayoutComponent>>(&mut self, component: C) {
        let component = component.into();
        self.0.push(component);
    }
}

pub enum LayoutComponent {
    ActionRow(ActionRow),
    Container(Container),
    Section(Section),
    Separator(Separator)
}

impl LayoutComponent {
    pub (crate) fn require_components_v2(&self) -> bool {
        match self {
            Self::ActionRow(_) => false,
            Self::Container(_) => true,
            Self::Section(_) => true,
            Self::Separator(_) => true,
        }
    }

    pub (crate) fn get_id(&self) -> i32 {
        match self {
            Self::ActionRow(action_row) => action_row.get_id(),
            Self::Container(container) => container.get_id(),
            Self::Section(section) => section.get_id(),
            Self::Separator(_) => 0
        }
    }
}

impl From<Container> for LayoutComponent {
    fn from(value: Container) -> Self {
        Self::Container(value)
    }
}

impl From<ActionRow> for LayoutComponent {
    fn from(value: ActionRow) -> Self {
        Self::ActionRow(value)
    }
}

impl From<Separator> for LayoutComponent {
    fn from(value: Separator) -> Self {
        Self::Separator(value)
    }
}

impl From<Section> for LayoutComponent {
    fn from(value: Section) -> Self {
        Self::Section(value)
    }
}

impl IntoComponent for ActionRow {
    fn into_component(self) -> ComponentType {
        LayoutComponent::ActionRow(self).into_component()
    }
}

impl IntoComponent for Container {
    fn into_component(self) -> ComponentType {
        LayoutComponent::Container(self).into_component()
    }
}

impl IntoComponent for Section {
    fn into_component(self) -> ComponentType {
        LayoutComponent::Section(self).into_component()
    }
}

impl IntoComponent for Separator {
    fn into_component(self) -> ComponentType {
        LayoutComponent::Separator(self).into_component()
    }
}

impl IntoTwilight<TwilightComponent> for LayoutComponent {
    fn into_twilight(self) -> TwilightComponent {
        match self {
            Self::ActionRow(action_row) => action_row.into_twilight(),
            Self::Container(container) => container.into_twilight(),
            Self::Section(section) => section.into_twilight(),
            Self::Separator(separator) => separator.into_twilight()
        }
    }
}

impl IntoTwilight<Vec<TwilightComponent>> for RootComponent {
    fn into_twilight(self) -> Vec<TwilightComponent> {
        self.0.into_iter()
            .map(|c| c.into_twilight())
            .collect()
    }
}