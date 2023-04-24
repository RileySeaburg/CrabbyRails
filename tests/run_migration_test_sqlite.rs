#[cfg(test)]
mod tests {

    use rustyroad::database::Database;
    use rustyroad::Project;
    use sqlx::sqlite::SqliteConnectOptions;
    use sqlx::ConnectOptions;

    // Define a new struct to hold the table names
    #[derive(Debug, sqlx::FromRow, PartialEq)]
    struct TableResult {
        name: String,
    }

    #[tokio::test]
    async fn test_run_migration() -> Result<(), Box<dyn std::error::Error>> {
        // SQLite databases do not require a username, password, host, or port, so we leave these fields empty
        let database: Database = Database::new(
            "test".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "sqlite".to_owned(),
        );

        // Create a new project with the desired name
        let project_name = String::from("test_project");
        let project = Project::create_new_project(project_name, database.clone())
            .await
            .expect("Error creating new project");

        // Verify that the migration was applied successfully
        let mut connection = SqliteConnectOptions::new()
            // Use the config_dev_db field as the SQLite database file path
            .filename(&project.config_dev_db)
            .connect()
            .await?;

        // Query to list all tables in the SQLite database
        let query = "SELECT name FROM sqlite_master WHERE type='table';";
        // Run the query and load the result into a Vec<TableResult>
        let result = sqlx::query_as::<_, TableResult>(query)
            .fetch_all(&mut connection)
            .await?;

        // Print the list of tables
        println!("List of tables in the database: {:?}", result);

        // Assert that the 'users' table was created
        assert!(
            result.contains(&TableResult {
                name: "users".to_string()
            }),
            "The 'users' table should have been created by the migration"
        );

        Ok(())
    }
}
