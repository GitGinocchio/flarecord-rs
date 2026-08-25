use std::ops::Deref;

use twilight_model::application::interaction::modal::{ModalInteractionData};

pub struct ModalData(pub (crate) ModalInteractionData);

impl ModalData {
    /// Ottiene l'identificativo unico del modal (generato automaticamente dal tipo)
    pub fn custom_id(&self) -> &str {
        &self.0.custom_id
    }
}

impl From<ModalInteractionData> for ModalData {
    fn from(value: ModalInteractionData) -> Self {
        Self(value)
    }
}

impl Deref for ModalData {
    type Target = ModalInteractionData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}