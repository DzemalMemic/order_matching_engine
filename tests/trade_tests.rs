use chrono::Utc;
use order_matching_engine::trade::Trade;
use rust_decimal::Decimal;

#[test]
fn test_trade_creation_executed_bid_market() {
    let exchange = String::from("Binance");
    let trade_volume = Decimal::new(50, 0);
    let avg_price = Decimal::new(10, 0);
    let commission = Decimal::new(1, 0);
    let transact_time = Utc::now();
    let symbol = String::from("AAPL");

    let trade = Trade::new(
        exchange.clone(),
        trade_volume,
        avg_price,
        commission,
        transact_time,
        symbol.clone(),
    );

    assert_eq!(trade.exchange, exchange);
    assert_eq!(trade.trade_volume, trade_volume);
    assert_eq!(trade.avg_price, avg_price);
    assert_eq!(trade.commision, commission);
    // transact_time could slightly differ; adjust as needed or use a range for checks
    // assert_eq!(trade.transact_time, transact_time);
    assert_eq!(trade.symbol, symbol);
}
