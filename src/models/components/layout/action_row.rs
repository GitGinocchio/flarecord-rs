use std::marker::PhantomData;

use twilight_model::{
    channel::message::{
        Component as TwilightComponent, 
        component::ActionRow as TwilightActionRow
    }
};

use crate::{models::components::{id::IdAssignable, interactive::{button::Button, select::Select}}, traits::component::IntoTwilight};

pub struct Empty;
pub struct Has1;
pub struct Has2;
pub struct Has3;
pub struct Has4;
pub struct Has5;
pub struct HasSelect;

pub enum ActionRow {
    Empty(ActionRowState<Empty>),
    HasSelect(ActionRowState<HasSelect>),
    Has1(ActionRowState<Has1>),
    Has2(ActionRowState<Has2>),
    Has3(ActionRowState<Has3>),
    Has4(ActionRowState<Has4>),
    Has5(ActionRowState<Has5>),
}

pub enum ActionRowChild {
    Button(Button),
    Select(Select)
}

impl IdAssignable for ActionRowChild {
    fn set_id(&mut self, id: &crate::models::components::id::HierarchicalId) {
        match self {
            Self::Button(b) => b.set_id(id),
            Self::Select(s) => s.set_id(id)
        }
    }
}

#[allow(unused)]
impl ActionRow {
    pub fn new() -> ActionRowState<Empty> {
        ActionRowState { 
            components: Vec::new(), 
            _marker: PhantomData,
            id: 0
        }
    }

    pub (crate) fn count(&self) -> usize {
        match self {
            Self::Empty(_) => 0,
            Self::Has1(_) => 1,
            Self::Has2(_) => 2,
            Self::Has3(_) => 3,
            Self::Has4(_) => 3,
            Self::Has5(_) => 4,
            Self::HasSelect(_) => 1
        }
    }

    pub (crate) fn get_id(&self) -> i32 {
        match self {
            ActionRow::Empty(a) => a.id,
            ActionRow::Has1(a) => a.id,
            ActionRow::Has2(a) => a.id,
            ActionRow::Has3(a) => a.id,
            ActionRow::Has4(a) => a.id,
            ActionRow::Has5(a) => a.id,
            ActionRow::HasSelect(a) => a.id
        }
    }

    pub (crate) fn get_children(&self) -> &[ActionRowChild] {
        match self {
            ActionRow::Empty(a) => &a.components,
            ActionRow::Has1(a) => &a.components,
            ActionRow::Has2(a) => &a.components,
            ActionRow::Has3(a) => &a.components,
            ActionRow::Has4(a) => &a.components,
            ActionRow::Has5(a) => &a.components,
            ActionRow::HasSelect(a) => &a.components
        }
    }
}

pub struct ActionRowState<S> {
    pub (crate) components: Vec<ActionRowChild>,
    id: i32,
    _marker: PhantomData<S>,
}

fn add_button<T, N>(mut ars: ActionRowState<T>, b: Button) -> ActionRowState<N> {
    ars.components.push(ActionRowChild::Button(b));
    ActionRowState { components: ars.components, _marker: PhantomData, id: ars.id }
}

impl ActionRowState<Empty> {
    pub fn select(mut self, s: Select) -> ActionRowState<HasSelect> {
        self.components.push(ActionRowChild::Select(s));
        ActionRowState { components: self.components, _marker: PhantomData, id: self.id }
    }

    pub fn button(self, b: Button) -> ActionRowState<Has1> {
        add_button(self, b)
    }
}

impl ActionRowState<Has1> {
    pub fn button(self, b: Button) -> ActionRowState<Has2> {
        add_button(self, b)
    }
}

impl ActionRowState<Has2> {
    pub fn button(self, b: Button) -> ActionRowState<Has3> {
        add_button(self, b)
    }
}

impl ActionRowState<Has3> {
    pub fn button(self, b: Button) -> ActionRowState<Has4> {
        add_button(self, b)
    }
}

