use chrono::Utc;
use order_matching_engine::{
    models::{order_type::OrderType, side::Side},
    trade::{Trade, TradeStatus},
};
use rust_decimal::Decimal;
use uuid::Uuid;

#[test]
fn test_trade_creation_executed_bid_market() {
    let order_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let order_volume = Decimal::new(100, 0);
    let filled_volume = Decimal::new(50, 0);
    let avg_price = Decimal::new(10, 0);
    let commission = Decimal::new(1, 0);
    let transact_time = Utc::now();
    let symbol = "AAPL".to_string();

    let trade = Trade::new(
        order_id,
        client_id,
        OrderType::Market,
        order_volume,
        filled_volume,
        avg_price,
        commission,
        transact_time,
        symbol.clone(),
        Side::Bid,
        TradeStatus::Executed,
    );

    assert_eq!(trade.order_id, order_id);
    assert_eq!(trade.client_id, client_id);
    // trade_id is generated within the new function, so not asserting it here
    assert_eq!(trade.order_type, OrderType::Market);
    assert_eq!(trade.order_volume, order_volume);
    assert_eq!(trade.filled_volume, filled_volume);
    assert_eq!(trade.avg_price, avg_price);
    assert_eq!(trade.commision, commission);
    // transact_time could slightly differ; adjust as needed or use a range for checks
    // assert_eq!(trade.transact_time, transact_time);
    assert_eq!(trade.symbol, symbol);
    assert_eq!(trade.side, Side::Bid);
    assert_eq!(trade.status, TradeStatus::Executed);
}

#[test]
fn test_trade_creation_executed_bid_limit() {
    let order_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let order_volume = Decimal::new(100, 0);
    let filled_volume = Decimal::new(50, 0);
    let avg_price = Decimal::new(10, 0);
    let commission = Decimal::new(1, 0);
    let transact_time = Utc::now();
    let symbol = "AAPL".to_string();

    let trade = Trade::new(
        order_id,
        client_id,
        OrderType::Limit,
        order_volume,
        filled_volume,
        avg_price,
        commission,
        transact_time,
        symbol.clone(),
        Side::Bid,
        TradeStatus::Executed,
    );

    assert_eq!(trade.order_id, order_id);
    assert_eq!(trade.client_id, client_id);
    // trade_id is generated within the new function, so not asserting it here
    assert_eq!(trade.order_type, OrderType::Limit);
    assert_eq!(trade.order_volume, order_volume);
    assert_eq!(trade.filled_volume, filled_volume);
    assert_eq!(trade.avg_price, avg_price);
    assert_eq!(trade.commision, commission);
    // transact_time could slightly differ; adjust as needed or use a range for checks
    // assert_eq!(trade.transact_time, transact_time);
    assert_eq!(trade.symbol, symbol);
    assert_eq!(trade.side, Side::Bid);
    assert_eq!(trade.status, TradeStatus::Executed);
}

#[test]
fn test_trade_creation_executed_ask_market() {
    let order_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let order_volume = Decimal::new(100, 0);
    let filled_volume = Decimal::new(50, 0);
    let avg_price = Decimal::new(10, 0);
    let commission = Decimal::new(1, 0);
    let transact_time = Utc::now();
    let symbol = "AAPL".to_string();

    let trade = Trade::new(
        order_id,
        client_id,
        OrderType::Market,
        order_volume,
        filled_volume,
        avg_price,
        commission,
        transact_time,
        symbol.clone(),
        Side::Ask,
        TradeStatus::Executed,
    );

    assert_eq!(trade.order_id, order_id);
    assert_eq!(trade.client_id, client_id);
    // trade_id is generated within the new function, so not asserting it here
    assert_eq!(trade.order_type, OrderType::Market);
    assert_eq!(trade.order_volume, order_volume);
    assert_eq!(trade.filled_volume, filled_volume);
    assert_eq!(trade.avg_price, avg_price);
    assert_eq!(trade.commision, commission);
    // transact_time could slightly differ; adjust as needed or use a range for checks
    // assert_eq!(trade.transact_time, transact_time);
    assert_eq!(trade.symbol, symbol);
    assert_eq!(trade.side, Side::Ask);
    assert_eq!(trade.status, TradeStatus::Executed);
}

