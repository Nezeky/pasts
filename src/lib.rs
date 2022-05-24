// Copyright © 2019-2022 The Pasts Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).
//
//! Minimal and simpler alternative to the futures crate.
//!
//! # Optional Features
//! The **std** feature is enabled by default, disable it to use on `no_std`.
//!
//! # Getting Started
//!
//! Add the following to your *Cargo.toml*:
//! ```toml
//! [dependencies]
//! pasts = "0.10"
//! aysnc-std = "1.0"
//! ```
//!
//! ## Multi-Tasking On Multiple Iterators of Futures
//! This example runs two timers in parallel using the `async-std` crate
//! counting from 0 to 6.  The "one" task will always be run for count 6 and
//! stop the program, although which task will run for count 5 may be either
//! "one" or "two" because they trigger at the same time.
//!
//! ```rust,no_run
#![doc = include_str!("../examples/counter.rs")]
//! ```
#![cfg_attr(not(feature = "std"), no_std)]
#![doc(
    html_logo_url = "https://ardaku.github.io/mm/logo.svg",
    html_favicon_url = "https://ardaku.github.io/mm/icon.svg",
    html_root_url = "https://docs.rs/pasts"
)]
#![forbid(unsafe_code)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

extern crate alloc;

mod exec;
mod iter;
mod r#loop;

pub use exec::{block_on, BlockOn, Executor};
pub use iter::{AsyncIter, AsyncIterator};
pub use r#loop::Loop;

pub mod prelude {
    //! Items that are almost always needed.
    //!
    //! Includes [`Poll`], [`Poll::Pending`], and [`Poll::Ready`].  As well as
    //! [`AsyncIterator`], [`BoxAsyncIterator`], [`LocalBoxAsyncIterator`],
    //! [`Context`], [`Pin`], and [`Future`].

    use alloc::boxed::Box;
    pub use core::{
        future::Future,
        pin::Pin,
        task::{
            Context,
            Poll::{self, Pending, Ready},
        },
    };

    pub use crate::AsyncIterator;

    /// An owned dynamically typed [`AsyncIterator`] for use in cases where you
    /// can't statically type your result or need to add some indirection.
    ///
    /// Requires non-ZST allocator.
    pub type BoxAsyncIterator<'a, T> =
        Pin<Box<dyn AsyncIterator<Item = T> + Unpin + Send + 'a>>;

    /// [`BoxAsyncIterator`], but without the [`Send`] requirement.
    ///
    /// Requires non-ZST allocator.
    pub type LocalBoxAsyncIterator<'a, T> =
        Pin<Box<dyn AsyncIterator<Item = T> + Unpin + 'a>>;
}
