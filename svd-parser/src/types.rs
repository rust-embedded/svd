//! Shared primitive types for use in SVD objects.
#![allow(clippy::manual_strip)]

use roxmltree::Node as Element;

use super::elementext::ElementExt;
use super::{Context, Parse, Result, SVDError};

macro_rules! unwrap {
    ($e:expr) => {
        $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
    };
}

impl Parse for u32 {
    type Object = u32;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<u32> {
        let text = tree.get_text()?;

        if text.starts_with("0x") || text.starts_with("0X") {
            u32::from_str_radix(&text["0x".len()..], 16)
                .with_context(|| format!("{} invalid", text))
        } else if text.starts_with('#') {
            // Handle strings in the binary form of:
            // #01101x1
            // along with don't care character x (replaced with 0)
            u32::from_str_radix(
                &str::replace(&text.to_lowercase()["#".len()..], "x", "0"),
                2,
            )
            .with_context(|| format!("{} invalid", text))
        } else if text.starts_with("0b") {
            // Handle strings in the binary form of:
            // 0b01101x1
            // along with don't care character x (replaced with 0)
            u32::from_str_radix(&str::replace(&text["0b".len()..], "x", "0"), 2)
                .with_context(|| format!("{} invalid", text))
        } else {
            text.parse::<u32>()
                .with_context(|| format!("{} invalid", text))
        }
    }
}

impl Parse for u64 {
    type Object = u64;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<u64> {
        let text = tree.get_text()?;

        if text.starts_with("0x") || text.starts_with("0X") {
            u64::from_str_radix(&text["0x".len()..], 16)
                .with_context(|| format!("{} invalid", text))
        } else if text.starts_with('#') {
            // Handle strings in the binary form of:
            // #01101x1
            // along with don't care character x (replaced with 0)
            u64::from_str_radix(
                &str::replace(&text.to_lowercase()["#".len()..], "x", "0"),
                2,
            )
            .with_context(|| format!("{} invalid", text))
        } else if text.starts_with("0b") {
            // Handle strings in the binary form of:
            // 0b01101x1
            // along with don't care character x (replaced with 0)
            u64::from_str_radix(&str::replace(&text["0b".len()..], "x", "0"), 2)
                .with_context(|| format!("{} invalid", text))
        } else {
            text.parse::<u64>()
                .with_context(|| format!("{} invalid", text))
        }
    }
}

pub struct BoolParse;

impl Parse for BoolParse {
    type Object = bool;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<bool> {
        let text = unwrap!(tree.text());
        Ok(match text {
            "0" => false,
            "1" => true,
            _ => match text.parse() {
                Ok(b) => b,
                Err(e) => {
                    return Err(
                        SVDError::InvalidBooleanValue(tree.id(), text.to_string(), e).into(),
                    )
                }
            },
        })
    }
}

pub struct DimIndex;

impl Parse for DimIndex {
    type Object = Vec<String>;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Vec<String>> {
        let text = tree.get_text()?;
        if text.contains('-') {
            let mut parts = text.splitn(2, '-');
            let start = unwrap!(unwrap!(parts.next()).parse::<u32>());
            let end = unwrap!(unwrap!(parts.next()).parse::<u32>()) + 1;

            Ok((start..end).map(|i| i.to_string()).collect())
        } else {
            Ok(text.split(',').map(|s| s.to_string()).collect())
        }
    }
}

//TODO: encode for DimIndex
