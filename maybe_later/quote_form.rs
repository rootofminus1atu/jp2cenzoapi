use serde::{Deserialize, Serialize};
use crate::api::quotes::Quote;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct QuoteForm {
    pub text: String
}