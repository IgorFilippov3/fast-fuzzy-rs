//! Node.js bindings for string normalization and fuzzy search.
//!
//! This crate exposes a small, fast fuzzy-search API to JavaScript via `napi`.
//! Public functions are annotated with `#[napi]` and documented for both Rust
//! and JS/TS consumers.
//!
//! ## JavaScript usage (TypeScript signatures)
//! ```ts
//! // search(query, items, options?)
//! declare function search(
//!   query: string,
//!   items: string[],
//!   options?: {
//!     limit?: number;        // default: 10
//!     threshold?: number;    // default: 0.0
//!     normalize?: boolean;   // default: true
//!     ignoreCase?: boolean;  // default: true
//!   }
//! ): Array<{ item: string; score: number; index: number }>
//!
//! // fuzzy(a, b, normalize?)
//! declare function fuzzy(a: string, b: string, normalize?: boolean): number
//! ```
//!
//! ## Notes
//! - When `normalize` is enabled, strings are Unicode-normalized and diacritics
//!   are removed; when `ignoreCase` is enabled, comparison is case-insensitive.
//! - Scores are in `0.0..=1.0` (higher is better). `threshold` filters out
//!   results below the given score. `limit` truncates the final sorted list.

use napi::bindgen_prelude::*;
use napi_derive::napi;

mod algo;
mod normalization;
mod search_options;
mod search_result;

pub use algo::levenshtein_distance;
pub use normalization::normalize_string;
pub use search_options::SearchOptions;
pub use search_result::SearchResult;

#[napi]
pub fn search(
    query: String,
    items: Vec<String>,
    options: Option<SearchOptions>,
) -> Result<Vec<SearchResult>> {
    let opts = options.unwrap_or_default();

    let normalized_query = if opts.normalize.unwrap_or(true) {
        normalize_string(&query, opts.ignore_case.unwrap_or(true))
    } else if opts.ignore_case.unwrap_or(true) {
        query.to_lowercase()
    } else {
        query
    };

    let mut results: Vec<SearchResult> = items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| {
            let normalized_item = if opts.normalize.unwrap_or(true) {
                normalize_string(item, opts.ignore_case.unwrap_or(true))
            } else if opts.ignore_case.unwrap_or(true) {
                item.to_lowercase()
            } else {
                item.clone()
            };

            let score =
                calculate_similarity(&normalized_query, &normalized_item);

            if score >= opts.threshold.unwrap_or(0.0) {
                Some(SearchResult {
                    item: item.clone(),
                    score,
                    index: index as u32,
                })
            } else {
                None
            }
        })
        .collect();

    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    if let Some(limit) = opts.limit {
        results.truncate(limit as usize);
    }

    Ok(results)
}

#[napi]
pub fn fuzzy(
    str1: String,
    str2: String,
    normalize: Option<bool>,
) -> Result<f64> {
    let should_normalize = normalize.unwrap_or(true);

    let s1 = if should_normalize {
        normalize_string(&str1, true)
    } else {
        str1
    };

    let s2 = if should_normalize {
        normalize_string(&str2, true)
    } else {
        str2
    };

    Ok(calculate_similarity(&s1, &s2))
}

fn calculate_similarity(str1: &str, str2: &str) -> f64 {
    if str1 == str2 {
        return 1.0;
    }

    if str1.is_empty() || str2.is_empty() {
        return 0.0;
    }

    let distance = levenshtein_distance(str1, str2);
    let max_len = str1.len().max(str2.len()) as f64;

    1.0 - (distance as f64 / max_len)
}
