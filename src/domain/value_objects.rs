use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn parse(value: &str) -> Result<Self, &'static str> {
        let normalized = value.trim().to_ascii_lowercase();
        let at = normalized.find('@');
        let dot = normalized.rfind('.');
        if normalized.len() > 254 || !matches!((at, dot), (Some(at_idx), Some(dot_idx)) if at_idx > 0 && dot_idx > at_idx + 1 && dot_idx < normalized.len() - 1) {
            return Err("invalid email");
        }
        Ok(Self(normalized))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayName(String);

impl DisplayName {
    pub fn parse(value: &str) -> Result<Self, &'static str> {
        let normalized = value.trim().to_owned();
        if normalized.is_empty() || normalized.len() > 255 {
            return Err("invalid name");
        }
        Ok(Self(normalized))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub fn validate_password_policy(password: &str) -> Result<(), &'static str> {
    let has_upper = password.chars().any(|ch| ch.is_ascii_uppercase());
    let has_lower = password.chars().any(|ch| ch.is_ascii_lowercase());
    let has_digit = password.chars().any(|ch| ch.is_ascii_digit());
    let has_symbol = password.chars().any(|ch| !ch.is_ascii_alphanumeric());
    if password.len() < 8 || !has_upper || !has_lower || !has_digit || !has_symbol {
        return Err("password does not meet policy");
    }
    Ok(())
}
