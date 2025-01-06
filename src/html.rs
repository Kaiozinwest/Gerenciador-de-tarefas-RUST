use crate::tarefas::Tarefa;
use std::fs::File;
use std::io::Write;

pub fn gerar_html(tarefas: &Vec<Tarefa>) {
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>");
    html.push_str("<html lang=\"en\">");
    html.push_str("<head>");
    html.push_str("<meta charset=\"UTF-8\">");
    html.push_str("<title>Gerenciador de Tarefas</title>");
    html.push_str("<style>");
    html.push_str("body { font-family: 'Arial', sans-serif; margin: 20px; background-color: #f3f4f6; color: #333; line-height: 1.6; }");
    html.push_str("h1 { color: #3b82f6; text-align: center; }");
    html.push_str("table { width: 100%; border-collapse: collapse; margin: 20px 0; box-shadow: 0 2px 5px rgba(0,0,0,0.1); }");
    html.push_str("th, td { border: 1px solid #ddd; padding: 12px; text-align: center; }");
    html.push_str("th { background-color: #3b82f6; color: white; font-weight: bold; }");
    html.push_str("tr:nth-child(even) { background-color: #f9fafb; }");
    html.push_str("tr:hover { background-color: #e5e7eb; transition: 0.2s ease-in-out; }");
    html.push_str("form { margin: 20px 0; padding: 20px; background: white; border-radius: 8px; box-shadow: 0 4px 10px rgba(0,0,0,0.1); }");
    html.push_str("input, select, textarea, button { font-size: 16px; padding: 10px; margin: 5px 0; width: 100%; box-sizing: border-box; border: 1px solid #ddd; border-radius: 6px; }");
    html.push_str("button { background-color: #3b82f6; color: white; font-weight: bold; cursor: pointer; border: none; }");
    html.push_str("button:hover { background-color: #2563eb; }");
    html.push_str("textarea { resize: none; }");
    html.push_str("p { text-align: center; font-style: italic; color: #6b7280; }");
    html.push_str("</style>");
    
    html.push_str("</head>");
    html.push_str("<body>");
    html.push_str("<h1>Gerenciador de Tarefas</h1>");

    html.push_str("<h2>Tarefas</h2>");
    html.push_str("<table>");
    html.push_str("<tr><th>ID</th><th>Título</th><th>Descrição</th><th>Prioridade</th><th>Status</th></tr>");
    for tarefa in tarefas {
        html.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            tarefa.id,
            tarefa.titulo,
            tarefa.descricao,
            tarefa.prioridade,
            if tarefa.completa { "Completa" } else { "Pendente" }
        ));
    }
    html.push_str("</table>");

    html.push_str("<h2>Adicionar Nova Tarefa</h2>");
    html.push_str("<form action=\"/adicionar\" method=\"post\">");
    html.push_str("<input type=\"text\" name=\"titulo\" placeholder=\"Título\" required>");
    html.push_str("<textarea name=\"descricao\" placeholder=\"Descrição\" required></textarea>");
    html.push_str("<select name=\"prioridade\">");
    html.push_str("<option value=\"Alta\">Alta</option>");
    html.push_str("<option value=\"Média\">Média</option>");
    html.push_str("<option value=\"Baixa\">Baixa</option>");
    html.push_str("</select>");
    html.push_str("<button type=\"submit\">Adicionar</button>");
    html.push_str("</form>");

    html.push_str("<h2>Remover Tarefa</h2>");
    html.push_str("<form action=\"/remover\" method=\"post\">");
    html.push_str("<input type=\"number\" name=\"id\" placeholder=\"ID\" required>");
    html.push_str("<button type=\"submit\">Remover</button>");
    html.push_str("</form>");

    html.push_str("<h2>Alterar Status</h2>");
    html.push_str("<form action=\"/alterar_status\" method=\"post\">");
    html.push_str("<input type=\"number\" name=\"id\" placeholder=\"ID\" required>");
    html.push_str("<button type=\"submit\">Alterar Status</button>");
    html.push_str("</form>");

    html.push_str("</body>");
    html.push_str("</html>");

    let mut file = File::create("index.html").expect("Erro ao criar arquivo HTML.");
    file.write_all(html.as_bytes())
        .expect("Erro ao escrever no arquivo HTML.");
}




