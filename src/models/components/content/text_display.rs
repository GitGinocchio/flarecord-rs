use twilight_model::channel::message::component::TextDisplay as TwilightTextDisplay;

use crate::traits::component::{IntoTwilight};

pub struct TextDisplay(TwilightTextDisplay);

impl TextDisplay {
    pub fn new() -> Self {
        Self(TwilightTextDisplay {
            id: None,
            content: String::new()
        })
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.0.content = content.into();
        self
    }

    pub fn heading(mut self, level: u8, text: impl Into<String>) -> Self {
        let level = level.clamp(1, 3);
        let hashes = "#".repeat(level as usize);
        
        self.0.content.push_str(&format!("{} {}\n", hashes, text.into()));
        self
    }

    pub fn paragraph(mut self, text: impl Into<String>) -> Self {
        self.0.content.push_str(&format!("{} ", text.into()));
        self
    }

    pub fn bold(mut self, text: impl Into<String>) -> Self {
        self.0.content.push_str(&format!("**{}** ", text.into()));
        self
    }

    pub fn newline(mut self) -> Self {
        self.0.content.push_str("\n");
        self
    }
}

impl IntoTwilight<TwilightTextDisplay> for TextDisplay {
    fn into_twilight(self) -> TwilightTextDisplay {
        self.0
    }
}