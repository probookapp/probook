pub mod auth;
pub mod license;
pub mod clients;
pub mod products;
pub mod quotes;
pub mod invoices;
pub mod payments;
pub mod settings;
pub mod expenses;
pub mod suppliers;
pub mod product_suppliers;
pub mod delivery_notes;
pub mod client_contacts;
pub mod reminders;
pub mod reports;
pub mod alerts;

use axum::{middleware, Router};
use std::sync::Arc;

use crate::AppState;
use crate::middleware::auth::auth_middleware;
use crate::middleware::license::license_middleware;

pub fn api_router(state: Arc<AppState>) -> Router {
    let public_routes = Router::new()
        .merge(auth::public_routes())
        .merge(license::public_routes());

    let protected_routes = Router::new()
        .merge(auth::protected_routes())
        .merge(clients::routes())
        .merge(products::routes())
        .merge(quotes::routes())
        .merge(invoices::routes())
        .merge(payments::routes())
        .merge(settings::routes())
        .merge(expenses::routes())
        .merge(suppliers::routes())
        .merge(product_suppliers::routes())
        .merge(delivery_notes::routes())
        .merge(client_contacts::routes())
        .merge(reminders::routes())
        .merge(reports::routes())
        .merge(alerts::routes())
        .layer(middleware::from_fn(license_middleware))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
}
