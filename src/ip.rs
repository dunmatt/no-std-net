// Effectively all the code in this repo is copied with permission from Rust's std library.
// They hold the copyright (http://rust-lang.org/COPYRIGHT) and whatever other rights, but this
// crate is MIT licensed also, so it's all good.

// Tests for this module
#[cfg(all(test, not(target_os = "emscripten")))]
mod tests;

use core::cmp::Ordering;
use core::fmt::{self, Write};
use core::hash;

use super::helper::WriteHelper;

/// An IP address, either IPv4 or IPv6.
///
/// This enum can contain either an [`Ipv4Addr`] or an [`Ipv6Addr`], see their
/// respective documentation for more details.
///
/// # Examples
///
/// ```
/// use no_std_net::{IpAddr, Ipv4Addr, Ipv6Addr};
///
/// let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
/// let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
///
/// assert_eq!("127.0.0.1".parse(), Ok(localhost_v4));
/// assert_eq!("::1".parse(), Ok(localhost_v6));
///
/// assert_eq!(localhost_v4.is_ipv6(), false);
/// assert_eq!(localhost_v4.is_ipv4(), true);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum IpAddr {
    /// An IPv4 address.
    V4(Ipv4Addr),
    /// An IPv6 address.
    V6(Ipv6Addr),
}

/// An IPv4 address.
///
/// IPv4 addresses are defined as 32-bit integers in [IETF RFC 791].
/// They are usually represented as four octets.
///
/// See [`IpAddr`] for a type encompassing both IPv4 and IPv6 addresses.
///
/// [IETF RFC 791]: https://tools.ietf.org/html/rfc791
///
/// # Textual representation
///
/// `Ipv4Addr` provides a [`FromStr`] implementation. The four octets are in decimal
/// notation, divided by `.` (this is called "dot-decimal notation").
/// Notably, octal numbers and hexadecimal numbers are not allowed per [IETF RFC 6943].
///
/// [IETF RFC 6943]: https://tools.ietf.org/html/rfc6943#section-3.1.1
/// [`FromStr`]: core::str::FromStr
///
/// # Examples
///
/// ```
/// use no_std_net::Ipv4Addr;
///
/// let localhost = Ipv4Addr::new(127, 0, 0, 1);
/// assert_eq!("127.0.0.1".parse(), Ok(localhost));
/// assert_eq!(localhost.is_loopback(), true);
/// ```
#[derive(Copy)]
pub struct Ipv4Addr {
    // Octets stored in transmit order.
    inner: [u8; 4],
}

/// An IPv6 address.
///
/// IPv6 addresses are defined as 128-bit integers in [IETF RFC 4291].
/// They are usually represented as eight 16-bit segments.
///
/// See [`IpAddr`] for a type encompassing both IPv4 and IPv6 addresses.
///
/// [IETF RFC 4291]: https://tools.ietf.org/html/rfc4291
///
/// # Textual representation
///
/// `Ipv6Addr` provides a [`FromStr`] implementation. There are many ways to represent
/// an IPv6 address in text, but in general, each segments is written in hexadecimal
/// notation, and segments are separated by `:`. For more information, see
/// [IETF RFC 5952].
///
/// [`FromStr`]: core::str::FromStr
/// [IETF RFC 5952]: https://tools.ietf.org/html/rfc5952
///
/// # Examples
///
/// ```
/// use no_std_net::Ipv6Addr;
///
/// let localhost = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
/// assert_eq!("::1".parse(), Ok(localhost));
/// assert_eq!(localhost.is_loopback(), true);
/// ```
#[derive(Copy)]
pub struct Ipv6Addr {
    // Octets stored in transmit order.
    inner: [u8; 16],
}

#[allow(missing_docs)]
#[derive(Copy, PartialEq, Eq, Clone, Hash, Debug)]
#[cfg(feature = "unstable_ip")]
pub enum Ipv6MulticastScope {
    InterfaceLocal,
    LinkLocal,
    RealmLocal,
    AdminLocal,
    SiteLocal,
    OrganizationLocal,
    Global,
}

impl IpAddr {
    /// Returns [`true`] for the special 'unspecified' address.
    ///
    /// See the documentation for [`Ipv4Addr::is_unspecified()`] and
    /// [`Ipv6Addr::is_unspecified()`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::{IpAddr, Ipv4Addr, Ipv6Addr};
    ///
    /// assert_eq!(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)).is_unspecified(), true);
    /// assert_eq!(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)).is_unspecified(), true);
    /// ```
    #[inline]
    pub const fn is_unspecified(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_unspecified(),
            IpAddr::V6(ip) => ip.is_unspecified(),
        }
    }

    /// Returns [`true`] if this is a loopback address.
    ///
    /// See the documentation for [`Ipv4Addr::is_loopback()`] and
    /// [`Ipv6Addr::is_loopback()`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::{IpAddr, Ipv4Addr, Ipv6Addr};
    ///
    /// assert_eq!(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)).is_loopback(), true);
    /// assert_eq!(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0x1)).is_loopback(), true);
    /// ```
    #[inline]
    pub const fn is_loopback(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_loopback(),
            IpAddr::V6(ip) => ip.is_loopback(),
        }
    }

    /// Returns [`true`] if the address appears to be globally routable.
    ///
    /// See the documentation for [`Ipv4Addr::is_global()`] and
    /// [`Ipv6Addr::is_global()`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// // Requires `unstable_ip` feature
    ///
    /// use no_std_net::{IpAddr, Ipv4Addr, Ipv6Addr};
    ///
    /// assert_eq!(IpAddr::V4(Ipv4Addr::new(80, 9, 12, 3)).is_global(), true);
    /// assert_eq!(IpAddr::V6(Ipv6Addr::new(0, 0, 0x1c9, 0, 0, 0xafc8, 0, 0x1)).is_global(), true);
    /// ```
    #[cfg(feature = "unstable_ip")]
    #[inline]
    pub const fn is_global(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_global(),
            IpAddr::V6(ip) => ip.is_global(),
        }
    }

    /// Returns [`true`] if this is a multicast address.
    ///
    /// See the documentation for [`Ipv4Addr::is_multicast()`] and
    /// [`Ipv6Addr::is_multicast()`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::{IpAddr, Ipv4Addr, Ipv6Addr};
    ///
    /// assert_eq!(IpAddr::V4(Ipv4Addr::new(224, 254, 0, 0)).is_multicast(), true);
    /// assert_eq!(IpAddr::V6(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0)).is_multicast(), true);
    /// ```
    #[inline]
    pub const fn is_multicast(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_multicast(),
            IpAddr::V6(ip) => ip.is_multicast(),
        }
    }

    /// Returns [`true`] if this address is in a range designated for documentation.
    ///
    /// See the documentation for [`Ipv4Addr::is_documentation()`] and
    /// [`Ipv6Addr::is_documentation()`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// // Requires `unstable_ip` feature
    ///
    /// use no_std_net::{IpAddr, Ipv4Addr, Ipv6Addr};
    ///
    /// assert_eq!(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 6)).is_documentation(), true);
    /// assert_eq!(
    ///     IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0)).is_documentation(),
    ///     true
    /// );
    /// ```
    #[cfg(feature = "unstable_ip")]
    #[inline]
    pub const fn is_documentation(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_documentation(),
            IpAddr::V6(ip) => ip.is_documentation(),
        }
    }

    /// Returns [`true`] if this address is an [`IPv4` address], and [`false`]
    /// otherwise.
    ///
    /// [`IPv4` address]: IpAddr::V4
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::{IpAddr, Ipv4Addr, Ipv6Addr};
    ///
    /// assert_eq!(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 6)).is_ipv4(), true);
    /// assert_eq!(IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0)).is_ipv4(), false);
    /// ```
    #[inline]
    pub const fn is_ipv4(&self) -> bool {
        matches!(self, IpAddr::V4(_))
    }

    /// Returns [`true`] if this address is an [`IPv6` address], and [`false`]
    /// otherwise.
    ///
    /// [`IPv6` address]: IpAddr::V6
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::{IpAddr, Ipv4Addr, Ipv6Addr};
    ///
    /// assert_eq!(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 6)).is_ipv6(), false);
    /// assert_eq!(IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0)).is_ipv6(), true);
    /// ```
    #[inline]
    pub const fn is_ipv6(&self) -> bool {
        matches!(self, IpAddr::V6(_))
    }
}