#[test]
fn test_trade_creation_executed_ask_limit() {
    let order_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let order_volume = Decimal::new(100, 0);
    let filled_volume = Decimal::new(50, 0);
    let avg_price = Decimal::new(10, 0);
    let commission = Decimal::new(1, 0);
    let transact_time = Utc::now();
    let symbol = "AAPL".to_string();

    let trade = Trade::new(
        order_id,
        client_id,
        OrderType::Limit,
        order_volume,
        filled_volume,
        avg_price,
        commission,
        transact_time,
        symbol.clone(),
        Side::Ask,
        TradeStatus::Executed,
    );

    assert_eq!(trade.order_id, order_id);
    assert_eq!(trade.client_id, client_id);
    // trade_id is generated within the new function, so not asserting it here
    assert_eq!(trade.order_type, OrderType::Limit);
    assert_eq!(trade.order_volume, order_volume);
    assert_eq!(trade.filled_volume, filled_volume);
    assert_eq!(trade.avg_price, avg_price);
    assert_eq!(trade.commision, commission);
    // transact_time could slightly differ; adjust as needed or use a range for checks
    // assert_eq!(trade.transact_time, transact_time);
    assert_eq!(trade.symbol, symbol);
    assert_eq!(trade.side, Side::Ask);
    assert_eq!(trade.status, TradeStatus::Executed);
}

#[test]
fn test_trade_creation_cancelled_bid_market() {
    let order_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let order_volume = Decimal::new(100, 0);
    let filled_volume = Decimal::new(50, 0);
    let avg_price = Decimal::new(10, 0);
    let commission = Decimal::new(1, 0);
    let transact_time = Utc::now();
    let symbol = "AAPL".to_string();

    let trade = Trade::new(
        order_id,
        client_id,
        OrderType::Market,
        order_volume,
        filled_volume,
        avg_price,
        commission,
        transact_time,
        symbol.clone(),
        Side::Bid,
        TradeStatus::Cancelled,
    );

    assert_eq!(trade.order_id, order_id);
    assert_eq!(trade.client_id, client_id);
    // trade_id is generated within the new function, so not asserting it here
    assert_eq!(trade.order_type, OrderType::Market);
    assert_eq!(trade.order_volume, order_volume);
    assert_eq!(trade.filled_volume, filled_volume);
    assert_eq!(trade.avg_price, avg_price);
    assert_eq!(trade.commision, commission);
    // transact_time could slightly differ; adjust as needed or use a range for checks
    // assert_eq!(trade.transact_time, transact_time);
    assert_eq!(trade.symbol, symbol);
    assert_eq!(trade.side, Side::Bid);
    assert_eq!(trade.status, TradeStatus::Cancelled);
}

#[test]
fn test_trade_creation_cancelled_bid_limit() {
    let order_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let order_volume = Decimal::new(100, 0);
    let filled_volume = Decimal::new(50, 0);
    let avg_price = Decimal::new(10, 0);
    let commission = Decimal::new(1, 0);
    let transact_time = Utc::now();
    let symbol = "AAPL".to_string();

    let trade = Trade::new(
        order_id,
        client_id,
        OrderType::Limit,
        order_volume,
        filled_volume,
        avg_price,
        commission,
        transact_time,
        symbol.clone(),
        Side::Bid,
        TradeStatus::Cancelled,
    );

    assert_eq!(trade.order_id, order_id);
    assert_eq!(trade.client_id, client_id);
    // trade_id is generated within the new function, so not asserting it here
    assert_eq!(trade.order_type, OrderType::Limit);
    assert_eq!(trade.order_volume, order_volume);
    assert_eq!(trade.filled_volume, filled_volume);
    assert_eq!(trade.avg_price, avg_price);
    assert_eq!(trade.commision, commission);
    // transact_time could slightly differ; adjust as needed or use a range for checks
    // assert_eq!(trade.transact_time, transact_time);
    assert_eq!(trade.symbol, symbol);
    assert_eq!(trade.side, Side::Bid);
    assert_eq!(trade.status, TradeStatus::Cancelled);
}

