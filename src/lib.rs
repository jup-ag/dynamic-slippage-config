use range::Range;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Deserialize;
use serde_json::{self, error::Result};
use solana_program::pubkey::Pubkey;

mod field_as_string;
mod range;

const DYNAMIC_SLIPPAGE_CONFIG_JSON: &str = include_str!("../dynamic_slippage_config.json");

pub struct Category {
    pub name: String,
    pub range: Range,
    /// This range applies when the 2 traded mints belong to that category
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

fn default_amplification_ratio() -> Decimal {
    dec!(1.5)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Default {
    #[serde(default = "default_amplification_ratio")]
    pub amplification_ratio: Decimal,
    pub range: Range,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DynamicSlippageConfigOriginal {
    categories: Vec<CategoryOriginal>,
    default: Default,
}

pub struct DynamicSlippageConfig {
    pub categories: Vec<Category>,
    pub default: Default,
}

fn deserialize_dynamic_slippage_config(
    dynamic_slippage_config: &str,
) -> Result<DynamicSlippageConfig> {
    let DynamicSlippageConfigOriginal {
        categories,
        default,
    } = serde_json::from_str::<DynamicSlippageConfigOriginal>(dynamic_slippage_config)?;
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

    Ok(DynamicSlippageConfig {
        categories,
        default,
    })
}

#[allow(dead_code)]
fn deserialize_embedded_dynamic_slippage_config() -> Result<DynamicSlippageConfig> {
    deserialize_dynamic_slippage_config(DYNAMIC_SLIPPAGE_CONFIG_JSON)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_deserialize_dynamic_slippage_config() {
        let DynamicSlippageConfig { categories, .. } =
            deserialize_embedded_dynamic_slippage_config().unwrap();

        assert_eq!(
            categories.iter().map(|c| &c.name).collect::<Vec<_>>(),
            vec!["stable", "lst", "bluechip", "verified"]
        );
        assert!(categories
            .iter()
            .find(|c| c.name == "lst")
            .unwrap()
            .pair_range
            .is_some());

        // Ensure no duplicated mints, otherwise the config behaviour will be confusing
        let duplicates = categories
            .iter()
            .flat_map(|c| &c.mints)
            .duplicates()
            .collect::<Vec<&Pubkey>>();

        assert_eq!(duplicates, Vec::<&Pubkey>::new());
    }
}
