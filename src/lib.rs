use serde::Deserialize;
use serde_json::{self, error::Result};
use solana_program::pubkey::Pubkey;

mod field_as_string;

#[derive(Copy, Clone, Deserialize, PartialEq, Debug, Default)]
pub struct Range {
    pub min: u16,
    pub max: u16,
}
pub struct Category {
    pub name: String,
    pub range: Range,
    pub pair_range: Option<Range>,
    pub mints: Vec<Pubkey>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CategoryOriginal {
    name: String,
    range: Range,
    pair_range: Option<Range>,
    mints: Vec<DeserializablePubkey>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeserializablePubkey(#[serde(with = "field_as_string")] pub Pubkey);

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultSlippage {
    pub range: Range,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DynamicSlippageCategories {
    categories: Vec<CategoryOriginal>,
    default: DefaultSlippage,
}

const DYNAMIC_SLIPPAGE_CONFIG_JSON: &str = include_str!("../dynamic_slippage_config.json");

pub fn deserialize_dynamic_slippage_config() -> Result<(DefaultSlippage, Vec<Category>)> {
    let DynamicSlippageCategories {
        categories,
        default,
    } = serde_json::from_str::<DynamicSlippageCategories>(DYNAMIC_SLIPPAGE_CONFIG_JSON)?;
    let categories = categories
        .into_iter()
        .map(
            |CategoryOriginal {
                 name,
                 range,
                 pair_range,
                 mints,
             }| Category {
                name,
                range,
                pair_range,
                mints: mints.into_iter().map(|m| m.0).collect(),
            },
        )
        .collect();

    Ok((default, categories))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_dynamic_slippage_config() {
        let (_, categories) = deserialize_dynamic_slippage_config().unwrap();

        assert_eq!(
            categories.iter().map(|c| &c.name).collect::<Vec<_>>(),
            vec!["stable", "lst", "bluechip", "verified"]
        );

        assert!(categories
            .iter()
            .find(|c| c.name == "lst")
            .unwrap()
            .pair_range
            .is_some())
    }
}
