/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![no_std]

#[cfg(feature = "panic")]
#[panic_handler]
fn panic(_p: &core::panic::PanicInfo) -> ! {
    loop {}
}