impl Ipv4Addr {
    /// Creates a new IPv4 address from four eight-bit octets.
    ///
    /// The result will represent the IP address `a`.`b`.`c`.`d`.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv4Addr;
    ///
    /// let addr = Ipv4Addr::new(127, 0, 0, 1);
    /// ```
    #[inline]
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr {
        Ipv4Addr {
            inner: [a, b, c, d],
        }
    }

    /// An IPv4 address with the address pointing to localhost: `127.0.0.1`
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv4Addr;
    ///
    /// let addr = Ipv4Addr::LOCALHOST;
    /// assert_eq!(addr, Ipv4Addr::new(127, 0, 0, 1));
    /// ```
    pub const LOCALHOST: Self = Ipv4Addr::new(127, 0, 0, 1);

    /// An IPv4 address representing an unspecified address: `0.0.0.0`
    ///
    /// This corresponds to the constant `INADDR_ANY` in other languages.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv4Addr;
    ///
    /// let addr = Ipv4Addr::UNSPECIFIED;
    /// assert_eq!(addr, Ipv4Addr::new(0, 0, 0, 0));
    /// ```
    #[doc(alias = "INADDR_ANY")]
    pub const UNSPECIFIED: Self = Ipv4Addr::new(0, 0, 0, 0);

    /// An IPv4 address representing the broadcast address: `255.255.255.255`
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv4Addr;
    ///
    /// let addr = Ipv4Addr::BROADCAST;
    /// assert_eq!(addr, Ipv4Addr::new(255, 255, 255, 255));
    /// ```
    pub const BROADCAST: Self = Ipv4Addr::new(255, 255, 255, 255);

    /// Returns the four eight-bit integers that make up this address.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv4Addr;
    ///
    /// let addr = Ipv4Addr::new(127, 0, 0, 1);
    /// assert_eq!(addr.octets(), [127, 0, 0, 1]);
    /// ```
    #[inline]
    pub const fn octets(&self) -> [u8; 4] {
        self.inner
    }

    /// Returns [`true`] for the special 'unspecified' address (`0.0.0.0`).
    ///
    /// This property is defined in _UNIX Network Programming, Second Edition_,
    /// W. Richard Stevens, p. 891; see also [ip7].
    ///
    /// [ip7]: http://man7.org/linux/man-pages/man7/ip.7.html
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv4Addr;
    ///
    /// assert_eq!(Ipv4Addr::new(0, 0, 0, 0).is_unspecified(), true);
    /// assert_eq!(Ipv4Addr::new(45, 22, 13, 197).is_unspecified(), false);
    /// ```
    #[inline]
    pub const fn is_unspecified(&self) -> bool {
        self.inner[0] == 0 && self.inner[1] == 0 && self.inner[2] == 0 && self.inner[3] == 0
    }

    /// Returns [`true`] if this is a loopback address (`127.0.0.0/8`).
    ///
    /// This property is defined by [IETF RFC 1122].
    ///
    /// [IETF RFC 1122]: https://tools.ietf.org/html/rfc1122
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv4Addr;
    ///
    /// assert_eq!(Ipv4Addr::new(127, 0, 0, 1).is_loopback(), true);
    /// assert_eq!(Ipv4Addr::new(45, 22, 13, 197).is_loopback(), false);
    /// ```
    #[inline]
    pub const fn is_loopback(&self) -> bool {
        self.octets()[0] == 127
    }

    /// Returns [`true`] if this is a private address.
    ///
    /// The private address ranges are defined in [IETF RFC 1918] and include:
    ///
    ///  - `10.0.0.0/8`
    ///  - `172.16.0.0/12`
    ///  - `192.168.0.0/16`
    ///
    /// [IETF RFC 1918]: https://tools.ietf.org/html/rfc1918
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv4Addr;
    ///
    /// assert_eq!(Ipv4Addr::new(10, 0, 0, 1).is_private(), true);
    /// assert_eq!(Ipv4Addr::new(10, 10, 10, 10).is_private(), true);
    /// assert_eq!(Ipv4Addr::new(172, 16, 10, 10).is_private(), true);
    /// assert_eq!(Ipv4Addr::new(172, 29, 45, 14).is_private(), true);
    /// assert_eq!(Ipv4Addr::new(172, 32, 0, 2).is_private(), false);
    /// assert_eq!(Ipv4Addr::new(192, 168, 0, 2).is_private(), true);
    /// assert_eq!(Ipv4Addr::new(192, 169, 0, 2).is_private(), false);
    /// ```
    #[inline]
    pub const fn is_private(&self) -> bool {
        match self.octets() {
            [10, ..] => true,
            [172, b, ..] if b >= 16 && b <= 31 => true,
            [192, 168, ..] => true,
            _ => false,
        }
    }

    /// Returns [`true`] if the address is link-local (`169.254.0.0/16`).
    ///
    /// This property is defined by [IETF RFC 3927].
    ///
    /// [IETF RFC 3927]: https://tools.ietf.org/html/rfc3927
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv4Addr;
    ///
    /// assert_eq!(Ipv4Addr::new(169, 254, 0, 0).is_link_local(), true);
    /// assert_eq!(Ipv4Addr::new(169, 254, 10, 65).is_link_local(), true);
    /// assert_eq!(Ipv4Addr::new(16, 89, 10, 65).is_link_local(), false);
    /// ```
    #[inline]
    pub const fn is_link_local(&self) -> bool {
        matches!(self.octets(), [169, 254, ..])
    }

