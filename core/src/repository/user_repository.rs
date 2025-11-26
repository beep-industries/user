use crate::models::{CreateUserRequest, Setting, UpdateSettingRequest, UpdateUserRequest, User};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, req: CreateUserRequest) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (display_name, profile_picture, keycloak_id)
            VALUES ($1, $2, $3)
            RETURNING id, display_name, profile_picture, status, keycloak_id, created_at, updated_at
            "#,
        )
        .bind(&req.display_name)
        .bind(&req.profile_picture)
        .bind(&req.keycloak_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, display_name, profile_picture, status, keycloak_id, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_keycloak_id(
        &self,
        keycloak_id: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, display_name, profile_picture, status, keycloak_id, created_at, updated_at
            FROM users
            WHERE keycloak_id = $1
            "#,
        )
        .bind(keycloak_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update_user(
        &self,
        user_id: Uuid,
        req: UpdateUserRequest,
    ) -> Result<User, sqlx::Error> {
        let mut query = String::from("UPDATE users SET updated_at = NOW()");
        let mut bindings = Vec::new();
        let mut param_count = 1;

        if let Some(display_name) = &req.display_name {
            query.push_str(&format!(", display_name = ${}", param_count));
            bindings.push(display_name.clone());
            param_count += 1;
        }

        if let Some(profile_picture) = &req.profile_picture {
            query.push_str(&format!(", profile_picture = ${}", param_count));
            bindings.push(profile_picture.clone());
            param_count += 1;
        }

        if let Some(status) = &req.status {
            query.push_str(&format!(", status = ${}", param_count));
            bindings.push(status.clone());
            param_count += 1;
        }

        query.push_str(&format!(" WHERE id = ${} RETURNING id, display_name, profile_picture, status, keycloak_id, created_at, updated_at", param_count));

        let mut q = sqlx::query_as::<_, User>(&query);
        for binding in bindings {
            q = q.bind(binding);
        }
        q = q.bind(user_id);

        let user = q.fetch_one(&self.pool).await?;

        Ok(user)
    }

    pub async fn get_setting_by_user_id(&self, user_id: Uuid) -> Result<Option<Setting>, sqlx::Error> {
        let setting = sqlx::query_as::<_, Setting>(
            r#"
            SELECT id, user_id, theme, lang, created_at, updated_at
            FROM param
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(setting)
    }

    pub async fn create_setting(&self, user_id: Uuid) -> Result<Setting, sqlx::Error> {
        let setting = sqlx::query_as::<_, Setting>(
            r#"
            INSERT INTO param (user_id)
            VALUES ($1)
            RETURNING id, user_id, theme, lang, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(setting)
    }

    pub async fn update_setting(
        &self,
        user_id: Uuid,
        req: UpdateSettingRequest,
    ) -> Result<Setting, sqlx::Error> {
        let mut query = String::from("UPDATE param SET updated_at = NOW()");
        let mut bindings = Vec::new();
        let mut bind_count = 1;

        if let Some(theme) = &req.theme {
            query.push_str(&format!(", theme = ${}", bind_count));
            bindings.push(theme.clone());
            bind_count += 1;
        }

        if let Some(lang) = &req.lang {
            query.push_str(&format!(", lang = ${}", bind_count));
            bindings.push(lang.clone());
            bind_count += 1;
        }

        query.push_str(&format!(" WHERE user_id = ${} RETURNING id, user_id, theme, lang, created_at, updated_at", bind_count));

        let mut q = sqlx::query_as::<_, Setting>(&query);
        for binding in bindings {
            q = q.bind(binding);
        }
        q = q.bind(user_id);

        let setting = q.fetch_one(&self.pool).await?;

        Ok(setting)
    }
}
