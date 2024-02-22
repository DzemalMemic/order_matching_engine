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

use crate::models::{order_type::OrderType, side::Side};

/// Represents the status of a trade within the order_matching_engine.
///
/// This enum is used to track the current state of a trade, from its initiation
/// to its final state, indicating whether the trade was successfully executed,
/// cancelled before execution, or rejected due to issues such as validation failures
/// or system errors.
#[derive(Debug, PartialEq)]
pub enum TradeStatus {
    /// Indicates that the trade has been successfully executed.
    Executed,
    /// Indicates that the trade was cancelled before it could be executed.
    Cancelled,
    /// Indicates that the trade was rejected and not executed due to an error
    /// or validation failure.
    Rejected,
    /// Indicates a default or unknown state
    None,
}

/// Represents a single trade transaction within the order_matching_engine.
///
/// This struct encapsulates all relevant details about a trade, including identifiers,
/// trade and order types, volumes, execution price, commission, and the time of the transaction.
/// It is used to record and manage trade activities, ensuring accurate tracking and processing
/// of trade data.
#[derive(Debug)]
pub struct Trade {
    /// Unique identifier for the order associated with this trade.
    pub order_id: Uuid,
    /// Unique identifier for the client initiating the trade.
    pub client_id: Uuid,
    /// Unique identifier for the trade itself.
    pub trade_id: Uuid,
    /// Type of order that resulted in this trade, e.g., Market or Limit.
    pub order_type: OrderType,
    /// Total volume of the order.
    pub order_volume: Decimal,
    /// Volume of the order that has been filled through this trade.
    pub filled_volume: Decimal,
    /// Average price at which the trade was executed.
    pub avg_price: Decimal,
    /// Commission charged for executing the trade.
    pub commision: Decimal,
    /// Timestamp of when the trade transaction was executed.
    pub transact_time: DateTime<Utc>,
    /// Symbol of the financial instrument being traded, e.g., stock ticker.
    pub symbol: String,
    /// Side of the trade, indicating whether it was a buy or sell order.
    pub side: Side,
    /// Current status of the trade, indicating if it was executed, cancelled, or rejected.
    pub status: TradeStatus,
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
    /// - `order_id`: A `Uuid` representing the unique identifier of the order associated with this trade.
    /// - `client_id`: A `Uuid` representing the unique identifier of the client who placed the order.
    /// - `order_type`: The type of the order (e.g., Market, Limit) encapsulated by the `OrderType` enum.
    /// - `order_volume`: The total volume of the order as a `Decimal`.
    /// - `filled_volume`: The volume of the order that has been filled by this trade as a `Decimal`.
    /// - `avg_price`: The average price at which the trade was executed as a `Decimal`.
    /// - `commision`: The commission charged for this trade as a `Decimal`.
    /// - `transact_time`: The timestamp of the transaction, represented by `DateTime<Utc>`.
    /// - `symbol`: A `String` representing the symbol of the financial instrument being traded.
    /// - `side`: The side of the trade (Buy or Sell), represented by the `Side` enum.
    /// - `status`: The current status of the trade (Executed, Cancelled, Rejected) encapsulated by the `TradeStatus` enum.
    ///
    /// # Returns
    /// Returns a `Trade` instance containing all the provided details and a newly generated unique trade ID.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        order_id: Uuid,
        client_id: Uuid,
        order_type: OrderType,
        order_volume: Decimal,
        filled_volume: Decimal,
        avg_price: Decimal,
        commision: Decimal,
        transact_time: DateTime<Utc>,
        symbol: String,
        side: Side,
        status: TradeStatus,
    ) -> Trade {
        Trade {
            order_id,
            client_id,
            trade_id: Uuid::new_v4(),
            order_type,
            order_volume,
            filled_volume,
            avg_price,
            commision,
            transact_time,
            symbol,
            side,
            status,
        }
    }
}
