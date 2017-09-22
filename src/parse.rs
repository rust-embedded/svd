use error::*;

pub fn bool(t: &str) -> Result<bool> {
    Ok(t.parse().map_err(|_| format!("`{}` is not a `bool`", t))?)
}

pub fn u32(t: &str) -> Result<u32> {
    Ok(t.parse().map_err(|_| format!("`{}` is not a `u32`", t))?)
}
