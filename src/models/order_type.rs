//! This module contains the OrderType enum.

/// Debug derives allows us to print whole structure.
/// PartialEq derives allows us to compare two OrderType
/// OrderType is an enum that represents the type of order.
/// For example, a limit order is an order to buy or sell a stock at a specific price or better.
/// Market order is an order to buy or sell a stock at the best available price.
/// We have also more types of orders, such as: Stop, StopLimit, TrailingStop, etc. But for this example we will use only Limit and Market.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum OrderType {
    Limit,
    Market,
    None
}
