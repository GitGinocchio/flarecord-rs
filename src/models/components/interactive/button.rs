use std::marker::PhantomData;

use twilight_model::{
    channel::message::{
        EmojiReactionType, 
        component::{
            Button as TwilightButton, 
            ButtonStyle as TwilightButtonStyle
        }
    }, 
    id::{Id, marker::SkuMarker
    }
};

use crate::{error::BotResult, models::{components::{context::ComponentContext, id::IdAssignable, interaction::ComponentInteraction, interactive::{Handler, InteractiveComponentHandler}}}, traits::component::IntoTwilight};


pub enum ButtonStyle {
    Primary,
    Secondary,
    Success,
    Danger,
}

impl Into<TwilightButtonStyle> for ButtonStyle {
    fn into(self) -> TwilightButtonStyle {
        match self {
            ButtonStyle::Primary => TwilightButtonStyle::Primary,
            ButtonStyle::Secondary => TwilightButtonStyle::Secondary,
            ButtonStyle::Success => TwilightButtonStyle::Success,
            ButtonStyle::Danger => TwilightButtonStyle::Danger,
        }
    }
}

pub struct Empty;
pub struct Premium;
pub struct Normal;
pub struct Link;

pub enum Button {
    Normal(ButtonKind<Normal>),
    Premium(ButtonKind<Premium>),
    Link(ButtonKind<Link>)
}

impl Button {
    pub fn new() -> ButtonKind<Empty> {
        ButtonKind::new()
    }
}

pub struct ButtonKind<S> {
    pub (crate) inner: TwilightButton,
    pub(crate) handler: Option<Box<dyn InteractiveComponentHandler>>,
    pub (crate) _marker: PhantomData<S>
}

impl ButtonKind<Empty> {
    pub fn new() -> Self {
        Self {
            inner: TwilightButton {
                custom_id: None,
                id: None,
                disabled: false,
                emoji: None,
                label: None,
                style: TwilightButtonStyle::Primary,
                sku_id: None,
                url: None
            },
            _marker: PhantomData,
            handler: None
        }
    }

    pub fn style(mut self, style: ButtonStyle) -> ButtonKind<Normal> {
        self.inner.style = style.into();

        ButtonKind {
            inner: self.inner,
            handler: self.handler,
            _marker: PhantomData
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> ButtonKind<Link> {
        self.inner.style = TwilightButtonStyle::Link;
        self.inner.url = Some(url.into());
        
        ButtonKind {
            inner: self.inner,
            handler: self.handler,
            _marker: PhantomData,
        }
    }

    pub fn premium(mut self, sku_id: Id<SkuMarker>) -> ButtonKind<Premium> {
        self.inner.style = TwilightButtonStyle::Premium;
        self.inner.sku_id = Some(sku_id);
        self.inner.custom_id = None;
        self.inner.label = None;
        self.inner.url = None;
        self.inner.emoji = None;
        
        ButtonKind {
            inner: self.inner,
            handler: self.handler,
            _marker: PhantomData,
        }
    }
}

impl ButtonKind<Normal> {
    /// If is not specified an `on_click` method the interaction is sent to the base Component
    pub fn on_click<F, Fut>(mut self, handler: F) -> Self
    where 
        F: Fn(ComponentInteraction, ComponentContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = BotResult<()>> + 'static,
    {
        self.handler = Some(Box::new(Handler(handler)));
        self
    }

    pub (crate) async fn clicked(&self, interaction: ComponentInteraction, ctx: ComponentContext) -> BotResult<()> {
        if let Some(handler) = &self.handler {
            return handler.handle(interaction, ctx).await;
        } else {
            Ok(())
        }
    } 
}

macro_rules! impl_common_button_methods {
    ($($state:ident),* $(,)?) => {
        $(
            impl ButtonKind<$state> {
                pub fn label(mut self, label: impl Into<String>) -> Self {
                    self.inner.label = Some(label.into());
                    self
                }

                pub fn emoji(mut self, emoji: EmojiReactionType) -> Self {
                    self.inner.emoji = Some(emoji);
                    self
                }
            }
        )*
    };
}

macro_rules! impl_into_button {
    ($(($state:ident, $variant:ident)),* $(,)?) => {
        $(
            impl ButtonKind<$state> {
                pub fn build(self) -> Button {
                    Button::$variant(self)
                }
            }

            impl From<ButtonKind<$state>> for Button {
                fn from(kind: ButtonKind<$state>) -> Self {
                    Button::$variant(kind)
                }
            }
        )*
    };
}

impl_common_button_methods!(Normal, Link);

impl_into_button!(
    (Normal, Normal),
    (Premium, Premium),
    (Link, Link),
);

impl IdAssignable for Button {
    fn set_id(&mut self, id: &crate::models::components::id::HierarchicalId) {
        match self {
            Self::Normal(normal) => normal.inner.custom_id = Some(id.as_string()),
            Self::Premium(_premium) => {},
            Self::Link(_link) => {}
        }
    }
}

impl IntoTwilight<TwilightButton> for Button {
    fn into_twilight(self) -> TwilightButton {
        match self {
            Self::Normal(button) => TwilightButton {
                id: button.inner.id,
                custom_id: button.inner.custom_id,
                disabled: button.inner.disabled,
                emoji: button.inner.emoji,
                label: button.inner.label,
                style: button.inner.style,
                url: button.inner.url,
                sku_id: button.inner.sku_id
            },
            Self::Premium(button) => TwilightButton {
                id: button.inner.id,
                custom_id: button.inner.custom_id,
                disabled: button.inner.disabled,
                emoji: button.inner.emoji,
                label: button.inner.label,
                style: button.inner.style,
                url: button.inner.url,
                sku_id: button.inner.sku_id
            },
            Self::Link(button) => TwilightButton {
                id: button.inner.id,
                custom_id: button.inner.custom_id,
                disabled: button.inner.disabled,
                emoji: button.inner.emoji,
                label: button.inner.label,
                style: button.inner.style,
                url: button.inner.url,
                sku_id: button.inner.sku_id
            }
        }
    }
}