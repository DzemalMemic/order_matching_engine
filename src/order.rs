//! This module contains the Order structure that represents an order in the stock exchange. It has a unique id, order type, volume, price, create time, symbol, and side.
use crate::models::order_type::OrderType;
use crate::models::side::Side;
use chrono::DateTime;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

// Debug allows us to print whole structure.
// Note that any other field should derive Debug
#[derive(Debug)]
pub struct Order {
    /// Unique id of the order. It is generated using the UUID version 4 algorithm. It is a 128-bit number that is unique to the order.
    pub order_id: Uuid,
    /// Order type can be Market or Limit. Market orders are orders to buy or sell a stock immediately at the best available current price. Limit orders are orders to buy or sell a stock at a specific price or better.
    pub order_type: OrderType,
    /// Volume is the number of shares or contracts to buy or sell.
    pub volume: Decimal,
    /// Price is the price at which the order is to be executed. If the order is a market order, this field is ignored.
    pub price: Decimal,
    /// Create time is the time when the order was created. It is in UTC time.
    pub create_time: DateTime<Utc>,
    /// Symbol is the stock symbol of the order. For example, AAPL for Apple Inc.
    pub symbol: String,
    /// Side is the side of the order. It can be Bid or Ask
    pub side: Side,
}

// Implementation of Order structure with new function that creates a new Order instance with the given parameters.
/// Order is a structure that represents an order in the stock exchange. It has a unique id, order type, volume, price, create time, symbol, and side.
/// Symbol is the stock symbol of the order. For example, AAPL for Apple Inc.
/// Side is the side of the order. It can be Bid or Ask.
impl Order {
    pub fn new(
        order_type: OrderType,
        volume: Decimal,
        price: Decimal,
        symbol: String,
        side: Side,
    ) -> Order {
        Order {
            order_id: Uuid::new_v4(),
            order_type,
            volume,
            price,
            create_time: Utc::now(),
            symbol,
            side,
        }
    }
}
