use sqlx::mysql::MySqlPool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPool::connect("mysql://user:password@localhost/lidluren").await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS hours (
            id INT AUTO_INCREMENT PRIMARY KEY,
            uren_gewerkt INT NOT NULL,
            aantal_minuten_pauze INT NOT NULL,
            begintijd TIME NOT NULL,
            eindtijd TIME NOT NULL,
            datum DATE NOT NULL
        )"
    )
    .execute(&pool)
    .await?;

    println!("Table 'hours' has been created successfully");
    Ok(())
}
fn main() {
    println!("Hello, world!");

fn print_ascii_table() {
    let table = r#"
    +----------------+---------------------+------------+----------+------------+
    | Uren gewerkt   | Aantal minuten pauze | Begintijd | Eindtijd | Datum      |
    +----------------+---------------------+------------+----------+------------+
    |                |                     |            |          |            |
    +----------------+---------------------+------------+----------+------------+
    "#;

    println!("{}", table);
}
}
