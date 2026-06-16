use std::marker::PhantomData;

use twilight_model::{
    channel::message::{
        Component as TwilightComponent, 
        component::ActionRow as TwilightActionRow
    }
};

use crate::{models::components::{id::ID_GEN, interactive::{button::Button, select::Select}}, traits::component::IntoTwilight};

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

impl ActionRowChild {
    pub (crate) fn get_id(&self) -> String {
        match self {
            Self::Button(btn) => btn.get_id(),
            Self::Select(select) => select.get_id()
        }
    }
}

#[allow(unused)]
impl ActionRow {
    pub fn new() -> ActionRowState<Empty> {
        ActionRowState { 
            components: Vec::new(), 
            _marker: PhantomData,
            id: ID_GEN.next_i32()
        }
    }

    pub (crate) fn get_children(&self) -> &Vec<ActionRowChild> {
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

    pub (crate) fn get_id(&self) -> i32 {
        match self {
            ActionRow::Empty(empty) => empty.get_id(),
            ActionRow::Has1(empty) => empty.get_id(),
            ActionRow::Has2(empty) => empty.get_id(),
            ActionRow::Has3(empty) => empty.get_id(),
            ActionRow::Has4(empty) => empty.get_id(),
            ActionRow::Has5(empty) => empty.get_id(),
            ActionRow::HasSelect(empty) => empty.get_id(),
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
            impl ActionRowState<$state> {
                pub (crate) fn get_id(&self) -> i32 {
                    self.id
                }
            }

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