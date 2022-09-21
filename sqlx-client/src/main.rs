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

    // SELECT with query!
    // query! macro makes compile-time query checks, query does NOT
    let rows = sqlx::query!("SELECT * FROM usernames")
        .fetch_all(&pool)
        .await?;

    println!("{:?}", rows);

    for row in rows {
        println!("{:?}", row);

        if let Some(name) = row.name {
            println!("{}", name);
        }
    }

    // UPDATE
    let update_statement = r#"UPDATE usernames SET name = 'Matrix_OLD' WHERE name = 'Matrix'"#;
    let mut rows_affected = sqlx::query(update_statement).execute(&pool).await?;
    println!("{:?}", rows_affected);

    // INSERT
    let insert_statement = r#"INSERT INTO usernames(name) VALUES('Matrix')"#;
    rows_affected = sqlx::query(insert_statement).execute(&pool).await?;
    println!("{:?}", rows_affected);

    // SELECT with parameters
    let name = "Matrix_OLD";
    let rows = sqlx::query!("SELECT * FROM usernames WHERE name = $1", name)
        .fetch_all(&pool)
        .await?;

    println!("Matrix_OLD DB count: {}", rows.len());

    // DELETE
    let delete_statement = "DELETE FROM usernames WHERE name = 'Matrix_OLD'";
    let res = sqlx::query(delete_statement).execute(&pool).await?;
    println!("{:?}", res);

    Ok(())
}
