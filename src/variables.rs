//! Contains structures and functions to work via the environment
//! variables that will parsed by this binary.

use ::serde_valid::Validate;

/// TODO
#[derive(Debug, ::serde::Deserialize)]
pub struct Categories(Vec<Category>);

impl Categories {
    /// TODO
    pub fn perform_self_check(&self) -> ::anyhow::Result<()> {
        for category in &self.0 {
            category.variables.perform_self_check()?;
        }
        Ok(())
    }

    /// TODO
    pub fn write_as_markdown(&self, path: impl AsRef<std::path::Path>) -> ::anyhow::Result<()> {
        let mut markdown = String::from("# Environment Variables\n");

        for category in &self.0 {
            markdown.push_str(category.write_as_markdown().as_str());
        }

        std::fs::write(path, markdown)?;
        Ok(())
    }

    pub fn write_as_shell(&self, path: impl AsRef<std::path::Path>) -> ::anyhow::Result<()> {
        let mut content = String::from("#!/bin/bash");

        for category in &self.0 {
			content.push_str("\n\n");
            content.push_str(category.write_as_shell().as_str());
        }

		content.push('\n');
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// TODO
#[derive(Debug, ::serde::Deserialize)]
pub struct Category {
    /// TODO
    category: String,
    /// TODO
    variables: Variables,
}

impl Category {
    /// TODO
    pub fn write_as_markdown(&self) -> String {
        let mut markdown = format!("##  {}", self.category);
        markdown.push_str(self.variables.write_as_markdown().as_str());
        markdown
    }

    /// TODO
    pub fn write_as_shell(&self) -> String {
        let mut content = format!(
            "function __environment_variables__automated__{}() {{",
            self.category.to_lowercase()
        );
        content.push_str(self.variables.write_as_shell().as_str());
        content.push_str("\n}");
        content
    }
}

/// TODO
#[derive(Debug, ::serde::Deserialize)]
pub struct Variables(Vec<Variable>);

impl Variables {
    /// TODO
    pub fn perform_self_check(&self) -> ::anyhow::Result<()> {
        for variable in &self.0 {
            variable.validate()?;
            variable.additional_self_check()?;
        }

        Ok(())
    }

    /// TODO
    pub fn write_as_markdown(&self) -> String {
        let mut markdown = String::new();
        for variable in &self.0 {
            markdown.push_str(variable.to_markdown().as_str());
            markdown.push('\n');
        }

        markdown
    }

    /// TODO
    pub fn write_as_shell(&self) -> String {
        let mut markdown = String::new();
        for variable in &self.0 {
            markdown.push('\n');
            markdown.push_str(variable.to_shell().as_str());
        }

        markdown
    }
}

/// The full description of an environment variable.
#[derive(Debug, ::serde::Deserialize, ::serde_valid::Validate)]
pub struct Variable {
    /// The name of the variable.
    #[validate(min_length = 1)]
    name: String,
    /// What the variable is used for.
    #[validate(min_length = 1)]
    description: String,
    /// What is the type of variable
    r#type: Type,
    /// The allowed values this variable can have.
    #[validate(unique_items)]
    values: Option<Vec<ValueEntry>>,
    /// The default value in case the variable is unset.
    default: Option<Value>,
    /// Meta-information about a variable
    #[serde(default)]
    meta: MetaInformation,
}

impl Variable {
    /// TODO
    fn additional_self_check(&self) -> ::anyhow::Result<()> {
        self.name_is_valid()?;
        self.correct_type_of_values()?;
        self.values_are_unique()?;
        self.default_is_in_value()?;
        Ok(())
    }

    /// TODO
    fn name_is_valid(&self) -> ::anyhow::Result<()> {
        if self.name.contains(' ') {
            anyhow::bail!("Name of '{}' contains whitespaces", self.name);
        }

        for character in self.name.chars() {
            if character == '_' {
                continue;
            }

            if !character.is_uppercase() {
                anyhow::bail!("Name of '{}' contains lowercase letters", self.name);
            } else if !character.is_ascii_alphabetic() {
                anyhow::bail!(
                    "Name of '{}' contains non ASCII-alphabetic letters",
                    self.name
                );
            }
        }

        Ok(())
    }

    /// TODO
    fn correct_type_of_values(&self) -> ::anyhow::Result<()> {
        if let Some(values) = &self.values {
            for (index, value_entry) in values.iter().enumerate() {
                let value = &value_entry.value;

                let error_message = format!(
                    "Value '{}' in {}.values[{}].value was expected to",
                    value, self.name, index
                );
                match self.r#type {
                    Type::Number => {
                        if let Value::Number(_) = value {
                            continue;
                        } else {
                            anyhow::bail!("{error_message} be a number");
                        }
                    }
                    Type::NonZeroNumber => {
                        if let Value::Number(value) = value {
                            if *value > 0 {
                                continue;
                            } else {
                                anyhow::bail!("{error_message} be bigger than '0'");
                            }
                        } else {
                            anyhow::bail!("{error_message} be a number");
                        }
                    }
                    Type::NumberAsBool => {
                        if let Value::Number(value) = value {
                            if *value == 0 || *value == 1 {
                                continue;
                            } else {
                                anyhow::bail!("{error_message} be '0' or '1'",);
                            }
                        } else {
                            anyhow::bail!("{error_message} be a number");
                        }
                    }
                    Type::String => {
                        if let Value::String(_) = value {
                            continue;
                        } else {
                            anyhow::bail!("{error_message} be a string");
                        }
                    }
                    Type::NonEmptyString => {
                        if let Value::String(value) = value {
                            if value.is_empty() {
                                anyhow::bail!("{error_message} be non-empty string");
                            } else {
                                continue;
                            }
                        } else {
                            anyhow::bail!("{error_message} be a string");
                        }
                    }
                    Type::List => {
                        if let Value::List(_) = value {
                            continue;
                        } else {
                            anyhow::bail!("{error_message} be a list");
                        }
                    }
                    Type::NonEmptyList => {
                        if let Value::List(value) = value {
                            if value.is_empty() {
                                anyhow::bail!("{error_message} be a non-empty list");
                            } else {
                                continue;
                            }
                        } else {
                            anyhow::bail!("{error_message} be a list");
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// TODO
    fn values_are_unique(&self) -> ::anyhow::Result<()> {
        if let Some(values) = &self.values {
            let mut uniq = std::collections::HashSet::new();
            if !values.into_iter().all(move |x| uniq.insert(&x.value)) {
                anyhow::bail!("Values of '{}' are not unique", self.name);
            }
        }

        Ok(())
    }

    /// TODO
    fn default_is_in_value(&self) -> ::anyhow::Result<()> {
        if let Some(default_value) = &self.default {
            if let Some(values) = &self.values {
                if !values
                    .iter()
                    .any(|value_entry| value_entry.value == *default_value)
                {
                    anyhow::bail!(
                        "Default value '{}' is not contained in {}.values",
                        default_value,
                        self.name
                    )
                }
            }
        }

        Ok(())
    }

    /// TODO
    fn to_markdown(&self) -> String {
        let mut markdown = format!(
            "\n### {}\n\n{}\n\n|Property|Value|\n|:-- |:-- |\n|Type|{}|\n|Default|{}|\n|Values|",
            self.name,
            self.description,
            self.r#type,
            if let Some(default) = &self.default {
                default.to_string()
            } else {
                String::from("This variable does not have a default value")
            }
        );

        if let Some(values) = &self.values {
            markdown.push_str("<ul>");
            for value_entry in values {
                markdown.push_str("<li>");
                markdown.push_str(value_entry.to_string().as_str());
                markdown.push_str("</li>");
            }
            markdown.push_str("</ul>");
        } else {
            markdown.push_str("Arbitrary values within the restriction of the type are allowed");
        }
        markdown.push_str("|");

        if self.meta != MetaInformation::InUse {
            markdown.push_str(format!("\n|Meta|{}", self.meta).as_str());
        }

        markdown
    }

    fn to_shell(&self) -> String {
        format!(
            "  # {}\n  VARS[{}]=\"${{{}:={}}}\"",
			self.description.replace("\n", "\n  # "),
            self.name,
            self.name,
            if let Some(default) = &self.default {
                default.to_string()
            } else {
                String::new()
            }
        )
    }
}

/// The type of an environment variable
#[derive(Debug, ::serde::Deserialize)]
pub enum Type {
    /// TODO
    Number,
    /// TODO
    NonZeroNumber,
    /// TODO
    NumberAsBool,
    /// TODO
    String,
    /// TODO
    NonEmptyString,
    /// TODO
    List,
    /// TODO
    NonEmptyList,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number | Self::String | Self::List => {
                write!(f, "{:?}", self)
            }
            Self::NonZeroNumber => write!(f, "Non-Zero Number"),
            Self::NumberAsBool => write!(f, "Number as Boolean"),
            Self::NonEmptyString => write!(f, "Non-Empty String"),
            Self::NonEmptyList => write!(f, "Non-Empty List"),
        }
    }
}

/// TODO
#[derive(Debug, PartialEq, Eq, Hash, ::serde::Deserialize)]
pub struct ValueEntry {
    /// An environment variable's possible value
    value: Value,
    /// An associated description for a given value
    description: String,
}

impl std::fmt::Display for ValueEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}` ⇒ {}", self.value, self.description)
    }
}

/// Describes a value a variables can have. It contains the value and a description.
#[derive(Debug, PartialEq, Eq, Hash, ::serde::Deserialize)]
#[serde(untagged)]
pub enum Value {
    /// TODO
    Number(isize),
    /// TODO
    String(String),
    /// TODO
    List(Vec<String>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(number) => {
                write!(f, "{number}")
            }
            Self::String(string) => {
                write!(f, "{string}")
            }
            Self::List(list) => {
                write!(f, "{list:?}")
            }
        }
    }
}

/// The state an environment variable can be in.
///
/// This is used when a variable has become deprecated or
/// when it has been removed.
#[derive(Debug, PartialEq, ::serde::Deserialize)]
pub enum MetaInformation {
    /// This variable is actively in use
    InUse,
    /// This variable should not be used
    UsageDiscouraged,
    /// The variable is deprecate and will be removed in a future version.
    Deprecated(String), // TODO maybe use semantic versioning crate
    /// The variable was deprecated and is now removed.
    Removed(String),
}

impl Default for MetaInformation {
    fn default() -> Self {
        Self::InUse
    }
}

impl std::fmt::Display for MetaInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InUse => write!(f, "in use"),
            Self::UsageDiscouraged => write!(f, "**Usage is discouraged**"),
            Self::Deprecated(version) => write!(f, "**Deprecated** since version {}", version),
            Self::Removed(version) => write!(f, "**Removed** in version {}", version),
        }
    }
}
