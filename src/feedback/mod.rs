// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Feedback cases, their action history, and the rules governing transitions.

#[allow(clippy::module_inception)]
mod feedback;
mod feedback_action;
mod feedback_processing_rule;
mod feedback_rating;
mod feedback_status;
mod feedback_track;
mod feedback_type;

pub use feedback::Feedback;
pub use feedback_action::FeedbackAction;
pub use feedback_processing_rule::FeedbackProcessingRule;
pub use feedback_rating::FeedbackRating;
pub use feedback_status::FeedbackStatus;
pub use feedback_track::FeedbackTrack;
pub use feedback_type::FeedbackType;
