// Copyright (C) 2019 - Will Glozer. All rights reserved.

use crate::{Version, kernel};

#[test]
fn parse_version() {
    let tests = vec![
        ("4.15.18",               Some((4, 15, 18))),
        ("4.15.0-46-generic",     Some((4, 15,  0))),
        ("2.6.35-6.9-generic",    Some((2,  6, 35))),
        ("2.6.35-rc3",            Some((2,  6, 35))),
        ("5.0.9-301.fc30.x86_64", Some((5,  0,  9))),
        ("4.19.0-6-amd64",        Some((4, 19,  0))),
    ];

    for (str, expect) in tests {
        let actual = Version::parse(str);
        let expect = expect.map(|(x, y, z)| Version::new(x, y, z));
        assert_eq!(expect, actual);
    }
}

#[test]
fn parse_debian() {
    let tests = vec![
        ("Linux version 4.19.0-6-amd64 (debian-kernel@lists.debian.org) (gcc version 8.3.0 (Debian 8.3.0-6)) #1 SMP Debian 4.19.67-2 (2019-08-28)", Some((4, 19, 67))),
    ];

    for (str, expect) in tests {
        let actual = kernel::debian(Some(str.to_string()));
        let expect = expect.map(|(x, y, z)| Version::new(x, y, z));
        assert_eq!(expect, actual);
    }
}

#[test]
fn parse_ubuntu() {
    let tests = vec![
        ("Ubuntu 4.15.0-46.49-generic 4.15.18", Some((4, 15, 18))),
    ];

    for (str, expect) in tests {
        let actual = kernel::ubuntu(Some(str.to_string()));
        let expect = expect.map(|(x, y, z)| Version::new(x, y, z));
        assert_eq!(expect, actual);
    }
}