    /// Returns [`true`] if the address appears to be globally routable.
    /// See [iana-ipv4-special-registry][ipv4-sr].
    ///
    /// The following return [`false`]:
    ///
    /// - private addresses (see [`Ipv4Addr::is_private()`])
    /// - the loopback address (see [`Ipv4Addr::is_loopback()`])
    /// - the link-local address (see [`Ipv4Addr::is_link_local()`])
    /// - the broadcast address (see [`Ipv4Addr::is_broadcast()`])
    /// - addresses used for documentation (see [`Ipv4Addr::is_documentation()`])
    /// - the unspecified address (see [`Ipv4Addr::is_unspecified()`]), and the whole
    ///   `0.0.0.0/8` block
    /// - addresses reserved for future protocols (see
    /// [`Ipv4Addr::is_ietf_protocol_assignment()`], except
    /// `192.0.0.9/32` and `192.0.0.10/32` which are globally routable
    /// - addresses reserved for future use (see [`Ipv4Addr::is_reserved()`]
    /// - addresses reserved for networking devices benchmarking (see
    /// [`Ipv4Addr::is_benchmarking()`])
    ///
    /// [ipv4-sr]: https://www.iana.org/assignments/iana-ipv4-special-registry/iana-ipv4-special-registry.xhtml
    ///
    /// # Examples
    ///
    /// ```
    /// // Requires `unstable_ip` feature
    ///
    /// use no_std_net::Ipv4Addr;
    ///
    /// // private addresses are not global
    /// assert_eq!(Ipv4Addr::new(10, 254, 0, 0).is_global(), false);
    /// assert_eq!(Ipv4Addr::new(192, 168, 10, 65).is_global(), false);
    /// assert_eq!(Ipv4Addr::new(172, 16, 10, 65).is_global(), false);
    ///
    /// // the 0.0.0.0/8 block is not global
    /// assert_eq!(Ipv4Addr::new(0, 1, 2, 3).is_global(), false);
    /// // in particular, the unspecified address is not global
    /// assert_eq!(Ipv4Addr::new(0, 0, 0, 0).is_global(), false);
    ///
    /// // the loopback address is not global
    /// assert_eq!(Ipv4Addr::new(127, 0, 0, 1).is_global(), false);
    ///
    /// // link local addresses are not global
    /// assert_eq!(Ipv4Addr::new(169, 254, 45, 1).is_global(), false);
    ///
    /// // the broadcast address is not global
    /// assert_eq!(Ipv4Addr::new(255, 255, 255, 255).is_global(), false);
    ///
    /// // the address space designated for documentation is not global
    /// assert_eq!(Ipv4Addr::new(192, 0, 2, 255).is_global(), false);
    /// assert_eq!(Ipv4Addr::new(198, 51, 100, 65).is_global(), false);
    /// assert_eq!(Ipv4Addr::new(203, 0, 113, 6).is_global(), false);
    ///
    /// // shared addresses are not global
    /// assert_eq!(Ipv4Addr::new(100, 100, 0, 0).is_global(), false);
    ///
    /// // addresses reserved for protocol assignment are not global
    /// assert_eq!(Ipv4Addr::new(192, 0, 0, 0).is_global(), false);
    /// assert_eq!(Ipv4Addr::new(192, 0, 0, 255).is_global(), false);
    ///
    /// // addresses reserved for future use are not global
    /// assert_eq!(Ipv4Addr::new(250, 10, 20, 30).is_global(), false);
    ///
    /// // addresses reserved for network devices benchmarking are not global
    /// assert_eq!(Ipv4Addr::new(198, 18, 0, 0).is_global(), false);
    ///
    /// // All the other addresses are global
    /// assert_eq!(Ipv4Addr::new(1, 1, 1, 1).is_global(), true);
    /// assert_eq!(Ipv4Addr::new(80, 9, 12, 3).is_global(), true);
    /// ```
    #[cfg(feature = "unstable_ip")]
    #[inline]
    pub const fn is_global(&self) -> bool {
        // check if this address is 192.0.0.9 or 192.0.0.10. These addresses are the only two
        // globally routable addresses in the 192.0.0.0/24 range.
        if u32::from_be_bytes(self.octets()) == 0xc0000009
            || u32::from_be_bytes(self.octets()) == 0xc000000a
        {
            return true;
        }
        !self.is_private()
            && !self.is_loopback()
            && !self.is_link_local()
            && !self.is_broadcast()
            && !self.is_documentation()
            && !self.is_shared()
            && !self.is_ietf_protocol_assignment()
            && !self.is_reserved()
            && !self.is_benchmarking()
            // Make sure the address is not in 0.0.0.0/8
            && self.octets()[0] != 0
    }

    /// Returns [`true`] if this address is part of the Shared Address Space defined in
    /// [IETF RFC 6598] (`100.64.0.0/10`).
    ///
    /// [IETF RFC 6598]: https://tools.ietf.org/html/rfc6598
    ///
    /// # Examples
    ///
    /// ```
    /// // Requires `unstable_ip` feature
    /// use no_std_net::Ipv4Addr;
    ///
    /// assert_eq!(Ipv4Addr::new(100, 64, 0, 0).is_shared(), true);
    /// assert_eq!(Ipv4Addr::new(100, 127, 255, 255).is_shared(), true);
    /// assert_eq!(Ipv4Addr::new(100, 128, 0, 0).is_shared(), false);
    /// ```
    #[cfg(feature = "unstable_ip")]
    #[inline]
    pub const fn is_shared(&self) -> bool {
        self.octets()[0] == 100 && (self.octets()[1] & 0b1100_0000 == 0b0100_0000)
    }

    /// Returns [`true`] if this address is part of `192.0.0.0/24`, which is reserved to
    /// IANA for IETF protocol assignments, as documented in [IETF RFC 6890].
    ///
    /// Note that parts of this block are in use:
    ///
    /// - `192.0.0.8/32` is the "IPv4 dummy address" (see [IETF RFC 7600])
    /// - `192.0.0.9/32` is the "Port Control Protocol Anycast" (see [IETF RFC 7723])
    /// - `192.0.0.10/32` is used for NAT traversal (see [IETF RFC 8155])
    ///
    /// [IETF RFC 6890]: https://tools.ietf.org/html/rfc6890
    /// [IETF RFC 7600]: https://tools.ietf.org/html/rfc7600
    /// [IETF RFC 7723]: https://tools.ietf.org/html/rfc7723
    /// [IETF RFC 8155]: https://tools.ietf.org/html/rfc8155
    ///
    /// # Examples
    ///
    /// ```
    /// // Requires `unstable_ip` feature
    /// use no_std_net::Ipv4Addr;
    ///
    /// assert_eq!(Ipv4Addr::new(192, 0, 0, 0).is_ietf_protocol_assignment(), true);
    /// assert_eq!(Ipv4Addr::new(192, 0, 0, 8).is_ietf_protocol_assignment(), true);
    /// assert_eq!(Ipv4Addr::new(192, 0, 0, 9).is_ietf_protocol_assignment(), true);
    /// assert_eq!(Ipv4Addr::new(192, 0, 0, 255).is_ietf_protocol_assignment(), true);
    /// assert_eq!(Ipv4Addr::new(192, 0, 1, 0).is_ietf_protocol_assignment(), false);
    /// assert_eq!(Ipv4Addr::new(191, 255, 255, 255).is_ietf_protocol_assignment(), false);
    /// ```
    #[cfg(feature = "unstable_ip")]
    #[inline]
    pub const fn is_ietf_protocol_assignment(&self) -> bool {
        self.octets()[0] == 192 && self.octets()[1] == 0 && self.octets()[2] == 0
    }

    /// Returns [`true`] if this address part of the `198.18.0.0/15` range, which is reserved for
    /// network devices benchmarking. This range is defined in [IETF RFC 2544] as `192.18.0.0`
    /// through `198.19.255.255` but [errata 423] corrects it to `198.18.0.0/15`.
    ///
    /// [IETF RFC 2544]: https://tools.ietf.org/html/rfc2544
    /// [errata 423]: https://www.rfc-editor.org/errata/eid423
    ///
    /// # Examples
    ///
    /// ```
    /// // Requires `unstable_ip` feature
    /// use no_std_net::Ipv4Addr;
    ///
    /// assert_eq!(Ipv4Addr::new(198, 17, 255, 255).is_benchmarking(), false);
    /// assert_eq!(Ipv4Addr::new(198, 18, 0, 0).is_benchmarking(), true);
    /// assert_eq!(Ipv4Addr::new(198, 19, 255, 255).is_benchmarking(), true);
    /// assert_eq!(Ipv4Addr::new(198, 20, 0, 0).is_benchmarking(), false);
    /// ```
    #[cfg(feature = "unstable_ip")]
    #[inline]
    pub const fn is_benchmarking(&self) -> bool {
        self.octets()[0] == 198 && (self.octets()[1] & 0xfe) == 18
    }

