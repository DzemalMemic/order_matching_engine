//! Side module. It contains the Side enum that represents the side of the order. It can be Bid or Ask.

// Debug derives the trait Debug which allows us to print whole structure.
#[derive(Debug)]
// PartialEq derives the trait PartialEq which allows us to compare two structures.
#[derive(PartialEq)]
// Enum Side is used to define the side of the order. It can be Bid or Ask.
/// Order side determines whether the order is a buy or sell order.
/// Buy orders are called also "Bid" and sell orders are called "Ask".
pub enum Side {
    Bid,
    Ask,
    None,
}
