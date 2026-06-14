use twilight_model::http::attachment::Attachment as TwilightOutgoingAttachment;

use crate::traits::component::IntoTwilight;

#[allow(unused)]
pub struct Attachment {
    filename: String,
    description: Option<String>,
    file: Option<Vec<u8>>,
    id: u64
}

#[allow(unused)]
impl Attachment {
    pub fn new(filename: impl Into<String>) -> Self {
        Self { 
            filename: filename.into(),
            ..Default::default()
        }
    }

    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = Some(description.into());
    }

    pub fn set_content(&mut self, content: impl Into<Vec<u8>>) {
        self.file = Some(content.into())
    }

    pub (crate) fn set_id(&mut self, id: u64) {
        self.id = id
    }
}

impl Default for Attachment {
    fn default() -> Self {
        Self {
            filename: "empty.file".into(),
            description: None,
            file: None,
            id: 0
        }
    }
}

impl IntoTwilight<TwilightOutgoingAttachment> for Attachment {
    fn into_twilight(self) -> TwilightOutgoingAttachment {
        TwilightOutgoingAttachment {
            description: self.description,
            file: self.file.unwrap_or(vec![]),
            filename: self.filename,
            id: self.id
        }
    }
}

impl From<TwilightOutgoingAttachment> for Attachment {
    fn from(value: TwilightOutgoingAttachment) -> Self {
        Self {
            filename: value.filename,
            file: Some(value.file),
            description: value.description,
            id: value.id
        }
    }
}