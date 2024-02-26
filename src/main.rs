use order_matching_engine::trade::Trade;

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

    // region: create trade
    let trade = Trade::new(
        String::from("Binance"),
        Decimal::new(50, 2),  // 50.00
        Decimal::new(385, 2), // 385.00
        Decimal::new(1, 2),   // 1.00
        Utc::now(),
        String::from("AAPL"),
    );

    println!("Created trade: {:?}", trade);
    // endregion: create trade
}
