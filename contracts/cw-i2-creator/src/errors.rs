//! The errors that can be thrown for this boolean contract, including demonstration ones.

use cosmwasm_std::{Addr, Instantiate2AddressError, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    /// Example error with placeholder
    #[error("Unauthorized, method can only be called by {:?}", owner)]
    Unauthorized { owner: Addr },

    /// Nice way of borrowing an error from elsewhere
    #[error("{0}")]
    Instantiate2Address(#[from] Instantiate2AddressError),

    /// This contract can only be instantiated once
    #[error("Already instantiated bro")]
    AlreadyInstantiated,
}
