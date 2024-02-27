//! The `trade` module provides functionality for managing trades within the order_matching_engine.
//!
//! This module defines the `Trade` struct, which represents a single trade transaction,
//! including details of the trade.
//! The module also provides functions and methods for creating, executing, and managing
//! trades, ensuring that trades are executed according to specified parameters and market conditions.
//!
//! Functionality includes:
//! - Creating new trade instances with specific parameters.
//! - Executing trades and updating their status.
//! - Calculating commissions and other trade-related fees.
//! - Handling trade settlement and ensuring proper record-keeping.
//!
//! This module is designed to be used by other parts of the application, such as order management
//! and portfolio management modules, to facilitate the execution and management of trades in a
//! systematic and efficient manner.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

/// Represents a single trade transaction within the order_matching_engine.
///
/// This struct encapsulates all relevant details about a trade.
/// It is used to record and manage trade activities, ensuring accurate tracking and processing
/// of trade data.
#[derive(Debug)]
pub struct Trade {
    /// Exchange on which the trade took place.
    pub exchange: String,
    /// Unique identifier for the trade itself.
    pub trade_id: Uuid,
    /// Volume of the order that has been filled/partially filled through this trade.
    pub trade_volume: Decimal,
    /// Average price at which the trade was executed.
    pub avg_price: Decimal,
    /// Commission charged for executing the trade.
    pub commision: Decimal,
    /// Timestamp of when the trade transaction was executed.
    pub transact_time: DateTime<Utc>,
    /// Symbol of the financial instrument being traded, e.g., stock ticker.
    pub symbol: String,
}

impl Trade {
    /// Creates a new instance of a `Trade`.
    ///
    /// This function initializes a new `Trade` with given details about the order,
    /// client, trade characteristics, and execution information. A unique trade ID
    /// is generated internally for each new trade instance. This function is typically
    /// used to record a new trade transaction in the system after an order has been
    /// successfully executed or processed in some form.
    ///
    /// # Parameters
    /// - `exchange`: A `String` representing the exchange on which the trade took place.
    /// - `trade_volume`: The volume of the order that has been filled by this trade as a `Decimal`.
    /// - `avg_price`: The average price at which the trade was executed as a `Decimal`.
    /// - `commision`: The commission charged for this trade as a `Decimal`.
    /// - `transact_time`: The timestamp of the transaction, represented by `DateTime<Utc>`.
    /// - `symbol`: A `String` representing the symbol of the financial instrument being traded.
    ///
    /// # Returns
    /// Returns a `Trade` instance containing all the provided details and a newly generated unique trade ID.
    pub fn new(
        exchange: String,
        trade_volume: Decimal,
        avg_price: Decimal,
        commision: Decimal,
        transact_time: DateTime<Utc>,
        symbol: String,
    ) -> Trade {
        Trade {
            exchange,
            trade_id: Uuid::new_v4(),
            trade_volume,
            avg_price,
            commision,
            transact_time,
            symbol,
        }
    }
}
