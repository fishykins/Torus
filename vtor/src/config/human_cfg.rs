use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct HumanCfg {
    /// The minimum width required to fit a human
    width: f32,
    /// The minimum ceiling height required for human habbitation
    height: f32,
}