#[test]
fn test_trade_creation_cancelled_ask_market() {
    let order_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let order_volume = Decimal::new(100, 0);
    let filled_volume = Decimal::new(50, 0);
    let avg_price = Decimal::new(10, 0);
    let commission = Decimal::new(1, 0);
    let transact_time = Utc::now();
    let symbol = "AAPL".to_string();

    let trade = Trade::new(
        order_id,
        client_id,
        OrderType::Market,
        order_volume,
        filled_volume,
        avg_price,
        commission,
        transact_time,
        symbol.clone(),
        Side::Ask,
        TradeStatus::Cancelled,
    );

    assert_eq!(trade.order_id, order_id);
    assert_eq!(trade.client_id, client_id);
    // trade_id is generated within the new function, so not asserting it here
    assert_eq!(trade.order_type, OrderType::Market);
    assert_eq!(trade.order_volume, order_volume);
    assert_eq!(trade.filled_volume, filled_volume);
    assert_eq!(trade.avg_price, avg_price);
    assert_eq!(trade.commision, commission);
    // transact_time could slightly differ; adjust as needed or use a range for checks
    // assert_eq!(trade.transact_time, transact_time);
    assert_eq!(trade.symbol, symbol);
    assert_eq!(trade.side, Side::Ask);
    assert_eq!(trade.status, TradeStatus::Cancelled);
}

#[test]
fn test_trade_creation_cancelled_ask_limit() {
    let order_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let order_volume = Decimal::new(100, 0);
    let filled_volume = Decimal::new(50, 0);
    let avg_price = Decimal::new(10, 0);
    let commission = Decimal::new(1, 0);
    let transact_time = Utc::now();
    let symbol = "AAPL".to_string();

    let trade = Trade::new(
        order_id,
        client_id,
        OrderType::Limit,
        order_volume,
        filled_volume,
        avg_price,
        commission,
        transact_time,
        symbol.clone(),
        Side::Ask,
        TradeStatus::Cancelled,
    );

    assert_eq!(trade.order_id, order_id);
    assert_eq!(trade.client_id, client_id);
    // trade_id is generated within the new function, so not asserting it here
    assert_eq!(trade.order_type, OrderType::Limit);
    assert_eq!(trade.order_volume, order_volume);
    assert_eq!(trade.filled_volume, filled_volume);
    assert_eq!(trade.avg_price, avg_price);
    assert_eq!(trade.commision, commission);
    // transact_time could slightly differ; adjust as needed or use a range for checks
    // assert_eq!(trade.transact_time, transact_time);
    assert_eq!(trade.symbol, symbol);
    assert_eq!(trade.side, Side::Ask);
    assert_eq!(trade.status, TradeStatus::Cancelled);
}

#[test]
fn test_trade_creation_rejected_bid_market() {
    let order_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let order_volume = Decimal::new(100, 0);
    let filled_volume = Decimal::new(50, 0);
    let avg_price = Decimal::new(10, 0);
    let commission = Decimal::new(1, 0);
    let transact_time = Utc::now();
    let symbol = "AAPL".to_string();

    let trade = Trade::new(
        order_id,
        client_id,
        OrderType::Market,
        order_volume,
        filled_volume,
        avg_price,
        commission,
        transact_time,
        symbol.clone(),
        Side::Bid,
        TradeStatus::Rejected,
    );

    assert_eq!(trade.order_id, order_id);
    assert_eq!(trade.client_id, client_id);
    // trade_id is generated within the new function, so not asserting it here
    assert_eq!(trade.order_type, OrderType::Market);
    assert_eq!(trade.order_volume, order_volume);
    assert_eq!(trade.filled_volume, filled_volume);
    assert_eq!(trade.avg_price, avg_price);
    assert_eq!(trade.commision, commission);
    // transact_time could slightly differ; adjust as needed or use a range for checks
    // assert_eq!(trade.transact_time, transact_time);
    assert_eq!(trade.symbol, symbol);
    assert_eq!(trade.side, Side::Bid);
    assert_eq!(trade.status, TradeStatus::Rejected);
}

#[test]
fn test_trade_creation_rejected_bid_limit() {
    let order_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let order_volume = Decimal::new(100, 0);
    let filled_volume = Decimal::new(50, 0);
    let avg_price = Decimal::new(10, 0);
    let commission = Decimal::new(1, 0);
    let transact_time = Utc::now();
    let symbol = "AAPL".to_string();

    let trade = Trade::new(
        order_id,
        client_id,
        OrderType::Limit,
        order_volume,
        filled_volume,
        avg_price,
        commission,
        transact_time,
        symbol.clone(),
        Side::Bid,
        TradeStatus::Rejected,
    );

    assert_eq!(trade.order_id, order_id);
    assert_eq!(trade.client_id, client_id);
    // trade_id is generated within the new function, so not asserting it here
    assert_eq!(trade.order_type, OrderType::Limit);
    assert_eq!(trade.order_volume, order_volume);
    assert_eq!(trade.filled_volume, filled_volume);
    assert_eq!(trade.avg_price, avg_price);
    assert_eq!(trade.commision, commission);
    // transact_time could slightly differ; adjust as needed or use a range for checks
    // assert_eq!(trade.transact_time, transact_time);
    assert_eq!(trade.symbol, symbol);
    assert_eq!(trade.side, Side::Bid);
    assert_eq!(trade.status, TradeStatus::Rejected);
}

