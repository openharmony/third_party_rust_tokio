use std::time::{Duration, Instant};
use tokio::time::Interval as TokioInterval;

pub struct Interval(TokioInterval);

/// Creates a new [`Interval`] that yields with an interval of `period`. The first tick
/// completes immediately. The default [`TimeoutPolicies`] is [`TimeoutPolicies::Burst`]
pub fn interval(period: Duration) -> Interval {
    Interval(tokio::time::interval(period))
}

/// Creates new [`Interval`] that yields with interval of `period` with the
/// first tick completing at `start`. The default [`TimeoutPolicies`] is
/// [`TimeoutPolicies::Burst`]
pub fn interval_at(start: Instant, period: Duration) -> Interval {
    Interval(tokio::time::interval_at(start.into(), period))
}

impl Interval {
    /// Waits until the next instant is reached
    pub async fn tick(&mut self) -> Instant {
        self.0.tick().await.into_std()
    }

    /// Resets the `Interval` from now on
    pub fn reset(&mut self) {
        self.0.reset()
    }

    /// Gets the period of the `Interval`
    pub fn period(&self) -> Duration {
        self.0.period()
    }
}