    /// Returns [`true`] if this address is reserved by IANA for future use. [IETF RFC 1112]
    /// defines the block of reserved addresses as `240.0.0.0/4`. This range normally includes the
    /// broadcast address `255.255.255.255`, but this implementation explicitly excludes it, since
    /// it is obviously not reserved for future use.
    ///
    /// [IETF RFC 1112]: https://tools.ietf.org/html/rfc1112
    ///
    /// # Warning
    ///
    /// As IANA assigns new addresses, this method will be
    /// updated. This may result in non-reserved addresses being
    /// treated as reserved in code that relies on an outdated version
    /// of this method.
    ///
    /// # Examples
    ///
    /// ```
    /// // Requires `unstable_ip` feature
    /// use no_std_net::Ipv4Addr;
    ///
    /// assert_eq!(Ipv4Addr::new(240, 0, 0, 0).is_reserved(), true);
    /// assert_eq!(Ipv4Addr::new(255, 255, 255, 254).is_reserved(), true);
    ///
    /// assert_eq!(Ipv4Addr::new(239, 255, 255, 255).is_reserved(), false);
    /// // The broadcast address is not considered as reserved for future use by this implementation
    /// assert_eq!(Ipv4Addr::new(255, 255, 255, 255).is_reserved(), false);
    /// ```
    #[cfg(feature = "unstable_ip")]
    #[inline]
    pub const fn is_reserved(&self) -> bool {
        self.octets()[0] & 240 == 240 && !self.is_broadcast()
    }

    /// Returns [`true`] if this is a multicast address (`224.0.0.0/4`).
    ///
    /// Multicast addresses have a most significant octet between `224` and `239`,
    /// and is defined by [IETF RFC 5771].
    ///
    /// [IETF RFC 5771]: https://tools.ietf.org/html/rfc5771
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv4Addr;
    ///
    /// assert_eq!(Ipv4Addr::new(224, 254, 0, 0).is_multicast(), true);
    /// assert_eq!(Ipv4Addr::new(236, 168, 10, 65).is_multicast(), true);
    /// assert_eq!(Ipv4Addr::new(172, 16, 10, 65).is_multicast(), false);
    /// ```
    #[inline]
    pub const fn is_multicast(&self) -> bool {
        self.octets()[0] >= 224 && self.octets()[0] <= 239
    }

    /// Returns [`true`] if this is a broadcast address (`255.255.255.255`).
    ///
    /// A broadcast address has all octets set to `255` as defined in [IETF RFC 919].
    ///
    /// [IETF RFC 919]: https://tools.ietf.org/html/rfc919
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv4Addr;
    ///
    /// assert_eq!(Ipv4Addr::new(255, 255, 255, 255).is_broadcast(), true);
    /// assert_eq!(Ipv4Addr::new(236, 168, 10, 65).is_broadcast(), false);
    /// ```
    #[inline]
    pub const fn is_broadcast(&self) -> bool {
        u32::from_be_bytes(self.octets()) == u32::from_be_bytes(Self::BROADCAST.octets())
    }

    /// Returns [`true`] if this address is in a range designated for documentation.
    ///
    /// This is defined in [IETF RFC 5737]:
    ///
    /// - `192.0.2.0/24` (TEST-NET-1)
    /// - `198.51.100.0/24` (TEST-NET-2)
    /// - `203.0.113.0/24` (TEST-NET-3)
    ///
    /// [IETF RFC 5737]: https://tools.ietf.org/html/rfc5737
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv4Addr;
    ///
    /// assert_eq!(Ipv4Addr::new(192, 0, 2, 255).is_documentation(), true);
    /// assert_eq!(Ipv4Addr::new(198, 51, 100, 65).is_documentation(), true);
    /// assert_eq!(Ipv4Addr::new(203, 0, 113, 6).is_documentation(), true);
    /// assert_eq!(Ipv4Addr::new(193, 34, 17, 19).is_documentation(), false);
    /// ```
    #[inline]
    pub const fn is_documentation(&self) -> bool {
        match self.octets() {
            [192, 0, 2, _] => true,
            [198, 51, 100, _] => true,
            [203, 0, 113, _] => true,
            _ => false,
        }
    }

    /// Converts this address to an IPv4-compatible [`IPv6` address].
    ///
    /// `a.b.c.d` becomes `::a.b.c.d`
    ///
    /// This isn't typically the method you want; these addresses don't typically
    /// function on modern systems. Use `to_ipv6_mapped` instead.
    ///
    /// [`IPv6` address]: Ipv6Addr
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::{Ipv4Addr, Ipv6Addr};
    ///
    /// assert_eq!(
    ///     Ipv4Addr::new(192, 0, 2, 255).to_ipv6_compatible(),
    ///     Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0xc000, 0x2ff)
    /// );
    /// ```
    #[inline]
    pub const fn to_ipv6_compatible(&self) -> Ipv6Addr {
        let [a, b, c, d] = self.octets();
        Ipv6Addr::new(
            0,
            0,
            0,
            0,
            0,
            0,
            ((a as u16) << 8) | b as u16,
            ((c as u16) << 8) | d as u16,
        )
    }

    /// Converts this address to an IPv4-mapped [`IPv6` address].
    ///
    /// `a.b.c.d` becomes `::ffff:a.b.c.d`
    ///
    /// [`IPv6` address]: Ipv6Addr
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::{Ipv4Addr, Ipv6Addr};
    ///
    /// assert_eq!(Ipv4Addr::new(192, 0, 2, 255).to_ipv6_mapped(),
    ///            Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc000, 0x2ff));
    /// ```
    #[inline]
    pub const fn to_ipv6_mapped(&self) -> Ipv6Addr {
        let [a, b, c, d] = self.octets();
        Ipv6Addr::new(
            0,
            0,
            0,
            0,
            0,
            0xffff,
            ((a as u16) << 8) | b as u16,
            ((c as u16) << 8) | d as u16,
        )
    }
}

impl fmt::Display for IpAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpAddr::V4(ip) => ip.fmt(fmt),
            IpAddr::V6(ip) => ip.fmt(fmt),
        }
    }
}

impl fmt::Debug for IpAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl From<Ipv4Addr> for IpAddr {
    /// Copies this address to a new `IpAddr::V4`.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::{IpAddr, Ipv4Addr};
    ///
    /// let addr = Ipv4Addr::new(127, 0, 0, 1);
    ///
    /// assert_eq!(
    ///     IpAddr::V4(addr),
    ///     IpAddr::from(addr)
    /// )
    /// ```
    #[inline]
    fn from(ipv4: Ipv4Addr) -> IpAddr {
        IpAddr::V4(ipv4)
    }
}

impl From<Ipv6Addr> for IpAddr {
    /// Copies this address to a new `IpAddr::V6`.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::{IpAddr, Ipv6Addr};
    ///
    /// let addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff);
    ///
    /// assert_eq!(
    ///     IpAddr::V6(addr),
    ///     IpAddr::from(addr)
    /// );
    /// ```
    #[inline]
    fn from(ipv6: Ipv6Addr) -> IpAddr {
        IpAddr::V6(ipv6)
    }
}

impl fmt::Display for Ipv4Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octets = self.octets();
        // Fast Path: if there's no alignment stuff, write directly to the buffer
        if fmt.precision().is_none() && fmt.width().is_none() {
            write!(
                fmt,
                "{}.{}.{}.{}",
                octets[0], octets[1], octets[2], octets[3]
            )
        } else {
            const IPV4_BUF_LEN: usize = 15; // Long enough for the longest possible IPv4 address
            let mut buf = [0u8; IPV4_BUF_LEN];
            let mut buf_slice = WriteHelper::new(&mut buf[..]);

            // Note: The call to write should never fail, hence the unwrap
            write!(
                buf_slice,
                "{}.{}.{}.{}",
                octets[0], octets[1], octets[2], octets[3]
            )
            .unwrap();
            let len = IPV4_BUF_LEN - buf_slice.into_raw().len();

            // This unsafe is OK because we know what is being written to the buffer
            let buf = unsafe { core::str::from_utf8_unchecked(&buf[..len]) };
            fmt.pad(buf)
        }
    }
}