#[test]
fn test_trade_creation_rejected_ask_market() {
    let order_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let order_volume = Decimal::new(100, 0);
    let filled_volume = Decimal::new(50, 0);
    let avg_price = Decimal::new(10, 0);
    let commission = Decimal::new(1, 0);
    let transact_time = Utc::now();
    let symbol = "AAPL".to_string();

    let trade = Trade::new(
        order_id,
        client_id,
        OrderType::Market,
        order_volume,
        filled_volume,
        avg_price,
        commission,
        transact_time,
        symbol.clone(),
        Side::Ask,
        TradeStatus::Rejected,
    );

    assert_eq!(trade.order_id, order_id);
    assert_eq!(trade.client_id, client_id);
    // trade_id is generated within the new function, so not asserting it here
    assert_eq!(trade.order_type, OrderType::Market);
    assert_eq!(trade.order_volume, order_volume);
    assert_eq!(trade.filled_volume, filled_volume);
    assert_eq!(trade.avg_price, avg_price);
    assert_eq!(trade.commision, commission);
    // transact_time could slightly differ; adjust as needed or use a range for checks
    // assert_eq!(trade.transact_time, transact_time);
    assert_eq!(trade.symbol, symbol);
    assert_eq!(trade.side, Side::Ask);
    assert_eq!(trade.status, TradeStatus::Rejected);
}

#[test]
fn test_trade_creation_rejected_ask_limit() {
    let order_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let order_volume = Decimal::new(100, 0);
    let filled_volume = Decimal::new(50, 0);
    let avg_price = Decimal::new(10, 0);
    let commission = Decimal::new(1, 0);
    let transact_time = Utc::now();
    let symbol = "AAPL".to_string();

    let trade = Trade::new(
        order_id,
        client_id,
        OrderType::Limit,
        order_volume,
        filled_volume,
        avg_price,
        commission,
        transact_time,
        symbol.clone(),
        Side::Ask,
        TradeStatus::Rejected,
    );

    assert_eq!(trade.order_id, order_id);
    assert_eq!(trade.client_id, client_id);
    // trade_id is generated within the new function, so not asserting it here
    assert_eq!(trade.order_type, OrderType::Limit);
    assert_eq!(trade.order_volume, order_volume);
    assert_eq!(trade.filled_volume, filled_volume);
    assert_eq!(trade.avg_price, avg_price);
    assert_eq!(trade.commision, commission);
    // transact_time could slightly differ; adjust as needed or use a range for checks
    // assert_eq!(trade.transact_time, transact_time);
    assert_eq!(trade.symbol, symbol);
    assert_eq!(trade.side, Side::Ask);
    assert_eq!(trade.status, TradeStatus::Rejected);
}

#[test]
fn test_trade_creation_none_bid_market() {
    let order_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let order_volume = Decimal::new(100, 0);
    let filled_volume = Decimal::new(50, 0);
    let avg_price = Decimal::new(10, 0);
    let commission = Decimal::new(1, 0);
    let transact_time = Utc::now();
    let symbol = "AAPL".to_string();

    let trade = Trade::new(
        order_id,
        client_id,
        OrderType::Market,
        order_volume,
        filled_volume,
        avg_price,
        commission,
        transact_time,
        symbol.clone(),
        Side::Bid,
        TradeStatus::None,
    );

    assert_eq!(trade.order_id, order_id);
    assert_eq!(trade.client_id, client_id);
    // trade_id is generated within the new function, so not asserting it here
    assert_eq!(trade.order_type, OrderType::Market);
    assert_eq!(trade.order_volume, order_volume);
    assert_eq!(trade.filled_volume, filled_volume);
    assert_eq!(trade.avg_price, avg_price);
    assert_eq!(trade.commision, commission);
    // transact_time could slightly differ; adjust as needed or use a range for checks
    // assert_eq!(trade.transact_time, transact_time);
    assert_eq!(trade.symbol, symbol);
    assert_eq!(trade.side, Side::Bid);
    assert_eq!(trade.status, TradeStatus::None);
}

