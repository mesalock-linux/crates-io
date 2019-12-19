// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

//! # The Rust SGX SDK Standard Library
//!
//! The Rust SGX standard library (previously named as `sgx_tstdc`) is
//! the foundation of portable Rust SGX SDK, a
//! set of minimal and battle-tested shared abstractions for the Rust SGX
//! ecosystem. Similar to Rust's libstd, it offers core types, like [`Vec<T>`] and
//! [`Option<T>`], library-defined [operations on language
//! primitives](#primitives), [standard macros](#macros), [I/O] and
//! [multithreading], among [many other things][other].
//!
//! `std` is available to all Rust crates by default, just as if each one
//! contained an `extern crate sgx_tstd as std;` import at the [crate root]. Therefore the
//! standard library can be accessed in [`use`] statements through the path
//! `std`, as in [`use std::env`], or in expressions through the absolute path
//! `::std`, as in [`::std::env::args`].

#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#![no_std]
#![needs_panic_runtime]
#![allow(non_camel_case_types)]
#![allow(unused_must_use)]
#![allow(unused_features)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(deprecated)]
#![allow(unused_assignments)]

#![feature(alloc_error_handler)]
#![feature(panic_unwind)]
#![feature(allocator_api)]
#![feature(allow_internal_unsafe)]
#![feature(allocator_internals)]
#![feature(allow_internal_unstable)]
#![feature(array_error_internals)]
#![feature(asm)]
#![feature(box_syntax)]
#![feature(c_variadic)]
#![feature(cfg_target_has_atomic)]
#![feature(char_error_internals)]
#![feature(concat_idents)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(custom_test_frameworks)]
#![feature(dropck_eyepatch)]
#![feature(fixed_size_array)]
#![feature(fn_traits)]
#![feature(generator_trait)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(hashmap_internals)]
#![feature(int_error_internals)]
#![feature(lang_items)]
#![feature(log_syntax)]
#![feature(maybe_uninit_ref)]
#![feature(needs_panic_runtime)]
#![feature(optin_builtin_traits)]
#![feature(prelude_import)]
#![feature(ptr_internals)]
#![feature(raw)]
#![feature(shrink_to)]
#![feature(rustc_attrs)]
#![feature(slice_concat_ext)]
#![feature(str_internals)]
#![feature(thread_local)]
#![feature(toowned_clone_into)]
#![feature(trace_macros)]
#![feature(try_reserve)]
#![feature(unboxed_closures)]
#![feature(untagged_unions)]
#![feature(unwind_attributes)]
#![feature(slice_patterns)]
#![feature(libc)]
#![feature(panic_internals)]
#![feature(std_internals)]
#![feature(panic_info_message)]
#![feature(unicode_internals)]
#![feature(alloc_layout_extra)]
#![feature(vec_remove_item)]
#![feature(int_error_matching)]
#![feature(drain_filter)]
#![default_lib_allocator]

// Explicitly import the prelude. The compiler uses this same unstable attribute
// to import the prelude implicitly when building crates that depend on std.
#[prelude_import]
#[allow(unused)]
use prelude::v1::*;

// Re-export a few macros from core
pub use core::{assert_eq, assert_ne, debug_assert, debug_assert_eq, debug_assert_ne};
pub use core::{unreachable, unimplemented, write, writeln, r#try, todo};


#[allow(unused_imports)] // macros from `alloc` are not used on all platforms
#[macro_use]
extern crate alloc as alloc_crate;


// We always need an unwinder currently for backtraces
#[allow(unused_extern_crates)]
extern crate sgx_unwind;
#[cfg(feature = "backtrace")]
extern crate sgx_backtrace_sys;
#[cfg(feature = "backtrace")]
extern crate sgx_demangle;
extern crate sgx_alloc;

#[macro_use]
extern crate sgx_types;
pub use sgx_types::{cfg_if, __cfg_if_items, __cfg_if_apply};

#[macro_use]
extern crate sgx_trts;
pub use sgx_trts::{global_ctors_object, global_dtors_object, is_x86_feature_detected, is_cpu_feature_supported};

extern crate sgx_tprotected_fs;
extern crate sgx_libc;
extern crate sgx_tseal;

// The standard macros that are not built-in to the compiler.
#[macro_use]
mod macros;

// The Rust prelude
pub mod prelude;

// Public module declarations and reexports

pub use core::any;
pub use core::cell;
pub use core::clone;
pub use core::cmp;
pub use core::convert;
pub use core::default;
pub use core::hash;
pub use core::intrinsics;
pub use core::iter;
pub use core::marker;
pub use core::mem;
pub use core::ops;
pub use core::ptr;
pub use core::raw;
pub use core::result;
pub use core::option;
pub use core::isize;
pub use core::i8;
pub use core::i16;
pub use core::i32;
pub use core::i64;
pub use core::i128;
pub use core::usize;
pub use core::u8;
pub use core::u16;
pub use core::u32;
pub use core::u64;
pub use core::u128;
pub use core::char;
pub use core::pin;
pub use core::hint;
pub use core::array;
pub use alloc_crate::boxed;
pub use alloc_crate::rc;
pub use alloc_crate::borrow;
pub use alloc_crate::fmt;
pub use alloc_crate::format;
pub use alloc_crate::slice;
pub use alloc_crate::str;
pub use alloc_crate::string;
pub use alloc_crate::vec;

pub mod f32;
pub mod f64;

#[macro_use]
pub mod thread;
pub mod ascii;
pub mod collections;
pub mod env;
pub mod error;
pub mod ffi;
pub mod sgxfs;
#[cfg(feature = "untrusted_fs")]
pub mod fs;
pub mod io;
pub mod net;
pub mod num;
pub mod os;
pub mod panic;
pub mod path;
pub mod sync;
pub mod time;
pub mod enclave;
pub mod untrusted;

pub mod task {
    //! Types and Traits for working with asynchronous tasks.
    #[doc(inline)]
    pub use core::task::*;
}

pub mod future;

// Platform-abstraction modules
#[macro_use]
mod sys_common;
mod sys;

pub mod alloc;

// Private support modules
mod panicking;
mod cpuid;
mod memchr;
#[cfg(not(feature = "untrusted_fs"))]
mod fs;

// The runtime entry point and a few unstable public functions used by the
// compiler
pub mod rt;

#[cfg(feature = "backtrace")]
pub mod backtrace;

pub use cpuid::*;
pub use self::thread::{rsgx_thread_self, rsgx_thread_equal};

#[cfg(debug_assertions)]
pub mod debug;
