use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};

use crate::routes::app_error::AppError;

use super::AppState;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Degen {
    pub balance: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u32,
}

pub async fn address_balance(
    Extension(app_state): Extension<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<Degen>, AppError> {
    let balances = app_state.balances.lock().unwrap();

    match balances.get(&address) {
        Some(degen) => Ok(Json(Degen {
            balance: degen.balance,
        })),
        None => Err(AppError::new(StatusCode::NOT_FOUND, "Address not found")),
    }
}

pub async fn send_to_address(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(transaction): Json<Transaction>,
) -> Result<Json<Degen>, AppError> {
    let mut balances = app_state.balances.lock().unwrap();

    match balances.get(&transaction.sender) {
        Some(sender) if sender.balance >= transaction.amount => {
            let sender_balance = sender.balance - transaction.amount;

            let recipient_balance = balances
                .entry(transaction.recipient.to_string())
                .or_insert(Degen { balance: 0 });
            recipient_balance.balance += transaction.amount;

            balances.insert(
                transaction.sender.clone(),
                Degen {
                    balance: sender_balance,
                },
            );

            Ok(Json(Degen {
                balance: sender_balance,
            }))
        }
        Some(_) => Err(AppError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Insufficient Funds",
        )),
        None => Err(AppError::new(
            StatusCode::NOT_FOUND,
            "Sender address not found",
        )),
    }
}
