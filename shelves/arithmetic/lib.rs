/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// This example uses `uniffi::` macros to describe the interface.

// This must come before other uniffi proc macros, due to how uniffi is patched.
uniffi::setup_scaffolding!("arithmetic");

extern crate trig;

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum ArithmeticError {
    #[error("Integer overflow on an operation with {a} and {b}")]
    IntegerOverflow { a: u64, b: u64 },
}

#[uniffi::export]
fn add(a: u64, b: u64) -> Result<u64> {
    a.checked_add(b)
        .ok_or(ArithmeticError::IntegerOverflow { a, b })
}

#[uniffi::export]
fn sub(a: u64, b: u64) -> Result<u64> {
    a.checked_sub(b)
        .ok_or(ArithmeticError::IntegerOverflow { a, b })
}

#[uniffi::export]
fn div(dividend: u64, divisor: u64) -> u64 {
    if divisor == 0 {
        panic!("Can't divide by zero");
    }
    dividend / divisor
}

#[uniffi::export]
fn equal(a: u64, b: u64) -> bool {
    a == b
}

type Result<T, E = ArithmeticError> = std::result::Result<T, E>;

// test an export from a subcrate

use subcrate::A;

#[uniffi::export]
fn hi() -> A {
    A { a: 64 }
}
