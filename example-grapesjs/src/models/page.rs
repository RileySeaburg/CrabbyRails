use chrono::{NaiveDateTime, TimeZone};
use rustyroad::database::{Database, PoolConnection};
use serde::{Deserialize, Deserializer};
use sqlx::FromRow;
        

    /// # Name: Page
    /// ### Description: A struct that represents a page created with page
    /// ### Example:
    /// ```
    /// use rustyroad::models::page::Page;
    ///
    /// let new_html = Page::new();
    /// ```
    /// ### Fields:
    /// * id: Option<i32>
    /// * html_content: String
    /// * created_at: DateTime<chrono::Utc>
    /// * updated_at: DateTime<chrono::Utc>
    /// * associated_user_id: i32
    /// * metadata: String
    /// ### Methods:
    /// * create_new_database_page(new_html: Page) -> Result<serde_json::Value, sqlx::Error>
    /// * get_page_by_id(id: i32) -> Result<Page, sqlx::Error>
    /// * get_db_pool() -> Result<sqlx::PgPool, sqlx::Error>
    /// ### Example:
    /// ```
    /// use rustyroad::models::page::Page;
    ///
    /// let new_html = Page::new();
    /// let result = Page::create_new_database_page(new_html);
    /// ```
    /// ### Example:
    /// ```
    /// use rustyroad::models::page::Page;
    ///
    /// let id = 1;
    ///
    /// let result = Page::get_page_by_id(id);
    /// ```
    ///
    /// ### Example:
    /// ```
    /// use rustyroad::models::page::Page;
    ///
    /// let result = Page::get_db_pool();
    /// ```
        #[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize, FromRow)]
        pub struct Page {
        pub id: Option<i32>,
        pub html_content: String,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
        pub associated_user_id: i32,
        pub metadata: String,
        }

        impl Page {
            pub fn new() -> Self {
                Self {
                    id: None,
                    html_content: "".to_string(),
                    created_at: chrono::Utc::now().naive_utc(),
                    updated_at: chrono::Utc::now().naive_local(),
                    associated_user_id: 0,
                    metadata: "".to_string(),
                }
            }

        /// # Name: create_page
        /// ### Description: Creates a new database page
        /// ### Parameters: new_html: Page
        /// ### Returns: Result<serde_json::Value, sqlx::Error>
        /// ### Example:
        /// ```
        /// use rustyroad::models::page::Page;
        ///
        /// let new_html = Page::new();
        /// let result = Page::create_new_database_page(new_html);
        /// ```
        pub async fn create_page(
            new_html: Page,
        ) -> Result<serde_json::Value, sqlx::Error> {
            let sql = r#"INSERT INTO page (html_content, created_at, updated_at, associated_user_id, metadata)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *;"#;

            let database = Database::get_database_from_rustyroad_toml().unwrap();

            
               let pool = Database::get_db_pool(database).await.unwrap();

                let pool_connection = match pool {
                    PoolConnection::Pg(pool) => pool,

                    _ => panic!("Error getting pg pool"),
                };
                

            let new_page:  Page = sqlx::query_as(&sql)
                .bind(new_html.html_content)
                .bind(new_html.created_at)
                .bind(new_html.updated_at)
                .bind(new_html.associated_user_id)
                .bind(new_html.metadata)
                .fetch_one(&pool_connection)
                .await?;

            Ok(serde_json::json!({
                "status": "success",
                "message": "Page saved successfully",
                "data": new_page
            }))
        }


    /// # Name: get_page_by_id
    /// ### Description: Gets a page by id
    /// ### Parameters: id: i32
    /// ### Returns: Result<serde_json::Value, sqlx::Error>
    /// ### Example:
    /// ```
    /// use rustyroad::models::page::Page;
    ///
    /// let id = 1;
    /// let result = Page::get_page_by_id(id);
    /// ```
        pub async fn get_page_by_id(id: i32) -> Result<Page, sqlx::Error> {
            let sql = r#"SELECT * FROM page WHERE id = $1"#;;
            let database = Database::get_database_from_rustyroad_toml().unwrap();
            
               let pool = Database::get_db_pool(database).await.unwrap();

                let pool_connection = match pool {
                    PoolConnection::Pg(pool) => pool,

                    _ => panic!("Error getting pg pool"),
                };
                
            let page: Page = sqlx::query_as(&sql).bind(id).fetch_one(&pool_connection).await?;
            Ok(page)
    }


    /// # Name: update_page
    /// ### Description: Updates a page
    /// ### Parameters: id: i32, new_html: Page
    /// ### Returns: Result<serde_json::Value, sqlx::Error>
    /// ### Example:
    /// ```
    /// use rustyroad::models::page::Page;
    /// let id = 1;
    /// let new_html = Page::new();
    /// let result = Page::update_page(id, new_html);
    /// ```
    pub async fn update_page(
        id: i32,
        new_html: Page,
    ) -> Result<serde_json::Value, sqlx::Error> {
        let sql = r#"UPDATE page SET html_content = $1, updated_at = $2, metadata = $3 WHERE id = $4 RETURNING *;"#;;
        let database = Database::get_database_from_rustyroad_toml().unwrap();
        
               let pool = Database::get_db_pool(database).await.unwrap();

                let pool_connection = match pool {
                    PoolConnection::Pg(pool) => pool,

                    _ => panic!("Error getting pg pool"),
                };
                
        let updated_page: Page = sqlx::query_as(&sql)
            .bind(new_html.html_content)
            .bind(new_html.updated_at)
            .bind(new_html.metadata)
            .bind(id)
            .fetch_one(&pool_connection)
            .await?;

        Ok(serde_json::json!({
            "status": "success",
            "message": "Page updated successfully",
            "data": updated_page
        }))

    }
}



fn deserialize_unix_timestamp<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let timestamp = i64::deserialize(deserializer)?;
    Ok(chrono::Utc.timestamp_opt(timestamp, 0).single().unwrap().naive_utc())
}
