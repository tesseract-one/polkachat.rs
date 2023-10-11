#![feature(iterator_try_collect)]
#![feature(result_option_inspect)]
#![feature(async_closure)]
#![feature(auto_traits, negative_impls)]

#[macro_use]
extern crate log;

#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "ios")]
mod ios;

mod core;
mod delegate;
mod error;
mod substrate;
mod contract;
mod ui;

pub (crate) use crate::error::Error;
pub (crate) use crate::error::Result;

pub (crate) use crate::core::Core;

pub (crate) use crate::ui::UI;
pub (crate) use crate::ui::UIProtocol;