impl ActionRowState<Has4> {
    pub fn button(self, b: Button) -> ActionRowState<Has5> {
        add_button(self, b)
    }
}

pub trait IntoActionRow {
    fn build(self) -> ActionRow;
}

macro_rules! impl_action_row {
    ($(($state:ident, $variant:ident)),* $(,)?) => {
        $(
            impl IntoActionRow for ActionRowState<$state> {
                fn build(self) -> ActionRow {
                    ActionRow::$variant(self)
                }
            }

            impl Into<ActionRow> for ActionRowState<$state> {
                fn into(self) -> ActionRow {
                    ActionRow::$variant(self)
                }
            }
        )*
    };
}

impl_action_row!(
    (Empty, Empty),
    (Has1, Has1),
    (Has2, Has2),
    (Has3, Has3),
    (Has4, Has4),
    (Has5, Has5),
    (HasSelect, HasSelect),
);

impl IdAssignable for ActionRow {
    fn set_id(&mut self, id: &crate::models::components::id::HierarchicalId) {
        match self {
            Self::Empty(s) => s.id = id.as_usize() as i32,
            Self::Has1(s) => s.id = id.as_usize() as i32,
            Self::Has2(s) => s.id = id.as_usize() as i32,
            Self::Has3(s) => s.id = id.as_usize() as i32,
            Self::Has4(s) => s.id = id.as_usize() as i32,
            Self::Has5(s) => s.id = id.as_usize() as i32,
            Self::HasSelect(s) => s.id = id.as_usize() as i32,
        }
    }

    fn children(&mut self) -> Box<dyn Iterator<Item = &mut dyn IdAssignable> + '_> {
        match self {
            Self::Empty(s) => Box::new(s.components.iter_mut().map(|c| c as &mut dyn IdAssignable)),
            Self::Has1(s) => Box::new(s.components.iter_mut().map(|c| c as &mut dyn IdAssignable)),
            Self::Has2(s) => Box::new(s.components.iter_mut().map(|c| c as &mut dyn IdAssignable)),
            Self::Has3(s) => Box::new(s.components.iter_mut().map(|c| c as &mut dyn IdAssignable)),
            Self::Has4(s) => Box::new(s.components.iter_mut().map(|c| c as &mut dyn IdAssignable)),
            Self::Has5(s) => Box::new(s.components.iter_mut().map(|c| c as &mut dyn IdAssignable)),
            Self::HasSelect(s) => Box::new(s.components.iter_mut().map(|c| c as &mut dyn IdAssignable)),
        }
    }
}

impl IntoTwilight<TwilightComponent> for ActionRowChild {
    fn into_twilight(self) -> TwilightComponent {
        match self {
            Self::Button(button) => TwilightComponent::Button(button.into_twilight()),
            Self::Select(select) => TwilightComponent::SelectMenu(select.into_twilight())
        }
    }
}

impl<T> IntoTwilight<TwilightActionRow> for ActionRowState<T> {
    fn into_twilight(self) -> TwilightActionRow {
        TwilightActionRow {
            id: Some(self.id),
            components: self.components
                .into_iter()
                .map(|c| c.into_twilight())
                .collect()
        }
    }
}

impl IntoTwilight<TwilightComponent> for ActionRow {
    fn into_twilight(self) -> TwilightComponent {
        match self {
            Self::Empty(empty) => TwilightComponent::ActionRow(empty.into_twilight()),
            Self::Has1(select) => TwilightComponent::ActionRow(select.into_twilight()),
            Self::Has2(select) => TwilightComponent::ActionRow(select.into_twilight()),
            Self::Has3(select) => TwilightComponent::ActionRow(select.into_twilight()),
            Self::Has4(select) => TwilightComponent::ActionRow(select.into_twilight()),
            Self::Has5(select) => TwilightComponent::ActionRow(select.into_twilight()),
            Self::HasSelect(select) => TwilightComponent::ActionRow(select.into_twilight()),
        }
    }
}