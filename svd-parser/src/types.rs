//! Shared primitive types for use in SVD objects.
#![allow(clippy::manual_strip)]

use roxmltree::Node;

use super::{Config, ElementExt, Parse, SVDError, SVDErrorAt};

impl Parse for u32 {
    type Object = u32;
    type Error = SVDErrorAt;
    type Config = ();

    fn parse(tree: &Node, _config: &Self::Config) -> Result<u32, Self::Error> {
        let text = tree.get_text()?;

        (if text.starts_with("0x") || text.starts_with("0X") {
            u32::from_str_radix(&text["0x".len()..], 16)
        } else if text.starts_with('#') {
            // Handle strings in the binary form of:
            // #01101x1
            // along with don't care character x (replaced with 0)
            u32::from_str_radix(
                &str::replace(&text.to_lowercase()["#".len()..], "x", "0"),
                2,
            )
        } else if text.starts_with("0b") {
            // Handle strings in the binary form of:
            // 0b01101x1
            // along with don't care character x (replaced with 0)
            u32::from_str_radix(&str::replace(&text["0b".len()..], "x", "0"), 2)
        } else {
            text.parse::<u32>()
        })
        .map_err(|e| SVDError::from(e).at(tree.id()))
    }
}

impl Parse for u64 {
    type Object = u64;
    type Error = SVDErrorAt;
    type Config = ();

    fn parse(tree: &Node, _config: &Self::Config) -> Result<u64, Self::Error> {
        let text = tree.get_text()?;

        (if text.starts_with("0x") || text.starts_with("0X") {
            u64::from_str_radix(&text["0x".len()..], 16)
        } else if text.starts_with('#') {
            // Handle strings in the binary form of:
            // #01101x1
            // along with don't care character x (replaced with 0)
            u64::from_str_radix(
                &str::replace(&text.to_lowercase()["#".len()..], "x", "0"),
                2,
            )
        } else if text.starts_with("0b") {
            // Handle strings in the binary form of:
            // 0b01101x1
            // along with don't care character x (replaced with 0)
            u64::from_str_radix(&str::replace(&text["0b".len()..], "x", "0"), 2)
        } else {
            text.parse::<u64>()
        })
        .map_err(|e| SVDError::from(e).at(tree.id()))
    }
}

pub struct BoolParse;

impl Parse for BoolParse {
    type Object = bool;
    type Error = SVDErrorAt;
    type Config = ();

    fn parse(tree: &Node, _config: &Self::Config) -> Result<bool, Self::Error> {
        let text = tree.get_text()?;
        match text {
            "0" => Ok(false),
            "1" => Ok(true),
            _ => match text.parse() {
                Ok(b) => Ok(b),
                Err(e) => Err(SVDError::InvalidBooleanValue(text.into(), e).at(tree.id())),
            },
        }
    }
}

pub struct DimIndex;

impl Parse for DimIndex {
    type Object = Vec<String>;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, _config: &Self::Config) -> Result<Vec<String>, Self::Error> {
        let text = tree.get_text()?;
        if text.contains('-') {
            let mut parts = text.splitn(2, '-');
            let start = parts
                .next()
                .ok_or_else(|| SVDError::DimIndexParse.at(tree.id()))?
                .parse::<u32>()
                .map_err(|e| SVDError::from(e).at(tree.id()))?;
            let end = parts
                .next()
                .ok_or_else(|| SVDError::DimIndexParse.at(tree.id()))?
                .parse::<u32>()
                .map_err(|e| SVDError::from(e).at(tree.id()))?;

            Ok((start..=end).map(|i| i.to_string()).collect())
        } else {
            Ok(text.split(',').map(|s| s.to_string()).collect())
        }
    }
}
