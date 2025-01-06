use crate::tarefas::Tarefa;
use std::fs::{self, OpenOptions};
use std::io::Write;

const ARQUIVO: &str = "tarefas.json";

pub fn carregar_tarefas() -> Vec<Tarefa> {
    if let Ok(conteudo) = fs::read_to_string(ARQUIVO) {
        serde_json::from_str(&conteudo).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

pub fn salvar_tarefas(tarefas: &Vec<Tarefa>) {
    if let Ok(json) = serde_json::to_string_pretty(tarefas) {
        let mut arquivo = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(ARQUIVO)
            .expect("Não foi possível abrir o arquivo");
        arquivo.write_all(json.as_bytes()).expect("Erro ao escrever no arquivo");
    }
}


