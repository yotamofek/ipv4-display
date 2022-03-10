use ufmt::{uDebug, uDisplay, uwrite};

use crate::{Ipv4AddrDisplay, MAX_LEN};

impl uDisplay for Ipv4AddrDisplay {
    #[inline]
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        let octets = self.0.octets();

        uwrite!(f, "{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
    }
}

impl uDebug for Ipv4AddrDisplay {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        uDisplay::fmt(self, f)
    }
}

impl Ipv4AddrDisplay {
    pub fn to_string_ufmt(&self) -> String {
        let mut s = String::with_capacity(MAX_LEN);
        let _ = uwrite!(s, "{}", self);
        s
    }
}

#[test]
fn test_ufmt() {
    use std::net::Ipv4Addr;

    assert_eq!(
        Ipv4AddrDisplay::new(Ipv4Addr::UNSPECIFIED).to_string_ufmt(),
        "0.0.0.0"
    );
    assert_eq!(
        Ipv4AddrDisplay::new(Ipv4Addr::LOCALHOST).to_string_ufmt(),
        "127.0.0.1"
    );
    assert_eq!(
        Ipv4AddrDisplay::new(Ipv4Addr::new(231, 2, 30, 102)).to_string_ufmt(),
        "231.2.30.102"
    );
}
