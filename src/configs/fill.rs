use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct FillConfig<'a> {
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
}

impl Default for FillConfig<'_> {
    fn default() -> Self {
        Self {
            style: "bold black",
            symbol: ".",
            disabled: false,
        }
    }
}
