use twilight_model::{
    channel::message::{
        component::{
            TextInput as TwilightTextInput, 
            TextInputStyle as TwilightTextStyle
        }
    }
};

use crate::traits::component::IntoTwilight;


/// Stili per i TextInput Discord
pub enum TextStyle {
    /// PlainText - Input semplice (≤60 chars UI optimized)
    Short,
    /// Paragraph - Testi lunghi con formattazione
    Paragraph,
}

impl IntoTwilight<TwilightTextStyle> for TextStyle {
    fn into_twilight(self) -> TwilightTextStyle {
        match self {
            TextStyle::Short => TwilightTextStyle::Short,
            TextStyle::Paragraph => TwilightTextStyle::Paragraph,
        }
    }
}

/// Builder per TextInput con chain ergonomico
pub struct TextInput(pub (crate) TwilightTextInput);

impl TextInput {
    /// Crea un nuovo builder per TextInput
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self(TwilightTextInput {
            id: None,
            custom_id: custom_id.into(),
            #[allow(deprecated)]
            label: None,
            max_length: None,
            min_length: None,
            placeholder: None,
            required: Some(true),
            style: TwilightTextStyle::Short,
            value: None 
        })
    }

    #[deprecated(note = "label field is deprecated")]
    #[allow(deprecated)]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.0.label = Some(label.into());
        self
    }

    /// Imposta il placeholder text (opzionale)
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.0.placeholder = Some(placeholder.into());
        self
    }

    /// Imposta il valore pre-compilato (opzionale)
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.0.value = Some(value.into());
        self
    }

    /// Imposta la lunghezza massima dei caratteri (1-4000)
    pub fn max_length(mut self, max_len: u16) -> Self {
        if max_len < 1 || max_len > 4000 {
            worker::console_warn!("TextInput max_length must be between 1 and 4000");
        }
        self.0.max_length = Some(max_len);
        self
    }

    /// Imposta la lunghezza minima dei caratteri (opzionale, 0-4000)
    pub fn min_length(mut self, min_len: u16) -> Self {
        if min_len > 4000 {
            worker::console_warn!("TextInput min_length must be <= 4000");
        }
        self.0.min_length = Some(min_len);
        self
    }

    /// Imposta se il campo è richiesto
    pub fn required(mut self, required: bool) -> Self {
        self.0.required = Some(required);
        self
    }

    /// Imposta lo stile del TextInput (PlainText o Paragraph)
    pub fn style(mut self, style: TextStyle) -> Self {
        self.0.style = style.into_twilight();
        self
    }
}

impl IntoTwilight<TwilightTextInput> for TextInput {
    fn into_twilight(self) -> TwilightTextInput {
        self.0
    }
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new("default")
    }
}
