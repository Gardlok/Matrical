/// A typed stage marker for transformation provenance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TagStage {
    /// Data entering a documented workflow.
    Input,
    /// A transformation step.
    Transform,
    /// Data leaving a documented workflow.
    Output,
    /// A caller-defined review or verification step.
    Review,
}

/// Bounded, non-executable metadata attached to an execution report.
///
/// Tags have a finite semantic namespace. Text carried by [`Tag::Source`] is an
/// inert source label: Matrical never interprets it as code, a query, a command,
/// or a Gear selector. Tags are not passed into Gear execution, so changing Tag
/// metadata cannot change transformation behavior.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Tag {
    /// Human-readable provenance source, such as an import or dataset label.
    Source(String),
    /// Typed transformation lifecycle stage.
    Stage(TagStage),
    /// Numeric sequence or batch identity supplied by the caller.
    Sequence(u64),
}

impl Tag {
    /// Creates an inert source-label Tag.
    pub fn source(source: impl Into<String>) -> Self {
        Self::Source(source.into())
    }

    /// Creates a typed lifecycle-stage Tag.
    pub const fn stage(stage: TagStage) -> Self {
        Self::Stage(stage)
    }

    /// Creates a numeric sequence/batch Tag.
    pub const fn sequence(sequence: u64) -> Self {
        Self::Sequence(sequence)
    }
}

#[cfg(test)]
mod tests {
    use super::{Tag, TagStage};

    #[test]
    fn tags_are_typed_comparable_metadata() {
        let tags = vec![
            Tag::source("fixture"),
            Tag::stage(TagStage::Transform),
            Tag::sequence(7),
        ];

        assert_eq!(tags.clone(), tags);
        assert_eq!(tags[1], Tag::Stage(TagStage::Transform));
    }
}
