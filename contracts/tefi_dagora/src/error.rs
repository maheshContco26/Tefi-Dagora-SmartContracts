use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    #[error("ThreadExists")]
    ThreadExists {},
    #[error("ThreadNotExists")]
    ThreadNotExists {},
    #[error("CommentNotExists")]
    CommentNotExists {},
    #[error("NotEnoughBalance")]
    NotEnoughBalance {},
    #[error("LessFeeAmount")]
    LessFeeAmount {},
    #[error("ConfigNotExists")]
    ConfigNotExists {},


    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