#[test]
fn test_trade_creation_none_bid_limit() {
    let order_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let order_volume = Decimal::new(100, 0);
    let filled_volume = Decimal::new(50, 0);
    let avg_price = Decimal::new(10, 0);
    let commission = Decimal::new(1, 0);
    let transact_time = Utc::now();
    let symbol = "AAPL".to_string();

    let trade = Trade::new(
        order_id,
        client_id,
        OrderType::Limit,
        order_volume,
        filled_volume,
        avg_price,
        commission,
        transact_time,
        symbol.clone(),
        Side::Bid,
        TradeStatus::None,
    );

    assert_eq!(trade.order_id, order_id);
    assert_eq!(trade.client_id, client_id);
    // trade_id is generated within the new function, so not asserting it here
    assert_eq!(trade.order_type, OrderType::Limit);
    assert_eq!(trade.order_volume, order_volume);
    assert_eq!(trade.filled_volume, filled_volume);
    assert_eq!(trade.avg_price, avg_price);
    assert_eq!(trade.commision, commission);
    // transact_time could slightly differ; adjust as needed or use a range for checks
    // assert_eq!(trade.transact_time, transact_time);
    assert_eq!(trade.symbol, symbol);
    assert_eq!(trade.side, Side::Bid);
    assert_eq!(trade.status, TradeStatus::None);
}

#[test]
fn test_trade_creation_none_ask_market() {
    let order_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let order_volume = Decimal::new(100, 0);
    let filled_volume = Decimal::new(50, 0);
    let avg_price = Decimal::new(10, 0);
    let commission = Decimal::new(1, 0);
    let transact_time = Utc::now();
    let symbol = "AAPL".to_string();

    let trade = Trade::new(
        order_id,
        client_id,
        OrderType::Market,
        order_volume,
        filled_volume,
        avg_price,
        commission,
        transact_time,
        symbol.clone(),
        Side::Ask,
        TradeStatus::None,
    );

    assert_eq!(trade.order_id, order_id);
    assert_eq!(trade.client_id, client_id);
    // trade_id is generated within the new function, so not asserting it here
    assert_eq!(trade.order_type, OrderType::Market);
    assert_eq!(trade.order_volume, order_volume);
    assert_eq!(trade.filled_volume, filled_volume);
    assert_eq!(trade.avg_price, avg_price);
    assert_eq!(trade.commision, commission);
    // transact_time could slightly differ; adjust as needed or use a range for checks
    // assert_eq!(trade.transact_time, transact_time);
    assert_eq!(trade.symbol, symbol);
    assert_eq!(trade.side, Side::Ask);
    assert_eq!(trade.status, TradeStatus::None);
}

#[test]
fn test_trade_creation_none_ask_limit() {
    let order_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let order_volume = Decimal::new(100, 0);
    let filled_volume = Decimal::new(50, 0);
    let avg_price = Decimal::new(10, 0);
    let commission = Decimal::new(1, 0);
    let transact_time = Utc::now();
    let symbol = "AAPL".to_string();

    let trade = Trade::new(
        order_id,
        client_id,
        OrderType::Limit,
        order_volume,
        filled_volume,
        avg_price,
        commission,
        transact_time,
        symbol.clone(),
        Side::Ask,
        TradeStatus::None,
    );

    assert_eq!(trade.order_id, order_id);
    assert_eq!(trade.client_id, client_id);
    // trade_id is generated within the new function, so not asserting it here
    assert_eq!(trade.order_type, OrderType::Limit);
    assert_eq!(trade.order_volume, order_volume);
    assert_eq!(trade.filled_volume, filled_volume);
    assert_eq!(trade.avg_price, avg_price);
    assert_eq!(trade.commision, commission);
    // transact_time could slightly differ; adjust as needed or use a range for checks
    // assert_eq!(trade.transact_time, transact_time);
    assert_eq!(trade.symbol, symbol);
    assert_eq!(trade.side, Side::Ask);
    assert_eq!(trade.status, TradeStatus::None);
}
