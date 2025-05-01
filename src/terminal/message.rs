#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageType {
    None,
    SendInput,
    ShowPopup,
    Quit,
}

pub struct Message {
    pub message_type: MessageType,
    pub content: Option<String>,
}

impl Message {
    pub fn new(message_type: MessageType, content: Option<String>) -> Self {
        Self {
            message_type,
            content,
        }
    }

    pub fn send_input(content: String) -> Self {
        Self::new(MessageType::SendInput, Some(content))
    }

    pub fn show_popup() -> Self {
        Self::new(MessageType::ShowPopup, None)
    }

    pub fn none() -> Self {
        Self::new(MessageType::None, None)
    }

    pub fn quit() -> Self {
        Self::new(MessageType::Quit, None)
    }
}
