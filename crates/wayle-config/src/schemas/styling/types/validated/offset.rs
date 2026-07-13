//! Offset value allowing negative numbers.

use std::{
    fmt::{self, Display, Formatter},
    ops::Deref,
};

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};

/// Offset value allowing negative numbers (clamped to [-500.0, 500.0]).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
#[serde(transparent)]
#[schemars(transparent)]
pub struct Offset(#[schemars(range(min = -500.0, max = 500.0))] f32);

impl Offset {
    /// Creates an offset value, clamping to [-500.0, 500.0].
    #[must_use]
    pub fn new(value: f32) -> Self {
        Self(value.clamp(-500.0, 500.0))
    }

    /// The raw `f32`.
    #[must_use]
    pub fn value(self) -> f32 {
        self.0
    }
}

impl Default for Offset {
    fn default() -> Self {
        Self(0.0)
    }
}

impl Deref for Offset {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<f32> for Offset {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl<'de> Deserialize<'de> for Offset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = f32::deserialize(deserializer)?;
        Ok(Self::new(raw))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_values() {
        assert_eq!(Offset::new(-1000.0).value(), -500.0);
        assert_eq!(Offset::new(1000.0).value(), 500.0);
    }

    #[test]
    fn preserves_in_range() {
        assert_eq!(Offset::new(0.0).value(), 0.0);
        assert_eq!(Offset::new(-10.5).value(), -10.5);
        assert_eq!(Offset::new(100.0).value(), 100.0);
    }
}
