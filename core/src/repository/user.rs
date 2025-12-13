use crate::models::{Setting, UpdateSettingRequest, UpdateUserRequest, User};
use sqlx::PgPool;
use std::future::Future;
use uuid::Uuid;

pub trait UserRepository: Send + Sync {
    fn create_user(&self, sub: Uuid) -> impl Future<Output = Result<User, sqlx::Error>> + Send;
    fn get_user_by_sub(
        &self,
        sub: Uuid,
    ) -> impl Future<Output = Result<Option<User>, sqlx::Error>> + Send;
    fn get_users_by_subs(
        &self,
        subs: &[Uuid],
    ) -> impl Future<Output = Result<Vec<User>, sqlx::Error>> + Send;
    fn get_or_create_user(
        &self,
        sub: Uuid,
    ) -> impl Future<Output = Result<User, sqlx::Error>> + Send;
    fn update_user(
        &self,
        sub: Uuid,
        req: UpdateUserRequest,
    ) -> impl Future<Output = Result<User, sqlx::Error>> + Send;
    fn get_setting_by_sub(
        &self,
        sub: Uuid,
    ) -> impl Future<Output = Result<Option<Setting>, sqlx::Error>> + Send;
    fn create_setting(
        &self,
        sub: Uuid,
    ) -> impl Future<Output = Result<Setting, sqlx::Error>> + Send;
    fn update_setting(
        &self,
        sub: Uuid,
        req: UpdateSettingRequest,
    ) -> impl Future<Output = Result<Setting, sqlx::Error>> + Send;
}

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for PostgresUserRepository {
    async fn create_user(&self, sub: Uuid) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (sub)
            VALUES ($1)
            RETURNING sub, display_name, profile_picture, description, created_at, updated_at
            "#,
        )
        .bind(sub)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_user_by_sub(&self, sub: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT sub, display_name, profile_picture, description, created_at, updated_at
            FROM users
            WHERE sub = $1
            "#,
        )
        .bind(sub)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_users_by_subs(&self, subs: &[Uuid]) -> Result<Vec<User>, sqlx::Error> {
        if subs.is_empty() {
            return Ok(Vec::new());
        }

        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT sub, display_name, profile_picture, description, created_at, updated_at
            FROM users
            WHERE sub = ANY($1)
            "#,
        )
        .bind(subs)
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    async fn get_or_create_user(&self, sub: Uuid) -> Result<User, sqlx::Error> {
        if let Some(user) = self.get_user_by_sub(sub).await? {
            return Ok(user);
        }
        self.create_user(sub).await
    }

    async fn update_user(&self, sub: Uuid, req: UpdateUserRequest) -> Result<User, sqlx::Error> {
        let mut builder: sqlx::QueryBuilder<sqlx::Postgres> =
            sqlx::QueryBuilder::new("UPDATE users SET updated_at = NOW()");

        if let Some(display_name) = &req.display_name {
            builder.push(", display_name = ");
            builder.push_bind(display_name);
        }

        if let Some(profile_picture) = &req.profile_picture {
            builder.push(", profile_picture = ");
            builder.push_bind(profile_picture);
        }

        if let Some(description) = &req.description {
            builder.push(", description = ");
            builder.push_bind(description);
        }

        builder.push(" WHERE sub = ");
        builder.push_bind(sub);
        builder.push(
            " RETURNING sub, display_name, profile_picture, description, created_at, updated_at",
        );

        let user = builder
            .build_query_as::<User>()
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn get_setting_by_sub(&self, sub: Uuid) -> Result<Option<Setting>, sqlx::Error> {
        let setting = sqlx::query_as::<_, Setting>(
            r#"
            SELECT sub, theme, lang, created_at, updated_at
            FROM param
            WHERE sub = $1
            "#,
        )
        .bind(sub)
        .fetch_optional(&self.pool)
        .await?;

        Ok(setting)
    }

    async fn create_setting(&self, sub: Uuid) -> Result<Setting, sqlx::Error> {
        let setting = sqlx::query_as::<_, Setting>(
            r#"
            INSERT INTO param (sub)
            VALUES ($1)
            RETURNING sub, theme, lang, created_at, updated_at
            "#,
        )
        .bind(sub)
        .fetch_one(&self.pool)
        .await?;

        Ok(setting)
    }

    async fn update_setting(
        &self,
        sub: Uuid,
        req: UpdateSettingRequest,
    ) -> Result<Setting, sqlx::Error> {
        let mut builder: sqlx::QueryBuilder<sqlx::Postgres> =
            sqlx::QueryBuilder::new("UPDATE param SET updated_at = NOW()");

        if let Some(theme) = &req.theme {
            builder.push(", theme = ");
            builder.push_bind(theme);
        }

        if let Some(lang) = &req.lang {
            builder.push(", lang = ");
            builder.push_bind(lang);
        }

        builder.push(" WHERE sub = ");
        builder.push_bind(sub);
        builder.push(" RETURNING sub, theme, lang, created_at, updated_at");

        let setting = builder
            .build_query_as::<Setting>()
            .fetch_one(&self.pool)
            .await?;

        Ok(setting)
    }
}