impl fmt::Debug for Ipv4Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl Clone for Ipv4Addr {
    #[inline]
    fn clone(&self) -> Ipv4Addr {
        *self
    }
}

impl PartialEq for Ipv4Addr {
    #[inline]
    fn eq(&self, other: &Ipv4Addr) -> bool {
        self.inner == other.inner
    }
}

impl PartialEq<Ipv4Addr> for IpAddr {
    #[inline]
    fn eq(&self, other: &Ipv4Addr) -> bool {
        match self {
            IpAddr::V4(v4) => v4 == other,
            IpAddr::V6(_) => false,
        }
    }
}

impl PartialEq<IpAddr> for Ipv4Addr {
    #[inline]
    fn eq(&self, other: &IpAddr) -> bool {
        match other {
            IpAddr::V4(v4) => self == v4,
            IpAddr::V6(_) => false,
        }
    }
}

impl Eq for Ipv4Addr {}

impl hash::Hash for Ipv4Addr {
    #[inline]
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        self.inner.hash(s)
    }
}

impl PartialOrd for Ipv4Addr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv4Addr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<Ipv4Addr> for IpAddr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv4Addr) -> Option<Ordering> {
        match self {
            IpAddr::V4(v4) => v4.partial_cmp(other),
            IpAddr::V6(_) => Some(Ordering::Greater),
        }
    }
}

impl PartialOrd<IpAddr> for Ipv4Addr {
    #[inline]
    fn partial_cmp(&self, other: &IpAddr) -> Option<Ordering> {
        match other {
            IpAddr::V4(v4) => self.partial_cmp(v4),
            IpAddr::V6(_) => Some(Ordering::Less),
        }
    }
}

impl Ord for Ipv4Addr {
    #[inline]
    fn cmp(&self, other: &Ipv4Addr) -> Ordering {
        // Compare as native endian
        u32::from_be_bytes(self.inner).cmp(&u32::from_be_bytes(other.inner))
    }
}

impl From<Ipv4Addr> for u32 {
    /// Converts an `Ipv4Addr` into a host byte order `u32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv4Addr;
    ///
    /// let addr = Ipv4Addr::new(0xca, 0xfe, 0xba, 0xbe);
    /// assert_eq!(0xcafebabe, u32::from(addr));
    /// ```
    #[inline]
    fn from(ip: Ipv4Addr) -> u32 {
        let ip = ip.octets();
        u32::from_be_bytes(ip)
    }
}

impl From<u32> for Ipv4Addr {
    /// Converts a host byte order `u32` into an `Ipv4Addr`.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv4Addr;
    ///
    /// let addr = Ipv4Addr::from(0xcafebabe);
    /// assert_eq!(Ipv4Addr::new(0xca, 0xfe, 0xba, 0xbe), addr);
    /// ```
    #[inline]
    fn from(ip: u32) -> Ipv4Addr {
        Ipv4Addr::from(ip.to_be_bytes())
    }
}

impl From<[u8; 4]> for Ipv4Addr {
    /// Creates an `Ipv4Addr` from a four element byte array.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv4Addr;
    ///
    /// let addr = Ipv4Addr::from([13u8, 12u8, 11u8, 10u8]);
    /// assert_eq!(Ipv4Addr::new(13, 12, 11, 10), addr);
    /// ```
    #[inline]
    fn from(octets: [u8; 4]) -> Ipv4Addr {
        Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3])
    }
}

impl From<[u8; 4]> for IpAddr {
    /// Creates an `IpAddr::V4` from a four element byte array.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::{IpAddr, Ipv4Addr};
    ///
    /// let addr = IpAddr::from([13u8, 12u8, 11u8, 10u8]);
    /// assert_eq!(IpAddr::V4(Ipv4Addr::new(13, 12, 11, 10)), addr);
    /// ```
    #[inline]
    fn from(octets: [u8; 4]) -> IpAddr {
        IpAddr::V4(Ipv4Addr::from(octets))
    }
}

impl Ipv6Addr {
    /// Creates a new IPv6 address from eight 16-bit segments.
    ///
    /// The result will represent the IP address `a:b:c:d:e:f:g:h`.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv6Addr;
    ///
    /// let addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff);
    /// ```
    #[inline]
    pub const fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Ipv6Addr {
        Ipv6Addr {
            inner: [
                (a >> 8) as u8,
                a as u8,
                (b >> 8) as u8,
                b as u8,
                (c >> 8) as u8,
                c as u8,
                (d >> 8) as u8,
                d as u8,
                (e >> 8) as u8,
                e as u8,
                (f >> 8) as u8,
                f as u8,
                (g >> 8) as u8,
                g as u8,
                (h >> 8) as u8,
                h as u8,
            ],
        }
    }

