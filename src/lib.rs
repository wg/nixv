// Copyright (C) 2019 - Will Glozer. All rights reserved.

use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self { major, minor, patch }
    }

    pub fn parse(str: &str) -> Option<Self> {
        let (major, str) = number(str)?;
        let (minor, str) = number(str)?;
        let (patch,   _) = number(str)?;
        Some(Self { major, minor, patch } )
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Version { major, minor, patch } = self;
        write!(f, "{}.{}.{}", major, minor, patch)
    }
}

fn number(str: &str) -> Option<(u64, &str)> {
    let mut split = str.splitn(2, |c: char| !c.is_numeric());
    let num = split.next()?;
    let str = split.next().unwrap_or("");
    Some((num.parse().ok()?, str))
}

pub mod kernel;

#[cfg(test)]
mod test;
