use alloc::string::String;
use crate::schema::*;
use crate::StructuredIntentObject;

pub struct SioBuilder {
    header: SioHeader,
    intent: SioIntent,
    knowledge: SioKnowledge,
    ruleset: SioRuleset,
    reality: SioReality,
    runtime: SioRuntime,
    context: SioContext,
}

impl SioBuilder {
    pub fn new() -> Self {
        Self {
            header: SioHeader::default(),
            intent: SioIntent::default(),
            knowledge: SioKnowledge::default(),
            ruleset: SioRuleset::default(),
            reality: SioReality::default(),
            runtime: SioRuntime::default(),
            context: SioContext::default(),
        }
    }

    pub fn build(self) -> StructuredIntentObject {
        StructuredIntentObject {
            header: self.header,
            intent: self.intent,
            knowledge: self.knowledge,
            ruleset: self.ruleset,
            reality: self.reality,
            runtime: self.runtime,
            context: self.context,
        }
    }

    pub fn set_profile(mut self, profile: &str) -> Self {
        self.header.profile = profile.to_string();
        self
    }
}

impl Default for SioBuilder {
    fn default() -> Self {
        Self::new()
    }
}