use std::marker::PhantomData;

use twilight_model::channel::message::{
    Component as TwilightComponent, 
    component::Section as TwilightSection
};

use crate::{models::components::{content::{text_display::TextDisplay, thumbnail::Thumbnail}, id::IdAssignable, interactive::button::Button}, traits::component::IntoTwilight};

pub enum SectionComponent {
    TextDisplay(TextDisplay)
}

impl IdAssignable for SectionComponent {
    fn set_id(&mut self, _id: &crate::models::components::id::HierarchicalId) {
        match self {
            Self::TextDisplay(_text_display) => {}
        }
    }
}

impl From<TextDisplay> for SectionComponent {
    fn from(value: TextDisplay) -> Self {
        Self::TextDisplay(value)
    }
}

pub enum SectionAccessory {
    Button(Button),
    Thumbnail(Thumbnail)
}

impl IdAssignable for SectionAccessory {
    fn set_id(&mut self, id: &crate::models::components::id::HierarchicalId) {
        match self {
            Self::Button(button) => button.set_id(id),
            Self::Thumbnail(_thumb) => {}
        }
    }
}

impl From<Button> for SectionAccessory {
    fn from(value: Button) -> Self {
        Self::Button(value)
    }
}

impl From<Thumbnail> for SectionAccessory {
    fn from(value: Thumbnail) -> Self {
        Self::Thumbnail(value)
    }
}

pub struct Empty;
pub struct HasComponent;
pub struct HasAccessory;
pub struct Ready;

pub enum Section {
    Ready(SectionState<HasComponent, HasAccessory>),
}

impl Section {
    pub fn new() -> SectionState<Empty, Empty> {
        SectionState::new()
    }

    pub (crate) fn get_accessory(&self) -> Option<&SectionAccessory> {
        match self {
            Self::Ready(state) => state.accessory.as_ref()
        }
    }

    pub (crate) fn get_id(&self) -> i32 {
        match self {
            Self::Ready(state) => state.id
        }
    }

    pub (crate) fn count(&self) -> usize {
        match self {
            Self::Ready(ready) => ready.components.len()
        }
    }
}

#[allow(unused)]
pub struct SectionState<C, A> {
    id: i32,
    components: Vec<SectionComponent>,
    accessory: Option<SectionAccessory>,
    _marker: PhantomData<(C, A)>
}

impl SectionState<Empty, Empty> {
    fn new() -> Self {
        Self {
            id: 0,
            components: Vec::new(),
            accessory: None,
            _marker: PhantomData
        }
    }

    pub fn accessory(self, accessory: impl Into<SectionAccessory>) -> SectionState<Empty, HasAccessory> {
        SectionState {
            id: self.id,
            components: self.components,
            accessory: Some(accessory.into()),
            _marker: PhantomData
        }
    }

    pub fn component(self, component: impl Into<SectionComponent>) -> SectionState<HasComponent, Empty> {
        add_component(self, component.into())
    }
}

fn add_component<A, B, C, D>(mut state: SectionState<A, B>, component: SectionComponent) -> SectionState<C, D> {
    state.components.push(component);
    SectionState { id: state.id, components: state.components, accessory: state.accessory, _marker: PhantomData }
}

impl IdAssignable for Section {
    fn children(&mut self) -> Box<dyn Iterator<Item = &mut dyn IdAssignable> + '_> {
        match self {
            Self::Ready(section) => {
                let iter = section.components
                    .iter_mut()
                    .map(|c| c as &mut dyn IdAssignable);

                if let Some(accessory) = section.accessory.as_mut() {
                    Box::new(iter.chain(std::iter::once(accessory as &mut dyn IdAssignable)))
                } else {
                    Box::new(iter)
                }
            }
        }
    }

    fn set_id(&mut self, id: &crate::models::components::id::HierarchicalId) {
        match self {
            Self::Ready(section) => section.id = id.as_usize() as i32
        }
    }
}

impl SectionState<HasComponent, Empty> {
    pub fn component(self, component: impl Into<SectionComponent>) -> SectionState<HasComponent, Empty> {
        add_component(self, component.into())
    }
    
    pub fn accessory(self, accessory: impl Into<SectionAccessory>) -> SectionState<HasComponent, HasAccessory> {
        SectionState {
            id: self.id,
            components: self.components,
            accessory: Some(accessory.into()),
            _marker: PhantomData
        }
    }
}

impl SectionState<Empty, HasAccessory> {
    pub fn component(self, component: impl Into<SectionComponent>) -> SectionState<HasComponent, HasAccessory> {
        add_component(self, component.into())
    }
}

impl SectionState<HasComponent, HasAccessory> {
    pub fn component(self, component: SectionComponent) -> SectionState<HasComponent, Empty> {
        add_component(self, component)
    }
}

impl SectionState<HasComponent, HasAccessory> {
    pub fn build(self) -> Section {
        Section::Ready(self)
    }
}

impl IntoTwilight<TwilightComponent> for SectionAccessory {
    fn into_twilight(self) -> TwilightComponent {
        match self {
            Self::Button(button) => TwilightComponent::Button(button.into_twilight()),
            Self::Thumbnail(thumbnail) => TwilightComponent::Thumbnail(thumbnail.into_twilight())
        }
    }
}

impl IntoTwilight<TwilightComponent> for SectionComponent {
    fn into_twilight(self) -> TwilightComponent {
        match self {
            Self::TextDisplay(text_display) => TwilightComponent::TextDisplay(text_display.into_twilight())
        }
    }
}

impl IntoTwilight<TwilightSection> for SectionState<HasComponent, HasAccessory> {
    fn into_twilight(self) -> TwilightSection {
        TwilightSection {
            id: Some(self.id),
            accessory: Box::new(self.accessory.expect("Section should be ready").into_twilight()),
            components: self.components
                .into_iter()
                .map(|c| c.into_twilight())
                .collect()
        }
    }
}

impl IntoTwilight<TwilightComponent> for Section {
    fn into_twilight(self) -> TwilightComponent {
        match self {
            Self::Ready(ready) => TwilightComponent::Section(ready.into_twilight())
        }
    }
}