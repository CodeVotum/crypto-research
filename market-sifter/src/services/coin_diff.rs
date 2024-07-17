use std::collections::HashMap;

use serde::Serialize;

use crate::models::CoinInfo;

#[derive(Debug, Serialize)]
struct Change {
    field: String,
    old_value: String,
    new_value: String,
}

#[derive(Debug, Serialize)]
struct DetailedChange {
    symbol: String,
    changes: Vec<Change>,
}

#[derive(Debug, Serialize)]
pub struct ChangeSummary {
    appeared: Vec<CoinInfo>,
    disappeared: Vec<CoinInfo>,
    detailed_changes: Vec<DetailedChange>,
}

macro_rules! compare_fields {
    ($old:expr, $new:expr, $changes:expr, $( $field:ident ),* ) => {
        $(
            if $old.$field != $new.$field {
                $changes.push(Change {
                    field: stringify!($field).to_string(),
                    old_value: $old.$field.to_string(),
                    new_value: $new.$field.to_string(),
                });
            }
        )*
    };
}

pub fn compare_coin_lists(old_list: Vec<CoinInfo>, new_list: Vec<CoinInfo>) -> ChangeSummary {
    let old_map: HashMap<String, &CoinInfo> =
        old_list.iter().map(|c| (c.symbol.clone(), c)).collect();
    let new_map: HashMap<String, &CoinInfo> =
        new_list.iter().map(|c| (c.symbol.clone(), c)).collect();

    let mut appeared = Vec::new();
    let mut disappeared = Vec::new();
    let mut detailed_changes = Vec::new();

    for new_coin in new_list.iter() {
        if let Some(old_coin) = old_map.get(&new_coin.symbol) {
            let mut changes = Vec::new();
            compare_fields!(
                old_coin,
                new_coin,
                changes,
                num_categories,
                market_cap_rank,
                categories
            );
            if !changes.is_empty() {
                detailed_changes.push(DetailedChange {
                    symbol: new_coin.symbol.clone(),
                    changes,
                });
            }
        } else {
            appeared.push(new_coin.clone());
        }
    }

    for old_coin in old_list {
        if !new_map.contains_key(&old_coin.symbol) {
            disappeared.push(old_coin);
        }
    }

    ChangeSummary {
        appeared,
        disappeared,
        detailed_changes,
    }
}
