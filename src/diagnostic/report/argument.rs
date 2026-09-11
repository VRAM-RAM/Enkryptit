use crate::diagnostic::style::EnkryptitStyle;

pub struct ReportArgument {
    pub value: String,
    pub style: EnkryptitStyle,
}

impl ReportArgument {
    pub fn new(value: impl Into<String>, style: EnkryptitStyle) -> Self {
        Self { value: value.into(), style }
    }

    pub fn label(value: impl Into<String>) -> Self {
        Self { value: value.into(), style: EnkryptitStyle::Label }
    }

    pub fn warning(value: impl Into<String>) -> Self {
        Self { value: value.into(), style: EnkryptitStyle::Warning }
    }

    pub fn value(value: impl Into<String>) -> Self {
        Self { value: value.into(), style: EnkryptitStyle::Value }
    }

    pub fn title(value: impl Into<String>) -> Self {
        Self { value: value.into(), style: EnkryptitStyle::Accent }
    }

    pub fn value_in_kib(value: impl Into<String>) -> Self {
        Self { value: format!("{} KiB", value.into()), style: EnkryptitStyle::Value }
    }

    pub fn accent(value: impl Into<String>) -> Self {
        Self { value: value.into(), style: EnkryptitStyle::Accent }
    }
}