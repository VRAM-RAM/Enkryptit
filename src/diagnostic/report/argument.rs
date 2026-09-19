use crate::diagnostic::report::style::EnkryptitStyle;

/// An argument for a [`Report`]. Defined by :
/// - Its value 
/// - Its [`EnkryptitStyle`]
/// 
/// The style has an importance when displaying the [`ReportArgument`] while displaying a [`Report`].
pub struct ReportArgument {
    pub value: String,
    pub style: EnkryptitStyle,
}

impl ReportArgument {
    /// Returns a [`ReportArgument`] of the given [`EnkryptitStyle`] and value.
    pub fn new(value: impl Into<String>, style: EnkryptitStyle) -> Self {
        Self { value: value.into(), style }
    }

    /// Returns a [`ReportArgument`] of the given value and of style [`EnkryptitStyle::Label`] 
    pub fn label(value: impl Into<String>) -> Self {
        Self { value: value.into(), style: EnkryptitStyle::Label }
    }

    #[allow(dead_code)]
    /// Returns a [`ReportArgument`] of the given value and of style [`EnkryptitStyle::Warning`] 
    pub fn warning(value: impl Into<String>) -> Self {
        Self { value: value.into(), style: EnkryptitStyle::Warning }
    }

    /// Returns a [`ReportArgument`] of the given value and of style [`EnkryptitStyle::Value`] 
    pub fn value(value: impl Into<String>) -> Self {
        Self { value: value.into(), style: EnkryptitStyle::Value }
    }

    /// Returns a [`ReportArgument`] of the given value and of style [`EnkryptitStyle::Accent`] 
    pub fn title(value: impl Into<String>) -> Self {
        Self { value: value.into(), style: EnkryptitStyle::Accent }
    }

    /// Returns a [`ReportArgument`] of the given value and of style [`EnkryptitStyle::Value`], formatted with a ` KiB` suffix.
    pub fn value_in_kib(value: impl Into<String>) -> Self {
        Self { value: format!("{} KiB", value.into()), style: EnkryptitStyle::Value }
    }

    /// Returns a [`ReportArgument`] of the given value and of style [`EnkryptitStyle::Accent`] 
    pub fn accent(value: impl Into<String>) -> Self {
        Self { value: value.into(), style: EnkryptitStyle::Accent }
    }
}