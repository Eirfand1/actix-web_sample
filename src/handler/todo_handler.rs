use crate::models::todo::{CreateTodo, Todo};
use actix_web::{web, HttpResponse, Responder};

pub async fn get_todo(todo_id: web::Path<i32>) -> impl Responder {
    let todo = Todo {
        id: todo_id.into_inner(),
        task: String::from("joget india"),
    };
    HttpResponse::Ok().json(todo)
}

pub async fn get_todos() -> impl Responder {
    let todos = vec![
        Todo {
            id: 1,
            task: String::from("salto"),
        },
        Todo {
            id: 2,
            task: String::from("akjwkwaj"),
        },
    ];
    HttpResponse::Ok().json(todos)
}
