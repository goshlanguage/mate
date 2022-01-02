use serde::{Deserialize, Serialize};

/// TODO:
///   Add serialize upstream
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MateCandle {
    pub close: f64,
    pub datetime: usize,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub volume: i64,
}
