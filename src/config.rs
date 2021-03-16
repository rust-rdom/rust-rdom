#[derive(Clone, Debug)]
pub struct ScreenMetrics {
    inner_width: u16,
    inner_height: u16
}

impl Default for ScreenMetrics {
    fn default() -> ScreenMetrics {
        ScreenMetrics{
            inner_width: 1500,
            inner_height: 1000,
        }
    }
}