use crate::db::types;
use std::env;
use tokio_postgres::{Error, NoTls};

pub async fn get_sources() -> Result<Vec<types::NewsSource>, Error> {
    // Подключение к базе данных
    let host = env::var("DB_HOST").unwrap();
    let login = env::var("DB_LOGIN").unwrap();
    let password = env::var("DB_PASSWORD").unwrap();
    let database = env::var("DB_DATABASE").unwrap();
    let table = env::var("DB_TABLE").unwrap();
    let port = env::var("DB_PORT").unwrap();

    let connection_string =
        format!("host={host} port={port} user={login} password={password} dbname={database}",);

    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;

    // Отдельная задача для работы соединения
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Ошибка подключения: {e}");
        }
    });

    let query = format!("SELECT * FROM {table}");

    // Пример запроса: выбираем все строки из таблицы users
    let rows = client.query(&query, &[]).await?;

    let mut a: Vec<types::NewsSource> = Vec::new();
    // Обработка результатов
    for row in rows {
        let id: u32 = row.get("id");
        let vk: String = row.get("vk");
        let readable: String = row.get("readable");
        let is_rt: bool = row.get("is_rt");
        let source = types::NewsSource {
            id,
            vk,
            readable,
            is_rt,
        };
        a.push(source);
    }

    Ok(a)
}
