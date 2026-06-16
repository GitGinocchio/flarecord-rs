use twilight_model::channel::message::{
    Component as TwilightComponent, 
    component::Separator as TwilightSeparator
};

use crate::traits::component::IntoTwilight;




#[derive(Clone)]
pub struct Separator(TwilightSeparator);

impl Separator {
    pub fn new() -> Self {
        Self(TwilightSeparator {
            id: None,
            divider: None,
            spacing: None
        })
    }

    pub fn spacing(mut self, spacing: u8) -> Self {
        self.0.spacing = Some(spacing.into());
        self
    }

    pub fn divider(mut self, divider: bool) -> Self {
        self.0.divider = Some(divider);
        self
    }
}

impl IntoTwilight<TwilightSeparator> for Separator {
    fn into_twilight(self) -> TwilightSeparator {
        self.0
    }
}

impl IntoTwilight<TwilightComponent> for Separator {
    fn into_twilight(self) -> TwilightComponent {
        TwilightComponent::Separator(self.into_twilight())
    }
}