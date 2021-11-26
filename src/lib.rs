mod accounts;
mod auth;
mod router;
mod routes;

use crate::router::route;
use solana_program::entrypoint;

entrypoint!(route);
