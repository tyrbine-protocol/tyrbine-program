use anchor_lang::error_code;


#[error_code]
pub enum TyrbineError {
    #[msg("Invalid Admin")]
    InvalidAdmin,

    #[msg("Missing Account")]
    MissingAccount,

    #[msg("Invalid Signer")]
    InvalidSigner,

    #[msg("Invalid ATA")]
    InvalidATA,

    #[msg("Invalid Treasury")]
    InvalidTreasury,

    #[msg("Missing SPL Account")]
    MissingSPLAccount,

    #[msg("Invalid LP Amount")]
    InvalidLpAmount,

    #[msg("Divide by zero")]
    DivideByZero,

    #[msg("Overflow")]
    Overflow,

    #[msg("Invalid Pyth Account")]
    InvalidPythAccount,

    #[msg("Slippage greater than permissible")]
    HighSlippage,

    #[msg("Invalid token owner")]
    InvalidTokenOwner,
    
    #[msg("Insufficient liquidity in the pool")]
    InsufficientLiquidity,

    #[msg("Pool A not active")]
    PoolANotActive,

    #[msg("Pool B not active")]
    PoolBNotActive,

    #[msg("Stoptap activated")]
    StoptapActivated,

    #[msg("Price not available")]
    PriceNotAvailable,

    #[msg("High volatility")]
    HighVolatility,

    #[msg("Switch in Off mode")]
    SwitchOff,
}