use serde::{Deserialize, Serialize};

use crate::ids::{SourceId, TokenId};

/// A PaymentSourceParams represents all of the supported ways that can
/// be used.
///
/// This includes creating a new customer with a payment method or creating
/// a payment method directly for a customer via `Customer::attach_source`.
/// Not to be confused with `SourceParams` which is used by `Source::create`
/// to create a source that is not necessarily attached to a customer.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PaymentSourceParams {
    /// Creates a payment method (e.g. card or bank account) from tokenized data,
    /// using a token typically received from Stripe Elements.
    Token(TokenId),

    /// Attach an existing source to an existing customer or
    /// create a new customer from an existing source.
    Source(SourceId),
}
