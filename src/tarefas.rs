use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tarefa {
    pub id: usize,
    pub titulo: String,
    pub descricao: String,
    pub completa: bool,
    pub prioridade: String,
    pub data_criacao: String,
}

#[derive(Debug, Deserialize)]
pub struct TarefaForm {
    pub titulo: String,
    pub descricao: String,
    pub prioridade: String,
}

#[derive(Debug, Deserialize)]
pub struct IdForm {
    pub id: usize,
}



