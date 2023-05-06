pub mod builder;
pub mod join_handle;

/// Task priority level, ranges from high to low
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PriorityLevel {
    AbsHigh,
    High,
    Low,
    AbsLow,
}