    /// An IPv6 address representing localhost: `::1`.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv6Addr;
    ///
    /// let addr = Ipv6Addr::LOCALHOST;
    /// assert_eq!(addr, Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    /// ```
    pub const LOCALHOST: Self = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);

    /// An IPv6 address representing the unspecified address: `::`
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv6Addr;
    ///
    /// let addr = Ipv6Addr::UNSPECIFIED;
    /// assert_eq!(addr, Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0));
    /// ```
    pub const UNSPECIFIED: Self = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);

    /// Returns the eight 16-bit segments that make up this address.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv6Addr;
    ///
    /// assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).segments(),
    ///            [0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff]);
    /// ```
    #[inline]
    pub const fn segments(&self) -> [u16; 8] {
        let arr = self.octets();
        [
            (arr[0] as u16) << 8 | (arr[1] as u16),
            (arr[2] as u16) << 8 | (arr[3] as u16),
            (arr[4] as u16) << 8 | (arr[5] as u16),
            (arr[6] as u16) << 8 | (arr[7] as u16),
            (arr[8] as u16) << 8 | (arr[9] as u16),
            (arr[10] as u16) << 8 | (arr[11] as u16),
            (arr[12] as u16) << 8 | (arr[13] as u16),
            (arr[14] as u16) << 8 | (arr[15] as u16),
        ]
    }

    /// Returns [`true`] for the special 'unspecified' address (`::`).
    ///
    /// This property is defined in [IETF RFC 4291].
    ///
    /// [IETF RFC 4291]: https://tools.ietf.org/html/rfc4291
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv6Addr;
    ///
    /// assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).is_unspecified(), false);
    /// assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0).is_unspecified(), true);
    /// ```
    #[inline]
    pub const fn is_unspecified(&self) -> bool {
        u128::from_be_bytes(self.octets()) == u128::from_be_bytes(Ipv6Addr::UNSPECIFIED.octets())
    }

    /// Returns [`true`] if this is a loopback address (::1).
    ///
    /// This property is defined in [IETF RFC 4291].
    ///
    /// [IETF RFC 4291]: https://tools.ietf.org/html/rfc4291
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv6Addr;
    ///
    /// assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).is_loopback(), false);
    /// assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0x1).is_loopback(), true);
    /// ```
    #[inline]
    pub const fn is_loopback(&self) -> bool {
        u128::from_be_bytes(self.octets()) == u128::from_be_bytes(Ipv6Addr::LOCALHOST.octets())
    }

    /// Returns [`true`] if the address appears to be globally routable.
    ///
    /// The following return [`false`]:
    ///
    /// - the loopback address
    /// - link-local and unique local unicast addresses
    /// - interface-, link-, realm-, admin- and site-local multicast addresses
    ///
    /// # Examples
    ///
    /// ```
    /// // Requires `unstable_ip` feature
    ///
    /// use no_std_net::Ipv6Addr;
    ///
    /// assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).is_global(), true);
    /// assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0x1).is_global(), false);
    /// assert_eq!(Ipv6Addr::new(0, 0, 0x1c9, 0, 0, 0xafc8, 0, 0x1).is_global(), true);
    /// ```
    #[cfg(feature = "unstable_ip")]
    #[inline]
    pub const fn is_global(&self) -> bool {
        match self.multicast_scope() {
            Some(Ipv6MulticastScope::Global) => true,
            None => self.is_unicast_global(),
            _ => false,
        }
    }

    /// Returns [`true`] if this is a unique local address (`fc00::/7`).
    ///
    /// This property is defined in [IETF RFC 4193].
    ///
    /// [IETF RFC 4193]: https://tools.ietf.org/html/rfc4193
    ///
    /// # Examples
    ///
    /// ```
    /// // Requires `unstable_ip` feature
    ///
    /// use no_std_net::Ipv6Addr;
    ///
    /// assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).is_unique_local(), false);
    /// assert_eq!(Ipv6Addr::new(0xfc02, 0, 0, 0, 0, 0, 0, 0).is_unique_local(), true);
    /// ```
    #[cfg(feature = "unstable_ip")]
    #[inline]
    pub const fn is_unique_local(&self) -> bool {
        (self.segments()[0] & 0xfe00) == 0xfc00
    }

    /// Returns [`true`] if this is a unicast address, as defined by [IETF RFC 4291].
    /// Any address that is not a [multicast address] (`ff00::/8`) is unicast.
    ///
    /// [IETF RFC 4291]: https://tools.ietf.org/html/rfc4291
    /// [multicast address]: Ipv6Addr::is_multicast
    ///
    /// # Examples
    ///
    /// ```
    /// // Requires `unstable_ip` feature
    ///
    /// use no_std_net::Ipv6Addr;
    ///
    /// // The unspecified and loopback addresses are unicast.
    /// assert_eq!(Ipv6Addr::UNSPECIFIED.is_unicast(), true);
    /// assert_eq!(Ipv6Addr::LOCALHOST.is_unicast(), true);
    ///
    /// // Any address that is not a multicast address (`ff00::/8`) is unicast.
    /// assert_eq!(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0).is_unicast(), true);
    /// assert_eq!(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0).is_unicast(), false);
    /// ```
    #[cfg(feature = "unstable_ip")]
    #[inline]
    pub const fn is_unicast(&self) -> bool {
        !self.is_multicast()
    }

    /// Returns `true` if the address is a unicast address with link-local scope,
    /// as defined in [RFC 4291].
    ///
    /// A unicast address has link-local scope if it has the prefix `fe80::/10`, as per [RFC 4291 section 2.4].
    /// Note that this encompasses more addresses than those defined in [RFC 4291 section 2.5.6],
    /// which describes "Link-Local IPv6 Unicast Addresses" as having the following stricter format:
    ///
    /// ```text
    /// | 10 bits  |         54 bits         |          64 bits           |
    /// +----------+-------------------------+----------------------------+
    /// |1111111010|           0             |       interface ID         |
    /// +----------+-------------------------+----------------------------+
    /// ```
    /// So while currently the only addresses with link-local scope an application will encounter are all in `fe80::/64`,
    /// this might change in the future with the publication of new standards. More addresses in `fe80::/10` could be allocated,
    /// and those addresses will have link-local scope.
    ///
    /// Also note that while [RFC 4291 section 2.5.3] mentions about the [loopback address] (`::1`) that "it is treated as having Link-Local scope",
    /// this does not mean that the loopback address actually has link-local scope and this method will return `false` on it.
    ///
    /// [RFC 4291]: https://tools.ietf.org/html/rfc4291
    /// [RFC 4291 section 2.4]: https://tools.ietf.org/html/rfc4291#section-2.4
    /// [RFC 4291 section 2.5.3]: https://tools.ietf.org/html/rfc4291#section-2.5.3
    /// [RFC 4291 section 2.5.6]: https://tools.ietf.org/html/rfc4291#section-2.5.6
    /// [loopback address]: Ipv6Addr::LOCALHOST
    ///
    /// # Examples
    ///
    /// ```
    /// // Requires `unstable_ip` feature
    ///
    /// use no_std_net::Ipv6Addr;
    ///
    /// // The loopback address (`::1`) does not actually have link-local scope.
    /// assert_eq!(Ipv6Addr::LOCALHOST.is_unicast_link_local(), false);
    ///
    /// // Only addresses in `fe80::/10` have link-local scope.
    /// assert_eq!(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0).is_unicast_link_local(), false);
    /// assert_eq!(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 0).is_unicast_link_local(), true);
    ///
    /// // Addresses outside the stricter `fe80::/64` also have link-local scope.
    /// assert_eq!(Ipv6Addr::new(0xfe80, 0, 0, 1, 0, 0, 0, 0).is_unicast_link_local(), true);
    /// assert_eq!(Ipv6Addr::new(0xfe81, 0, 0, 0, 0, 0, 0, 0).is_unicast_link_local(), true);
    /// ```
    #[cfg(feature = "unstable_ip")]
    #[inline]
    pub const fn is_unicast_link_local(&self) -> bool {
        (self.segments()[0] & 0xffc0) == 0xfe80
    }

    /// Returns [`true`] if this is a deprecated unicast site-local address (`fec0::/10`). The
    /// unicast site-local address format is defined in [RFC 4291 section 2.5.7] as:
    ///
    /// ```no_rust
    /// |   10     |
    /// |  bits    |         54 bits         |         64 bits            |
    /// +----------+-------------------------+----------------------------+
    /// |1111111011|        subnet ID        |       interface ID         |
    /// +----------+-------------------------+----------------------------+
    /// ```
    ///
    /// [RFC 4291 section 2.5.7]: https://tools.ietf.org/html/rfc4291#section-2.5.7
    ///
    /// # Examples
    ///
    /// ```
    /// // Requires `unstable_ip` feature
    ///
    /// use no_std_net::Ipv6Addr;
    ///
    /// assert_eq!(
    ///     Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).is_unicast_site_local(),
    ///     false
    /// );
    /// assert_eq!(Ipv6Addr::new(0xfec2, 0, 0, 0, 0, 0, 0, 0).is_unicast_site_local(), true);
    /// ```
    ///
    /// # Warning
    ///
    /// As per [RFC 3879], the whole `fec0::/10` prefix is
    /// deprecated. New software must not support site-local
    /// addresses.
    ///
    /// [RFC 3879]: https://tools.ietf.org/html/rfc3879
    #[cfg(feature = "unstable_ip")]
    #[inline]
    pub const fn is_unicast_site_local(&self) -> bool {
        (self.segments()[0] & 0xffc0) == 0xfec0
    }

    /// Returns [`true`] if this is an address reserved for documentation
    /// (`2001:db8::/32`).
    ///
    /// This property is defined in [IETF RFC 3849].
    ///
    /// [IETF RFC 3849]: https://tools.ietf.org/html/rfc3849
    ///
    /// # Examples
    ///
    /// ```
    /// // Requires `unstable_ip` feature
    ///
    /// use no_std_net::Ipv6Addr;
    ///
    /// assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).is_documentation(), false);
    /// assert_eq!(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0).is_documentation(), true);
    /// ```
    #[cfg(feature = "unstable_ip")]
    #[inline]
    pub const fn is_documentation(&self) -> bool {
        (self.segments()[0] == 0x2001) && (self.segments()[1] == 0xdb8)
    }

    /// Returns [`true`] if the address is a globally routable unicast address.
    ///
    /// The following return false:
    ///
    /// - the loopback address
    /// - the link-local addresses
    /// - unique local addresses
    /// - the unspecified address
    /// - the address range reserved for documentation
    ///
    /// This method returns [`true`] for site-local addresses as per [RFC 4291 section 2.5.7]
    ///
    /// ```no_rust
    /// The special behavior of [the site-local unicast] prefix defined in [RFC3513] must no longer
    /// be supported in new implementations (i.e., new implementations must treat this prefix as
    /// Global Unicast).
    /// ```
    ///
    /// [RFC 4291 section 2.5.7]: https://tools.ietf.org/html/rfc4291#section-2.5.7
    ///
    /// # Examples
    ///
    /// ```
    /// // Requires `unstable_ip` feature
    ///
    /// use no_std_net::Ipv6Addr;
    ///
    /// assert_eq!(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0).is_unicast_global(), false);
    /// assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).is_unicast_global(), true);
    /// ```
    #[cfg(feature = "unstable_ip")]
    #[inline]
    pub const fn is_unicast_global(&self) -> bool {
        self.is_unicast()
            && !self.is_loopback()
            && !self.is_unicast_link_local()
            && !self.is_unique_local()
            && !self.is_unspecified()
            && !self.is_documentation()
    }

    /// Returns the address's multicast scope if the address is multicast.
    ///
    /// # Examples
    ///
    /// ```
    /// // Requires `unstable_ip` feature
    ///
    /// use no_std_net::{Ipv6Addr, Ipv6MulticastScope};
    ///
    /// assert_eq!(
    ///     Ipv6Addr::new(0xff0e, 0, 0, 0, 0, 0, 0, 0).multicast_scope(),
    ///     Some(Ipv6MulticastScope::Global)
    /// );
    /// assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).multicast_scope(), None);
    /// ```
    #[cfg(feature = "unstable_ip")]
    #[inline]
    pub const fn multicast_scope(&self) -> Option<Ipv6MulticastScope> {
        if self.is_multicast() {
            match self.segments()[0] & 0x000f {
                1 => Some(Ipv6MulticastScope::InterfaceLocal),
                2 => Some(Ipv6MulticastScope::LinkLocal),
                3 => Some(Ipv6MulticastScope::RealmLocal),
                4 => Some(Ipv6MulticastScope::AdminLocal),
                5 => Some(Ipv6MulticastScope::SiteLocal),
                8 => Some(Ipv6MulticastScope::OrganizationLocal),
                14 => Some(Ipv6MulticastScope::Global),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Returns [`true`] if this is a multicast address (`ff00::/8`).
    ///
    /// This property is defined by [IETF RFC 4291].
    ///
    /// [IETF RFC 4291]: https://tools.ietf.org/html/rfc4291
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv6Addr;
    ///
    /// assert_eq!(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0).is_multicast(), true);
    /// assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).is_multicast(), false);
    /// ```
    #[inline]
    pub const fn is_multicast(&self) -> bool {
        (self.segments()[0] & 0xff00) == 0xff00
    }

    /// Converts this address to an [`IPv4` address] if it's an "IPv4-mapped IPv6 address"
    /// defined in [IETF RFC 4291 section 2.5.5.2], otherwise returns [`None`].
    ///
    /// `::ffff:a.b.c.d` becomes `a.b.c.d`.
    /// All addresses *not* starting with `::ffff` will return `None`.
    ///
    /// [`IPv4` address]: Ipv4Addr
    /// [IETF RFC 4291 section 2.5.5.2]: https://tools.ietf.org/html/rfc4291#section-2.5.5.2
    ///
    /// # Examples
    ///
    /// ```
    /// // Requires `unstable_ip` feature
    ///
    /// use no_std_net::{Ipv4Addr, Ipv6Addr};
    ///
    /// assert_eq!(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0).to_ipv4_mapped(), None);
    /// assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).to_ipv4_mapped(),
    ///            Some(Ipv4Addr::new(192, 10, 2, 255)));
    /// assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1).to_ipv4_mapped(), None);
    /// ```
    #[cfg(feature = "unstable_ip")]
    #[inline]
    pub const fn to_ipv4_mapped(&self) -> Option<Ipv4Addr> {
        match self.octets() {
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, a, b, c, d] => {
                Some(Ipv4Addr::new(a, b, c, d))
            }
            _ => None,
        }
    }

    /// Converts this address to an [`IPv4` address]. Returns [`None`] if this address is
    /// neither IPv4-compatible or IPv4-mapped.
    ///
    /// `::a.b.c.d` and `::ffff:a.b.c.d` become `a.b.c.d`
    ///
    /// [`IPv4` address]: Ipv4Addr
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::{Ipv4Addr, Ipv6Addr};
    ///
    /// assert_eq!(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0).to_ipv4(), None);
    /// assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff).to_ipv4(),
    ///            Some(Ipv4Addr::new(192, 10, 2, 255)));
    /// assert_eq!(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1).to_ipv4(),
    ///            Some(Ipv4Addr::new(0, 0, 0, 1)));
    /// ```
    #[inline]
    pub const fn to_ipv4(&self) -> Option<Ipv4Addr> {
        if let [0, 0, 0, 0, 0, 0 | 0xffff, ab, cd] = self.segments() {
            let [a, b] = ab.to_be_bytes();
            let [c, d] = cd.to_be_bytes();
            Some(Ipv4Addr::new(a, b, c, d))
        } else {
            None
        }
    }

    /// Returns the sixteen eight-bit integers the IPv6 address consists of.
    ///
    /// ```
    /// use no_std_net::Ipv6Addr;
    ///
    /// assert_eq!(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0).octets(),
    ///            [255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    /// ```
    #[inline]
    pub const fn octets(&self) -> [u8; 16] {
        self.inner
    }
}

/// Write an Ipv6Addr, conforming to the canonical style described by
/// [RFC 5952](https://tools.ietf.org/html/rfc5952).
impl fmt::Display for Ipv6Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // If there are no alignment requirements, write out the IP address to
        // f. Otherwise, write it to a local buffer, then use f.pad.
        if f.precision().is_none() && f.width().is_none() {
            let segments = self.segments();

            // Special case for :: and ::1; otherwise they get written with the
            // IPv4 formatter
            if self.is_unspecified() {
                f.write_str("::")
            } else if self.is_loopback() {
                f.write_str("::1")
            } else if let Some(ipv4) = self.to_ipv4() {
                match segments[5] {
                    // IPv4 Compatible address
                    0 => write!(f, "::{}", ipv4),
                    // IPv4 Mapped address
                    0xffff => write!(f, "::ffff:{}", ipv4),
                    _ => unreachable!(),
                }
            } else {
                #[derive(Copy, Clone, Default)]
                struct Span {
                    start: usize,
                    len: usize,
                }

                // Find the inner 0 span
                let zeroes = {
                    let mut longest = Span::default();
                    let mut current = Span::default();

                    for (i, &segment) in segments.iter().enumerate() {
                        if segment == 0 {
                            if current.len == 0 {
                                current.start = i;
                            }

                            current.len += 1;

                            if current.len > longest.len {
                                longest = current;
                            }
                        } else {
                            current = Span::default();
                        }
                    }

                    longest
                };

                /// Write a colon-separated part of the address
                #[inline]
                fn fmt_subslice(f: &mut fmt::Formatter<'_>, chunk: &[u16]) -> fmt::Result {
                    if let Some((first, tail)) = chunk.split_first() {
                        write!(f, "{:x}", first)?;
                        for segment in tail {
                            f.write_char(':')?;
                            write!(f, "{:x}", segment)?;
                        }
                    }
                    Ok(())
                }

                if zeroes.len > 1 {
                    fmt_subslice(f, &segments[..zeroes.start])?;
                    f.write_str("::")?;
                    fmt_subslice(f, &segments[zeroes.start + zeroes.len..])
                } else {
                    fmt_subslice(f, &segments)
                }
            }
        } else {
            // Slow path: write the address to a local buffer, the use f.pad.
            // Defined recursively by using the fast path to write to the
            // buffer.

            // This is the largest possible size of an IPv6 address
            const IPV6_BUF_LEN: usize = (4 * 8) + 7;
            let mut buf = [0u8; IPV6_BUF_LEN];
            let mut buf_slice = WriteHelper::new(&mut buf[..]);

            // Note: This call to write should never fail, so unwrap is okay.
            write!(buf_slice, "{}", self).unwrap();
            let len = IPV6_BUF_LEN - buf_slice.into_raw().len();

            // This is safe because we know exactly what can be in this buffer
            let buf = unsafe { core::str::from_utf8_unchecked(&buf[..len]) };
            f.pad(buf)
        }
    }
}

