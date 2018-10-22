use actix_web::{HttpRequest, Responder};

use super::super::global_state::State;

pub fn index(req: &HttpRequest<State>) -> impl Responder {
    let state = req.state();
    "Under construction"
}