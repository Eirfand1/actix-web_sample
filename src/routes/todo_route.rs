use crate::handler::todo_handler;
use actix_web::web;

pub fn configure_todo_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/todos")
                .route("", web::get().to(todo_handler::get_todos))
                .route("/{id}", web::get().to(todo_handler::get_todo)),
        ),
    );
}