impl fmt::Debug for Ipv6Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl Clone for Ipv6Addr {
    #[inline]
    fn clone(&self) -> Ipv6Addr {
        *self
    }
}

impl PartialEq for Ipv6Addr {
    #[inline]
    fn eq(&self, other: &Ipv6Addr) -> bool {
        self.inner == other.inner
    }
}

impl PartialEq<IpAddr> for Ipv6Addr {
    #[inline]
    fn eq(&self, other: &IpAddr) -> bool {
        match other {
            IpAddr::V4(_) => false,
            IpAddr::V6(v6) => self == v6,
        }
    }
}

impl PartialEq<Ipv6Addr> for IpAddr {
    #[inline]
    fn eq(&self, other: &Ipv6Addr) -> bool {
        match self {
            IpAddr::V4(_) => false,
            IpAddr::V6(v6) => v6 == other,
        }
    }
}

impl Eq for Ipv6Addr {}

impl hash::Hash for Ipv6Addr {
    #[inline]
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        self.inner.hash(s)
    }
}

impl PartialOrd for Ipv6Addr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv6Addr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<Ipv6Addr> for IpAddr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv6Addr) -> Option<Ordering> {
        match self {
            IpAddr::V4(_) => Some(Ordering::Less),
            IpAddr::V6(v6) => v6.partial_cmp(other),
        }
    }
}

