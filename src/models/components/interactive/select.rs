use std::marker::PhantomData;

use twilight_model::{
    channel::{
        ChannelType, 
        message::{
            component::{
                SelectDefaultValue, 
                SelectMenu as TwilightSelectMenu, 
                SelectMenuType
            }
        }
    }, 
    id::{
        Id, 
        marker::{
            ChannelMarker, 
            GenericMarker, 
            RoleMarker, 
            UserMarker
        }
    }
};

use crate::{models::components::{context::ComponentContext, interaction::ComponentInteraction}, traits::component::IntoTwilight};

pub enum Select {
    String(SelectKind<String>),
    User(SelectKind<Id<UserMarker>>),
    Role(SelectKind<Id<RoleMarker>>),
    Mentionable(SelectKind<Id<GenericMarker>>),
    Channel(SelectKind<Id<ChannelMarker>>)
}

impl Select {
    pub (crate) fn set_id(&mut self, id: String) {
        match self {
            Select::Channel(select) => select.set_id(id),
            Select::String(select) => select.set_id(id),
            Select::User(select) => select.set_id(id),
            Select::Role(select) => select.set_id(id),
            Select::Mentionable(select) => select.set_id(id)
        }
    }

    pub fn string() -> SelectKind<String> {
        Self::new(SelectMenuType::Text)
    }

    pub fn user() -> SelectKind<Id<UserMarker>> {
        Self::new(SelectMenuType::User)
    }

    pub fn role() -> SelectKind<Id<RoleMarker>> {
        Self::new(SelectMenuType::Role)
    }

    pub fn mentionable() -> SelectKind<Id<GenericMarker>> {
        Self::new(SelectMenuType::Mentionable)
    }

    pub fn channel() -> SelectKind<Id<ChannelMarker>> {
        Self::new(SelectMenuType::Channel)
    }

    fn new<T>(kind: SelectMenuType) -> SelectKind<T> {
        SelectKind {
            inner: TwilightSelectMenu {
                id: None,
                channel_types: None,
                custom_id: "0".into(),
                default_values: None,
                disabled: false,
                kind: kind,
                max_values: None,
                min_values: None,
                options: Some(vec![]),
                placeholder: None,
                required: None
            },
            handler: None,
            _marker: PhantomData
        }
    }
}

pub struct SelectKind<T> {
    pub(crate) inner: TwilightSelectMenu,
    pub(crate) handler: Option<Box<dyn Fn(ComponentInteraction, ComponentContext) + Send + Sync>>,
    _marker: PhantomData<T>,
}

impl SelectKind<Id<ChannelMarker>> {
    pub fn channel_types(mut self, channel_types: Vec<ChannelType> ) -> Self {
        self.inner.channel_types = Some(channel_types);
        self
    }
}

impl<T> SelectKind<T> {
    pub fn on_select<F>(mut self, handler: F) -> Self 
    where F: Fn(ComponentInteraction, ComponentContext) + Send + Sync + 'static 
    {
        self.handler = Some(Box::new(handler));
        self
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.inner.placeholder = Some(text.into());
        self
    }

    pub fn min_values(mut self, min: u8) -> Self {
        self.inner.min_values = Some(min);
        self
    }

    pub fn max_values(mut self, max: u8) -> Self {
        self.inner.max_values = Some(max);
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.inner.required = Some(required);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.inner.disabled = disabled;
        self
    }
}

impl<T> SelectKind<T> {
    pub (crate) fn set_id(&mut self, id: String) {
        self.inner.custom_id = id;
    }
}

macro_rules! impl_default_values {
    ($(($t:ty, $variant:ident)),* $(,)?) => {
        $(
            impl SelectKind<$t> {
                pub fn default_values(mut self, values: Vec<$t>) -> Self {
                    self.inner.default_values = Some(
                        values.into_iter()
                            .map(|id| SelectDefaultValue::$variant(id))
                            .collect()
                    );
                    self
                }
            }
        )*
    };
}

impl_default_values!(
    (Id<ChannelMarker>, Channel),
    (Id<UserMarker>, User),
    (Id<RoleMarker>, Role),
);

macro_rules! impl_into_select {
    ($(($t:ty, $variant:ident)),* $(,)?) => {
        $(
            impl SelectKind<$t> {
                pub fn build(self) -> Select {
                    Select::$variant(self)
                }
            }

            impl From<SelectKind<$t>> for Select {
                fn from(kind: SelectKind<$t>) -> Self {
                    Select::$variant(kind)
                }
            }
        )*
    };
}

impl_into_select!(
    (String, String),
    (Id<UserMarker>, User),
    (Id<RoleMarker>, Role),
    (Id<GenericMarker>, Mentionable),
    (Id<ChannelMarker>, Channel),
);

impl<T> IntoTwilight<TwilightSelectMenu> for SelectKind<T> {
    fn into_twilight(self) -> TwilightSelectMenu {
        TwilightSelectMenu {
            id: self.inner.id,
            channel_types: self.inner.channel_types,
            custom_id: self.inner.custom_id,
            default_values: self.inner.default_values,
            disabled: self.inner.disabled,
            kind: self.inner.kind,
            max_values: self.inner.max_values,
            min_values: self.inner.min_values,
            options: self.inner.options,
            placeholder: self.inner.placeholder,
            required: self.inner.required
        }
    }
}

impl IntoTwilight<TwilightSelectMenu> for Select {
    fn into_twilight(self) -> TwilightSelectMenu {
        match self {
            Self::String(select) => select.into_twilight(),
            Self::Channel(select) => select.into_twilight(),
            Self::User(select) => select.into_twilight(),
            Self::Role(select) => select.into_twilight(),
            Self::Mentionable(select) => select.into_twilight()
        }
    }
}