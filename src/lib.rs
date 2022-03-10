#![doc = include_str!("../README.md")]

#[cfg(feature = "ufmt")]
#[doc(hidden)]
pub mod ufmt;

use std::{fmt, net::Ipv4Addr, str};

const MAX_LEN: usize = 3 * 4 + 3;
type Buf = [u8; MAX_LEN];

macro_rules! write_octet {
    ($octet:expr, $buf:ident, $at:ident) => {
        let octet = $octet;
        if octet >= 100 {
            $buf[$at] = b'0' + octet / 100;
            $at += 1;
        }
        if octet >= 10 {
            $buf[$at] = b'0' + (octet / 10) % 10;
            $at += 1;
        }
        $buf[$at] = b'0' + octet % 10;
        $at += 1;
    };
}

const fn ipv4_display(ip: Ipv4Addr) -> (Buf, usize) {
    let mut arr: Buf = [b'.'; MAX_LEN];
    let mut at = 0;

    let [a, b, c, d] = ip.octets();
    write_octet!(a, arr, at);
    at += 1;
    write_octet!(b, arr, at);
    at += 1;
    write_octet!(c, arr, at);
    at += 1;
    write_octet!(d, arr, at);

    (arr, at)
}

#[repr(transparent)]
#[derive(Clone, Copy)]
/// A wrapper around [`Ipv4Addr`], providing an implementation of [`fmt::Display`]
/// that is more performant than the one provided by the Rust standard library.
///
/// Examples:
/// ```rust
/// use std::net::Ipv4Addr;
/// use ipv4_display::Ipv4AddrDisplay;
///
/// assert_eq!(Ipv4AddrDisplay::new(Ipv4Addr::UNSPECIFIED).to_string(), "0.0.0.0");
/// assert_eq!(Ipv4AddrDisplay::new(Ipv4Addr::LOCALHOST).to_string(), "127.0.0.1");
/// assert_eq!(Ipv4AddrDisplay::new(Ipv4Addr::new(231, 2, 30, 102)).to_string(), "231.2.30.102");
/// ```
pub struct Ipv4AddrDisplay(Ipv4Addr);

impl Ipv4AddrDisplay {
    #[inline]
    pub fn new(addr: Ipv4Addr) -> Self {
        Self(addr)
    }
}

impl From<Ipv4Addr> for Ipv4AddrDisplay {
    #[inline]
    fn from(addr: Ipv4Addr) -> Self {
        Self(addr)
    }
}

impl fmt::Display for Ipv4AddrDisplay {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (arr, len) = ipv4_display(self.0);
        f.write_str(unsafe { str::from_utf8_unchecked(&arr[..len]) })
    }
}

impl fmt::Debug for Ipv4AddrDisplay {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
