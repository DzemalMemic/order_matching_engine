
use chrono::DateTime;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::Utc;
use crate::models::order_type::OrderType;
use crate::models::side::Side;

/// Debug allows us to print whole structure.
/// Note that any other field should derive Debug
#[derive(Debug)]
pub struct Order {
    /// Unique id
    pub order_id: Uuid,
    pub order_type: OrderType,
    pub volume: Decimal,
    pub price: Decimal,
    pub create_time: DateTime<Utc>,
    pub symbol: String,
    pub side: Side
}

impl Order {
    pub fn new(order_type: OrderType, volume: Decimal, price: Decimal, symbol: String, side: Side) -> Order {
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

