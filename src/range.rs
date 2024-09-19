use serde::Deserialize;

#[derive(Copy, Clone, Deserialize, PartialEq, Debug, Default)]
pub struct Range {
    pub min: u16,
    pub max: u16,
}
