use range::Range;
use serde::Deserialize;
use serde_json::{self, error::Result};
use solana_program::pubkey::Pubkey;

mod field_as_string;
mod range;

const TOKEN_CATEGORIES_JSON: &str = include_str!("../token_categories.json");
const SLIPPAGE_CONFIG_JSON: &str = include_str!("../slippage_config.json");

#[derive(Deserialize)]
pub struct Category {
    pub name: String,
    pub mints: Vec<DeserializablePubkey>,
}

#[derive(Deserialize)]
pub struct TokenCategories {
    pub categories: Vec<Category>,
    pub excluded_from_intermediate_hop_mints: Vec<DeserializablePubkey>,
}

#[derive(Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeserializablePubkey(#[serde(with = "field_as_string")] pub Pubkey);

#[derive(Deserialize)]
pub struct SlippageConfig {
    pub slippage_base_tolerance: Vec<SlippageRange>,
}

#[derive(Deserialize)]
pub struct SlippageRange {
    pub name: String,
    pub range: Range,
}

#[allow(dead_code)]
fn deserialize_token_categories() -> Result<TokenCategories> {
    serde_json::from_str::<TokenCategories>(TOKEN_CATEGORIES_JSON)
}

#[allow(dead_code)]
fn deserialize_slippage_config() -> Result<SlippageConfig> {
    serde_json::from_str::<SlippageConfig>(SLIPPAGE_CONFIG_JSON)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_deserialize_token_categories_config() {
        let TokenCategories { categories, .. } = deserialize_token_categories().unwrap();

        assert_eq!(
            categories.iter().map(|c| &c.name).collect::<Vec<_>>(),
            vec!["solana", "stable", "lst", "bluechip"]
        );

        // Ensure no duplicated mints, otherwise the config behaviour will be confusing
        let duplicates = categories
            .iter()
            .flat_map(|c| &c.mints)
            .duplicates()
            .collect::<Vec<&DeserializablePubkey>>();

        assert_eq!(duplicates, Vec::<&DeserializablePubkey>::new());
    }

    #[test]
    fn test_deserialize_slippage_config() {
        let slippage_config = deserialize_slippage_config().unwrap();

        assert_eq!(
            slippage_config
                .slippage_base_tolerance
                .iter()
                .map(|c| &c.name)
                .collect::<Vec<_>>(),
            vec![
                "solana",
                "stable",
                "lst",
                "bluechip",
                "default",
                "pump_new_graduate",
                "degen",
                "new_token",
                "pump_new_graduate_first_hour",
                "degen_buy",
                "degen_sell"
            ]
        );
    }
}
