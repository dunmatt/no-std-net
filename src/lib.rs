// Effectively all the code in this repo is copied with permission from Rust's std library.
// They hold the copyright (http://rust-lang.org/COPYRIGHT) and whatever other rights, but this
// crate is MIT licensed also, so it's all good.

//! Networking primitives for TCP/UDP communication.
//!
//! This module provides networking functionality for the Transmission Control and User
//! Datagram Protocols, as well as types for IP and socket addresses.  It has been ported
//! from std::net to remove the dependency on std.
//!
//! This crate is a WIP, issues, feedback and PRs are welcome as long as they follow the theme of
//! "std::net" clone.
//!
//! # Organization
//!
//! * [`IpAddr`] represents IP addresses of either IPv4 or IPv6; [`Ipv4Addr`] and
//!   [`Ipv6Addr`] are respectively IPv4 and IPv6 addresses
//! * [`TcpListener`] and [`TcpStream`] provide functionality for communication over TCP
//! * [`UdpSocket`] provides functionality for communication over UDP
//! * [`SocketAddr`] represents socket addresses of either IPv4 or IPv6; [`SocketAddrV4`]
//!   and [`SocketAddrV6`] are respectively IPv4 and IPv6 socket addresses
//! * [`ToSocketAddrs`] is a trait that used for generic address resolution when interacting
//!   with networking objects like [`TcpListener`], [`TcpStream`] or [`UdpSocket`]
//! * Other types are return or parameter types for various methods in this module
//!
//! [`IpAddr`]: ../../no-std-net/enum.IpAddr.html
//! [`Ipv4Addr`]: ../../no-std-net/struct.Ipv4Addr.html
//! [`Ipv6Addr`]: ../../no-std-net/struct.Ipv6Addr.html
//! [`SocketAddr`]: ../../std/net/enum.SocketAddr.html
//! [`SocketAddrV4`]: ../../std/net/struct.SocketAddrV4.html
//! [`SocketAddrV6`]: ../../std/net/struct.SocketAddrV6.html
//! [`TcpListener`]: ../../std/net/struct.TcpListener.html
//! [`TcpStream`]: ../../std/net/struct.TcpStream.html
//! [`ToSocketAddrs`]: ../../std/net/trait.ToSocketAddrs.html
//! [`UdpSocket`]: ../../std/net/struct.UdpSocket.html

// TODO: figure out how to put links into rustdocs and update the above

#![no_std]
#![deny(
	dead_code,
	missing_docs,
	unused_imports,
	unused_must_use,
	unused_parens,
	unused_qualifications,
	warnings,
)]
#![forbid(unsafe_code)]

use core::fmt;

mod addr;
mod ip;
mod parser;

#[cfg(feature = "serde")]
extern crate serde;
#[cfg(feature = "serde")]
mod de;
#[cfg(feature = "serde")]
mod ser;


pub use addr::{ SocketAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs };
pub use ip::{ IpAddr, Ipv4Addr, Ipv6Addr, Ipv6MulticastScope };
