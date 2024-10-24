use crate::{Action, InputMessage, NoteFilter, OutputMessage};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Default)]
pub struct Kinds {
    allowed_kinds: Option<Vec<i64>>,
    blocked_kinds: Option<Vec<i64>>,
    messages: Option<HashMap<String, String>>,
}

impl NoteFilter for Kinds {
    fn filter_note(&mut self, input: &InputMessage) -> OutputMessage {
        let kind = input.event.kind;
        let msg = self
            .messages
            .as_ref()
            .and_then(|msgs| msgs.get(&kind.to_string()).cloned())
            .unwrap_or_else(|| "blocked: note kind is not allowed here".to_string());
        if let Some(allowed_kinds) = &self.allowed_kinds {
            if !allowed_kinds.contains(&kind) {
                return OutputMessage::new(input.event.id.clone(), Action::Reject, Some(msg));
            }
        }
        if let Some(blocked_kinds) = &self.blocked_kinds {
            if blocked_kinds.contains(&kind) {
                return OutputMessage::new(input.event.id.clone(), Action::Reject, Some(msg));
            }
        }
        OutputMessage::new(input.event.id.clone(), Action::Accept, None)
    }

    fn name(&self) -> &'static str {
        "kinds"
    }
}
