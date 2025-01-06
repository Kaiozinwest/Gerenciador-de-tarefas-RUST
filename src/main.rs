mod tarefas;
mod persistencia;
mod html;

use crate::tarefas::*;
use crate::persistencia::*;
use crate::html::*;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct AppState {
    tarefas: Mutex<Vec<Tarefa>>,
}

#[actix_web::get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let tarefas = data.tarefas.lock().unwrap();
    gerar_html(&tarefas);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(std::fs::read_to_string("index.html").unwrap())
}

#[actix_web::post("/adicionar")]
async fn adicionar(data: web::Data<AppState>, form: web::Form<TarefaForm>) -> impl Responder {
    let mut tarefas = data.tarefas.lock().unwrap();
    let id = tarefas.len() + 1;
    let data_criacao = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    tarefas.push(Tarefa {
        id,
        titulo: form.titulo.clone(),
        descricao: form.descricao.clone(),
        completa: false,
        prioridade: form.prioridade.clone(),
        data_criacao,
    });
    salvar_tarefas(&tarefas);
    gerar_html(&tarefas);
    HttpResponse::SeeOther().append_header(("Location", "/")).finish()
}

#[actix_web::post("/remover")]
async fn remover(data: web::Data<AppState>, form: web::Form<IdForm>) -> impl Responder {
    let mut tarefas = data.tarefas.lock().unwrap();
    if let Some(posicao) = tarefas.iter().position(|t| t.id == form.id) {
        tarefas.remove(posicao);
        salvar_tarefas(&tarefas);
        gerar_html(&tarefas);
    }
    HttpResponse::SeeOther().append_header(("Location", "/")).finish()
}

#[actix_web::post("/alterar_status")]
async fn alterar_status(data: web::Data<AppState>, form: web::Form<IdForm>) -> impl Responder {
    let mut tarefas = data.tarefas.lock().unwrap();
    if let Some(tarefa) = tarefas.iter_mut().find(|t| t.id == form.id) {
        tarefa.completa = !tarefa.completa;
        salvar_tarefas(&tarefas);
        gerar_html(&tarefas);
    }
    HttpResponse::SeeOther().append_header(("Location", "/")).finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // Inicializa o logger
    println!("Servidor iniciado em: http://127.0.0.1:8080");

    let tarefas = web::Data::new(AppState {
        tarefas: Mutex::new(carregar_tarefas()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(tarefas.clone())
            .service(index)
            .service(adicionar)
            .service(remover)
            .service(alterar_status)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}



