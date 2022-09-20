// Example fetching data from Postgresql
use sqlx::postgres::PgPoolOptions;

/*
Example prerequisites:
1) Postgress DB at postgres://dbuser:dbuser_password@localhost/db
2) Credentials username:dbuser, password: dbuser_passwrod, database: db
3) Schema:
CREATE TABLE username
CREATE TABLE usernames (name VARCHAR(50));
INSERT INTO usernames(name) VALUES('Bill');
INSERT INTO usernames(name) VALUES('John');
4) .env containing DATABASE_URL=postgres://dbuser:dbuser_password@localhost/db
*/

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://dbuser:dbuser_password@localhost/db")
        .await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let rows = sqlx::query!("SELECT * FROM usernames")
        .fetch_all(&pool)
        .await?;

    println!("{:?}", rows);

    for row in rows {
        println!("{:?}", row)
    }

    Ok(())
}