impl PartialOrd<IpAddr> for Ipv6Addr {
    #[inline]
    fn partial_cmp(&self, other: &IpAddr) -> Option<Ordering> {
        match other {
            IpAddr::V4(_) => Some(Ordering::Greater),
            IpAddr::V6(v6) => self.partial_cmp(v6),
        }
    }
}

impl Ord for Ipv6Addr {
    #[inline]
    fn cmp(&self, other: &Ipv6Addr) -> Ordering {
        self.segments().cmp(&other.segments())
    }
}
impl From<Ipv6Addr> for u128 {
    /// Convert an `Ipv6Addr` into a host byte order `u128`.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv6Addr;
    ///
    /// let addr = Ipv6Addr::new(
    ///     0x1020, 0x3040, 0x5060, 0x7080,
    ///     0x90A0, 0xB0C0, 0xD0E0, 0xF00D,
    /// );
    /// assert_eq!(0x102030405060708090A0B0C0D0E0F00D_u128, u128::from(addr));
    /// ```
    #[inline]
    fn from(ip: Ipv6Addr) -> u128 {
        let ip = ip.octets();
        u128::from_be_bytes(ip)
    }
}
impl From<u128> for Ipv6Addr {
    /// Convert a host byte order `u128` into an `Ipv6Addr`.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv6Addr;
    ///
    /// let addr = Ipv6Addr::from(0x102030405060708090A0B0C0D0E0F00D_u128);
    /// assert_eq!(
    ///     Ipv6Addr::new(
    ///         0x1020, 0x3040, 0x5060, 0x7080,
    ///         0x90A0, 0xB0C0, 0xD0E0, 0xF00D,
    ///     ),
    ///     addr);
    /// ```
    #[inline]
    fn from(ip: u128) -> Ipv6Addr {
        Ipv6Addr::from(ip.to_be_bytes())
    }
}

impl From<[u8; 16]> for Ipv6Addr {
    /// Creates an `Ipv6Addr` from a sixteen element byte array.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv6Addr;
    ///
    /// let addr = Ipv6Addr::from([
    ///     25u8, 24u8, 23u8, 22u8, 21u8, 20u8, 19u8, 18u8,
    ///     17u8, 16u8, 15u8, 14u8, 13u8, 12u8, 11u8, 10u8,
    /// ]);
    /// assert_eq!(
    ///     Ipv6Addr::new(
    ///         0x1918, 0x1716,
    ///         0x1514, 0x1312,
    ///         0x1110, 0x0f0e,
    ///         0x0d0c, 0x0b0a
    ///     ),
    ///     addr
    /// );
    /// ```
    #[inline]
    fn from(octets: [u8; 16]) -> Ipv6Addr {
        Ipv6Addr { inner: octets }
    }
}

impl From<[u16; 8]> for Ipv6Addr {
    /// Creates an `Ipv6Addr` from an eight element 16-bit array.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::Ipv6Addr;
    ///
    /// let addr = Ipv6Addr::from([
    ///     525u16, 524u16, 523u16, 522u16,
    ///     521u16, 520u16, 519u16, 518u16,
    /// ]);
    /// assert_eq!(
    ///     Ipv6Addr::new(
    ///         0x20d, 0x20c,
    ///         0x20b, 0x20a,
    ///         0x209, 0x208,
    ///         0x207, 0x206
    ///     ),
    ///     addr
    /// );
    /// ```
    #[inline]
    fn from(segments: [u16; 8]) -> Ipv6Addr {
        let [a, b, c, d, e, f, g, h] = segments;
        Ipv6Addr::new(a, b, c, d, e, f, g, h)
    }
}

impl From<[u8; 16]> for IpAddr {
    /// Creates an `IpAddr::V6` from a sixteen element byte array.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::{IpAddr, Ipv6Addr};
    ///
    /// let addr = IpAddr::from([
    ///     25u8, 24u8, 23u8, 22u8, 21u8, 20u8, 19u8, 18u8,
    ///     17u8, 16u8, 15u8, 14u8, 13u8, 12u8, 11u8, 10u8,
    /// ]);
    /// assert_eq!(
    ///     IpAddr::V6(Ipv6Addr::new(
    ///         0x1918, 0x1716,
    ///         0x1514, 0x1312,
    ///         0x1110, 0x0f0e,
    ///         0x0d0c, 0x0b0a
    ///     )),
    ///     addr
    /// );
    /// ```
    #[inline]
    fn from(octets: [u8; 16]) -> IpAddr {
        IpAddr::V6(Ipv6Addr::from(octets))
    }
}

impl From<[u16; 8]> for IpAddr {
    /// Creates an `IpAddr::V6` from an eight element 16-bit array.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_net::{IpAddr, Ipv6Addr};
    ///
    /// let addr = IpAddr::from([
    ///     525u16, 524u16, 523u16, 522u16,
    ///     521u16, 520u16, 519u16, 518u16,
    /// ]);
    /// assert_eq!(
    ///     IpAddr::V6(Ipv6Addr::new(
    ///         0x20d, 0x20c,
    ///         0x20b, 0x20a,
    ///         0x209, 0x208,
    ///         0x207, 0x206
    ///     )),
    ///     addr
    /// );
    /// ```
    #[inline]
    fn from(segments: [u16; 8]) -> IpAddr {
        IpAddr::V6(Ipv6Addr::from(segments))
    }
}
