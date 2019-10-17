// Copyright (C) 2019 - Will Glozer. All rights reserved.

use std::ffi::CStr;
use std::fs;
use std::mem::zeroed;
use std::str::from_utf8;
use crate::Version;

#[cfg(not(target_os = "linux"))]
pub fn version() -> Option<Version> {
    Version::parse(&release()?)
}

#[cfg(target_os = "linux")]
pub fn version() -> Option<Version> {
    ubuntu(read("/proc/version_signature")).or_else(|| {
        debian(read("/proc/version")).or_else(|| {
            Version::parse(&release()?)
        })
    })
}

pub(crate) fn release() -> Option<String> {
    Some(unsafe {
        let mut buf = zeroed();
        match libc::uname(&mut buf) {
            0 => CStr::from_ptr(buf.release[..].as_ptr()),
            _ => return None,
        }.to_string_lossy().to_string()
    })
}

#[allow(dead_code)]
pub(crate) fn debian(line: Option<String>) -> Option<Version> {
    let line   = line?;
    let marker = "Debian ";
    let index  = line.rfind(marker)? + marker.len();
    Version::parse(&line[index..])
}

#[allow(dead_code)]
pub(crate) fn ubuntu(line: Option<String>) -> Option<Version> {
    let (name, version) = triple(&line?)?;
    match name.as_str() {
        "Ubuntu" => Some(version),
        _        => None,
    }
}

fn triple(line: &str) -> Option<(String, Version)> {
    let mut split = line.splitn(3, char::is_whitespace);
    let name      = split.next()?;
    let _         = split.next()?;
    let upstream  = split.next()?;
    let version   = Version::parse(&upstream)?;
    Some((name.to_string(), version))
}

#[allow(dead_code)]
fn read(path: &str) -> Option<String> {
    match from_utf8(&fs::read(path).ok()?) {
        Ok(line) => Some(line.to_string()),
        Err(_)   => None,
    }
}
