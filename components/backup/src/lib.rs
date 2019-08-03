// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#![recursion_limit = "200"]
#![allow(unused_imports)]

#[macro_use(
    kv,
    slog_kv,
    slog_trace,
    slog_debug,
    slog_info,
    slog_warn,
    slog_error,
    slog_record,
    slog_b,
    slog_log,
    slog_record_static
)]
extern crate slog;
#[macro_use]
extern crate slog_global;

mod endpoint;
mod service;
mod storage;

pub use endpoint::{Endpoint, Task};
pub use service::Service;
pub use storage::{LocalStorage, Storage};