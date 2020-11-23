// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>

//! A panic handler that infinitely waits.

use core::panic::PanicInfo;

// #[panic_handler] is used to define the behavior of panic! in #![no_std] applications.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unimplemented!()
}
