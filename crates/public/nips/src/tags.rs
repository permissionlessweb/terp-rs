use crate::types::Tag;

/// Builder for constructing tags with a fluent API
#[derive(Debug, Clone, Default)]
pub struct TagBuilder {
    tags: Vec<Tag>,
}

impl TagBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn e(mut self, event_id: impl Into<String>) -> Self {
        self.tags.push(Tag::e(event_id));
        self
    }

    pub fn e_with_relay(mut self, event_id: impl Into<String>, relay: impl Into<String>) -> Self {
        self.tags.push(Tag::e_with_relay(event_id, relay));
        self
    }

    pub fn e_full(
        mut self,
        event_id: impl Into<String>,
        relay: impl Into<String>,
        author: impl Into<String>,
    ) -> Self {
        self.tags.push(Tag::e_full(event_id, relay, author));
        self
    }

    pub fn p(mut self, pubkey: impl Into<String>) -> Self {
        self.tags.push(Tag::p(pubkey));
        self
    }

    pub fn p_with_relay(mut self, pubkey: impl Into<String>, relay: impl Into<String>) -> Self {
        self.tags.push(Tag::p_with_relay(pubkey, relay));
        self
    }

    pub fn a(mut self, kind: u16, pubkey: impl Into<String>, d: impl Into<String>) -> Self {
        self.tags.push(Tag::a(kind, pubkey, d));
        self
    }

    pub fn a_with_relay(
        mut self,
        kind: u16,
        pubkey: impl Into<String>,
        d: impl Into<String>,
        relay: impl Into<String>,
    ) -> Self {
        self.tags.push(Tag::a_with_relay(kind, pubkey, d, relay));
        self
    }

    pub fn d(mut self, value: impl Into<String>) -> Self {
        self.tags.push(Tag::d(value));
        self
    }

    pub fn t(mut self, value: impl Into<String>) -> Self {
        self.tags.push(Tag::t(value));
        self
    }

    pub fn g(mut self, value: impl Into<String>) -> Self {
        self.tags.push(Tag::g(value));
        self
    }

    pub fn r(mut self, value: impl Into<String>) -> Self {
        self.tags.push(Tag::r(value));
        self
    }

    pub fn custom(mut self, tag: Tag) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn build(self) -> Vec<Tag> {
        self.tags
    }
}