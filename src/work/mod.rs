// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Employee work schedules expressed as local calendar dates and time ranges.

mod local_time_range;
mod work_schedule;

pub use local_time_range::LocalTimeRange;
pub use work_schedule::WorkSchedule;
