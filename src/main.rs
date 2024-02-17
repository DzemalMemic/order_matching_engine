use {chrono::Utc, order_matching_engine::order::Order, rust_decimal::Decimal, uuid::Uuid};
use {
    order_matching_engine::models::order_type::OrderType, order_matching_engine::models::side::Side,
};

fn main() {
    // region: create order
    let order = Order {
        order_id: Uuid::new_v4(),
        order_type: OrderType::Limit,
        volume: Decimal::new(100, 2), // 100.00
        price: Decimal::new(1500, 2), // 15.00
        create_time: Utc::now(),
        symbol: String::from("AAPL"),
        side: Side::Bid,
    };

    println!("Created order: {:?}", order);
    // endregion: create order
}
