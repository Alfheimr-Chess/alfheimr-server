#[derive(Debug, thiserror::Error, displaydoc::Display)]
/// General error type for Álfheimr
pub enum Error {
    /// Failed to parse Parlett
    ParlettParse,
    /// Failed to parse FFen
    FFenParse,
    /// Invalid move given to board
    InvalidMove,
    /// Failed to evaluate rhai code
    RhaiError(#[from] Box<rhai::plugin::EvalAltResult>)
}
