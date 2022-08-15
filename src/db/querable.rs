use rocket::async_trait;

#[async_trait]
pub trait Querable {
    fn column_names(&self) -> &'static [&'static str];
    fn column_types(&self) -> &'static [&'static str];
    fn table(&self) -> &'static str;
    async fn select_query(table: &str, where_clause: &str, storage: &crate::db::Storage) -> sqlx::Result<Vec<Self>>
    where
        Self: Sized;
    async fn insert_query(&mut self,table: &str, storage: &crate::db::Storage) -> sqlx::Result<Self>
    where                                    
        Self: Sized;                         
    async fn update_query(&self,table: &str, storage: &crate::db::Storage) -> sqlx::Result<Self>
    where
        Self: Sized;
}

#[macro_export]
macro_rules! querable_struct {
    {
        $(#[$struct_attr:meta])*
        pub struct $struct_name:ident {
            $(
                $(#[$field_attr:meta])*
                pub $field_name:ident: $field_type:ty,
            )+
        }
    } => {
        $(#[$struct_attr])*
        pub struct $struct_name {
            $(
                $(#[$field_attr])*
                pub $field_name: $field_type,
            )+
        }
        #[rocket::async_trait]
        impl crate::db::Querable for $struct_name {
            fn column_names(&self) -> &'static [&'static str] {
                &[
                    $(stringify!($field_name),)+
                ]
            }
            fn column_types(&self) -> &'static [&'static str] {
                &[
                    $(stringify!($field_type),)+
                ]
            }
            fn table(&self) -> &'static str {
                stringify!($struct_name)
            }
            async fn insert_query(&mut self, table: &str, storage: &crate::db::Storage) -> sqlx::Result<Self>
            where
                Self: Sized {
                let uuid = crate::types::uuid::Uuid::new();
                self.id = Some(uuid);
                sqlx::query_as(
                    &format!(
                        r#"
                            INSERT INTO {} (
                                {}
                            ) VALUES (
                                {}
                            )
                            RETURNING *
                        "#,
                        table,
                        self.column_names().join(","),
                        vec!["?"; self.column_names().len()].join(",")
                    )
                )
                $(.bind(&self.$field_name))+
                .fetch_one(&storage.db)
                .await
            }
            async fn update_query(&self, table: &str, storage: &crate::db::Storage) -> sqlx::Result<Self>
            where
                Self: Sized {
                sqlx::query_as(
                    &format!(
                        r#"
                            UPDATE {}
                            SET
                                {}
                            WHERE
                                id = ?
                            RETURNING *
                        "#,
                        table,
                        self.column_names().join(" = ?, ") + " = ?",
                    )
                )
                $(.bind(&self.$field_name))+
                .bind(&self.id)
                .fetch_one(&storage.db)
                .await
            }
            async fn select_query(table: &str, where_clause: &str, storage: &crate::db::Storage) -> sqlx::Result<Vec<Self>>
            where
                Self: Sized {
                sqlx::query_as(
                    &format!(
                        r#"
                            SELECT *
                            FROM {}
                            WHERE
                                {}
                        "#,
                        table,
                        where_clause,
                    )
                )
                .fetch_all(&storage.db)
                .await
            }
        }
    };
}
