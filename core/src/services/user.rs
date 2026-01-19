use crate::error::CoreError;
use crate::models::{
    Setting, UpdateSettingRequest, UpdateUserRequest, User, UserBasicInfo, UserFullInfo,
};
use crate::repository::UserRepository;
use crate::services::KeycloakClient;
use std::future::Future;
use uuid::Uuid;

pub trait UserService: Send + Sync {
    fn get_user_by_sub(
        &self,
        sub: Uuid,
    ) -> impl Future<Output = Result<UserBasicInfo, CoreError>> + Send;
    fn get_user_by_username(
        &self,
        username: &str,
    ) -> impl Future<Output = Result<UserBasicInfo, CoreError>> + Send;
    fn get_users_by_subs(
        &self,
        subs: &[Uuid],
    ) -> impl Future<Output = Result<Vec<UserBasicInfo>, CoreError>> + Send;
    fn get_current_user_info(
        &self,
        user: &User,
        full_info: bool,
    ) -> impl Future<Output = Result<serde_json::Value, CoreError>> + Send;
    fn update_user(
        &self,
        user: &User,
        req: UpdateUserRequest,
    ) -> impl Future<Output = Result<UserBasicInfo, CoreError>> + Send;
    fn get_user_settings(
        &self,
        sub: Uuid,
    ) -> impl Future<Output = Result<Setting, CoreError>> + Send;
    fn update_user_settings(
        &self,
        sub: Uuid,
        req: UpdateSettingRequest,
    ) -> impl Future<Output = Result<Setting, CoreError>> + Send;
    fn get_or_create_user(&self, sub: Uuid)
    -> impl Future<Output = Result<User, CoreError>> + Send;
}

#[derive(Clone)]
pub struct UserServiceImpl<R: UserRepository, K: KeycloakClient> {
    user_repo: R,
    keycloak_client: K,
}

impl<R: UserRepository, K: KeycloakClient> UserServiceImpl<R, K> {
    pub fn new(user_repo: R, keycloak_client: K) -> Self {
        Self {
            user_repo,
            keycloak_client,
        }
    }
}

impl<R: UserRepository + Clone, K: KeycloakClient> UserService for UserServiceImpl<R, K> {
    async fn get_user_by_sub(&self, sub: Uuid) -> Result<UserBasicInfo, CoreError> {
        let user = self
            .user_repo
            .get_user_by_sub(sub)
            .await?
            .ok_or_else(|| CoreError::NotFound("User not found".to_string()))?;

        Ok(user.into())
    }

    async fn get_user_by_username(&self, username: &str) -> Result<UserBasicInfo, CoreError> {
        // Get user ID from Keycloak by username
        let sub = self
            .keycloak_client
            .get_user_id_by_username(username)
            .await?;

        // Get user from local DB
        let user = self
            .user_repo
            .get_user_by_sub(sub)
            .await?
            .ok_or_else(|| CoreError::NotFound("User not found".to_string()))?;

        Ok(user.into())
    }

    async fn get_users_by_subs(&self, subs: &[Uuid]) -> Result<Vec<UserBasicInfo>, CoreError> {
        let users = self.user_repo.get_users_by_subs(subs).await?;

        Ok(users.into_iter().map(Into::into).collect())
    }

    async fn get_current_user_info(
        &self,
        user: &User,
        full_info: bool,
    ) -> Result<serde_json::Value, CoreError> {
        if full_info {
            let keycloak_info = self.keycloak_client.get_user_info(user.sub).await?;

            let full = UserFullInfo {
                sub: user.sub,
                display_name: user.display_name.clone(),
                profile_picture: user.profile_picture.clone(),
                description: user.description.clone(),
                username: keycloak_info.username,
                email: keycloak_info.email,
            };
            serde_json::to_value(full).map_err(|e| CoreError::InternalError(e.to_string()))
        } else {
            let basic: UserBasicInfo = user.clone().into();
            serde_json::to_value(basic).map_err(|e| CoreError::InternalError(e.to_string()))
        }
    }

    async fn update_user(
        &self,
        user: &User,
        req: UpdateUserRequest,
    ) -> Result<UserBasicInfo, CoreError> {
        // Update Keycloak first (if it fails, we don't touch the local DB)
        if req.has_keycloak_fields() {
            self.keycloak_client
                .update_user_info(user.sub, &req)
                .await?;
        }

        // Update local DB
        let updated_user = if req.has_local_fields() {
            self.user_repo.update_user(user.sub, req).await?
        } else {
            user.clone()
        };

        Ok(updated_user.into())
    }

    async fn get_user_settings(&self, sub: Uuid) -> Result<Setting, CoreError> {
        self.user_repo
            .get_setting_by_sub(sub)
            .await?
            .ok_or_else(|| CoreError::NotFound("Settings not found".to_string()))
    }

    async fn update_user_settings(
        &self,
        sub: Uuid,
        req: UpdateSettingRequest,
    ) -> Result<Setting, CoreError> {
        Ok(self.user_repo.update_setting(sub, req).await?)
    }

    async fn get_or_create_user(&self, sub: Uuid) -> Result<User, CoreError> {
        Ok(self.user_repo.get_or_create_user(sub).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{KeycloakUserInfo, Setting, User};
    use crate::services::KeycloakError;
    use chrono::Utc;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    // Mock KeycloakClient
    #[derive(Clone)]
    struct MockKeycloakClient {
        users: Arc<Mutex<HashMap<Uuid, KeycloakUserInfo>>>,
        should_fail: bool,
    }

    impl MockKeycloakClient {
        fn new() -> Self {
            Self {
                users: Arc::new(Mutex::new(HashMap::new())),
                should_fail: false,
            }
        }

        fn with_user(self, sub: Uuid, info: KeycloakUserInfo) -> Self {
            self.users.lock().unwrap().insert(sub, info);
            self
        }

        fn failing() -> Self {
            Self {
                users: Arc::new(Mutex::new(HashMap::new())),
                should_fail: true,
            }
        }
    }

    impl KeycloakClient for MockKeycloakClient {
        async fn get_user_info(&self, sub: Uuid) -> Result<KeycloakUserInfo, KeycloakError> {
            if self.should_fail {
                return Err(KeycloakError::GetUserError("Keycloak unavailable".into()));
            }
            self.users
                .lock()
                .unwrap()
                .get(&sub)
                .cloned()
                .ok_or_else(|| KeycloakError::UserNotFound(sub))
        }

        async fn get_user_id_by_username(&self, username: &str) -> Result<Uuid, KeycloakError> {
            if self.should_fail {
                return Err(KeycloakError::GetUserError("Keycloak unavailable".into()));
            }
            let users = self.users.lock().unwrap();
            for (sub, info) in users.iter() {
                if info.username == username {
                    return Ok(*sub);
                }
            }
            Err(KeycloakError::UserNotFoundByUsername(username.to_string()))
        }

        async fn update_user_info(
            &self,
            sub: Uuid,
            update_req: &UpdateUserRequest,
        ) -> Result<(), KeycloakError> {
            if self.should_fail {
                return Err(KeycloakError::UpdateUserError(
                    "Keycloak unavailable".into(),
                ));
            }
            let mut users = self.users.lock().unwrap();
            if let Some(user) = users.get_mut(&sub) {
                if let Some(username) = &update_req.username {
                    user.username = username.clone();
                }
                if let Some(email) = &update_req.email {
                    user.email = email.clone();
                }
            }
            Ok(())
        }
    }

    // Mock UserRepository
    #[derive(Clone)]
    struct MockUserRepository {
        users: Arc<Mutex<HashMap<Uuid, User>>>,
        settings: Arc<Mutex<HashMap<Uuid, Setting>>>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                users: Arc::new(Mutex::new(HashMap::new())),
                settings: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn with_user(self, user: User) -> Self {
            self.users.lock().unwrap().insert(user.sub, user);
            self
        }

        fn with_setting(self, setting: Setting) -> Self {
            self.settings.lock().unwrap().insert(setting.sub, setting);
            self
        }
    }

    impl UserRepository for MockUserRepository {
        async fn create_user(&self, sub: Uuid) -> Result<User, sqlx::Error> {
            let now = Utc::now();
            let user = User {
                sub,
                display_name: String::new(),
                profile_picture: String::new(),
                description: String::new(),
                created_at: now,
                updated_at: now,
            };
            self.users.lock().unwrap().insert(sub, user.clone());
            Ok(user)
        }

        async fn get_user_by_sub(&self, sub: Uuid) -> Result<Option<User>, sqlx::Error> {
            Ok(self.users.lock().unwrap().get(&sub).cloned())
        }

        async fn get_users_by_subs(&self, subs: &[Uuid]) -> Result<Vec<User>, sqlx::Error> {
            let users = self.users.lock().unwrap();
            let result: Vec<User> = subs
                .iter()
                .filter_map(|sub| users.get(sub).cloned())
                .collect();
            Ok(result)
        }

        async fn get_or_create_user(&self, sub: Uuid) -> Result<User, sqlx::Error> {
            if let Some(user) = self.users.lock().unwrap().get(&sub).cloned() {
                return Ok(user);
            }
            self.create_user(sub).await
        }

        async fn update_user(
            &self,
            sub: Uuid,
            req: UpdateUserRequest,
        ) -> Result<User, sqlx::Error> {
            let mut users = self.users.lock().unwrap();
            if let Some(user) = users.get_mut(&sub) {
                if let Some(display_name) = req.display_name {
                    user.display_name = display_name;
                }
                if let Some(profile_picture) = req.profile_picture {
                    user.profile_picture = profile_picture;
                }
                if let Some(description) = req.description {
                    user.description = description;
                }
                user.updated_at = Utc::now();
                return Ok(user.clone());
            }
            Err(sqlx::Error::RowNotFound)
        }

        async fn get_setting_by_sub(&self, sub: Uuid) -> Result<Option<Setting>, sqlx::Error> {
            Ok(self.settings.lock().unwrap().get(&sub).cloned())
        }

        async fn create_setting(&self, sub: Uuid) -> Result<Setting, sqlx::Error> {
            let now = Utc::now();
            let setting = Setting {
                sub,
                theme: Some("light".to_string()),
                lang: Some("en".to_string()),
                created_at: now,
                updated_at: now,
            };
            self.settings.lock().unwrap().insert(sub, setting.clone());
            Ok(setting)
        }

        async fn update_setting(
            &self,
            sub: Uuid,
            req: UpdateSettingRequest,
        ) -> Result<Setting, sqlx::Error> {
            let mut settings = self.settings.lock().unwrap();
            if let Some(setting) = settings.get_mut(&sub) {
                if let Some(theme) = req.theme {
                    setting.theme = Some(theme);
                }
                if let Some(lang) = req.lang {
                    setting.lang = Some(lang);
                }
                setting.updated_at = Utc::now();
                return Ok(setting.clone());
            }
            Err(sqlx::Error::RowNotFound)
        }
    }

    fn create_test_user(sub: Uuid) -> User {
        let now = Utc::now();
        User {
            sub,
            display_name: "Test User".to_string(),
            profile_picture: "https://example.com/pic.jpg".to_string(),
            description: "A test user".to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    fn create_test_setting(sub: Uuid) -> Setting {
        let now = Utc::now();
        Setting {
            sub,
            theme: Some("dark".to_string()),
            lang: Some("fr".to_string()),
            created_at: now,
            updated_at: now,
        }
    }

    mod get_user_by_sub {
        use super::*;

        #[tokio::test]
        async fn returns_user_when_exists() {
            let sub = Uuid::new_v4();
            let user = create_test_user(sub);

            let repo = MockUserRepository::new().with_user(user.clone());
            let keycloak = MockKeycloakClient::new();
            let service = UserServiceImpl::new(repo, keycloak);

            let result = service.get_user_by_sub(sub).await.unwrap();

            assert_eq!(result.sub, sub);
            assert_eq!(result.display_name, "Test User");
        }

        #[tokio::test]
        async fn returns_not_found_when_user_does_not_exist() {
            let sub = Uuid::new_v4();

            let repo = MockUserRepository::new();
            let keycloak = MockKeycloakClient::new();
            let service = UserServiceImpl::new(repo, keycloak);

            let result = service.get_user_by_sub(sub).await;

            assert!(matches!(result, Err(CoreError::NotFound(_))));
        }
    }

    mod get_current_user_info {
        use super::*;

        #[tokio::test]
        async fn returns_basic_info_when_full_info_is_false() {
            let sub = Uuid::new_v4();
            let user = create_test_user(sub);

            let repo = MockUserRepository::new();
            let keycloak = MockKeycloakClient::new();
            let service = UserServiceImpl::new(repo, keycloak);

            let result = service.get_current_user_info(&user, false).await.unwrap();

            assert_eq!(result["sub"], sub.to_string());
            assert_eq!(result["display_name"], "Test User");
            assert!(result.get("username").is_none());
            assert!(result.get("email").is_none());
        }

        #[tokio::test]
        async fn returns_full_info_when_full_info_is_true() {
            let sub = Uuid::new_v4();
            let user = create_test_user(sub);
            let keycloak_info = KeycloakUserInfo {
                username: "testuser".to_string(),
                email: "test@example.com".to_string(),
            };

            let repo = MockUserRepository::new();
            let keycloak = MockKeycloakClient::new().with_user(sub, keycloak_info);
            let service = UserServiceImpl::new(repo, keycloak);

            let result = service.get_current_user_info(&user, true).await.unwrap();

            assert_eq!(result["sub"], sub.to_string());
            assert_eq!(result["display_name"], "Test User");
            assert_eq!(result["username"], "testuser");
            assert_eq!(result["email"], "test@example.com");
        }

        #[tokio::test]
        async fn returns_keycloak_error_when_keycloak_fails() {
            let sub = Uuid::new_v4();
            let user = create_test_user(sub);

            let repo = MockUserRepository::new();
            let keycloak = MockKeycloakClient::failing();
            let service = UserServiceImpl::new(repo, keycloak);

            let result = service.get_current_user_info(&user, true).await;

            assert!(matches!(result, Err(CoreError::KeycloakError(_))));
        }
    }

    mod update_user {
        use super::*;

        #[tokio::test]
        async fn updates_local_fields_only() {
            let sub = Uuid::new_v4();
            let user = create_test_user(sub);

            let repo = MockUserRepository::new().with_user(user.clone());
            let keycloak = MockKeycloakClient::new();
            let service = UserServiceImpl::new(repo, keycloak);

            let req = UpdateUserRequest {
                display_name: Some("New Name".to_string()),
                profile_picture: None,
                description: None,
                username: None,
                email: None,
            };

            let result = service.update_user(&user, req).await.unwrap();

            assert_eq!(result.display_name, "New Name");
        }

        #[tokio::test]
        async fn updates_keycloak_fields() {
            let sub = Uuid::new_v4();
            let user = create_test_user(sub);
            let keycloak_info = KeycloakUserInfo {
                username: "olduser".to_string(),
                email: "old@example.com".to_string(),
            };

            let repo = MockUserRepository::new().with_user(user.clone());
            let keycloak = MockKeycloakClient::new().with_user(sub, keycloak_info);
            let service = UserServiceImpl::new(repo, keycloak);

            let req = UpdateUserRequest {
                display_name: None,
                profile_picture: None,
                description: None,
                username: Some("newuser".to_string()),
                email: None,
            };

            let result = service.update_user(&user, req).await;

            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn returns_keycloak_error_when_keycloak_update_fails() {
            let sub = Uuid::new_v4();
            let user = create_test_user(sub);

            let repo = MockUserRepository::new().with_user(user.clone());
            let keycloak = MockKeycloakClient::failing();
            let service = UserServiceImpl::new(repo, keycloak);

            let req = UpdateUserRequest {
                display_name: None,
                profile_picture: None,
                description: None,
                username: Some("newuser".to_string()),
                email: None,
            };

            let result = service.update_user(&user, req).await;

            assert!(matches!(result, Err(CoreError::KeycloakError(_))));
        }

        #[tokio::test]
        async fn does_not_update_local_db_when_keycloak_fails() {
            let sub = Uuid::new_v4();
            let user = create_test_user(sub);

            let repo = MockUserRepository::new().with_user(user.clone());
            let keycloak = MockKeycloakClient::failing();
            let service = UserServiceImpl::new(repo.clone(), keycloak);

            let req = UpdateUserRequest {
                display_name: Some("New Name".to_string()),
                profile_picture: None,
                description: None,
                username: Some("newuser".to_string()),
                email: None,
            };

            let _ = service.update_user(&user, req).await;

            // Verify local DB was not updated
            let stored_user = repo.get_user_by_sub(sub).await.unwrap().unwrap();
            assert_eq!(stored_user.display_name, "Test User");
        }
    }

    mod get_user_settings {
        use super::*;

        #[tokio::test]
        async fn returns_settings_when_exist() {
            let sub = Uuid::new_v4();
            let setting = create_test_setting(sub);

            let repo = MockUserRepository::new().with_setting(setting);
            let keycloak = MockKeycloakClient::new();
            let service = UserServiceImpl::new(repo, keycloak);

            let result = service.get_user_settings(sub).await.unwrap();

            assert_eq!(result.sub, sub);
            assert_eq!(result.theme, Some("dark".to_string()));
            assert_eq!(result.lang, Some("fr".to_string()));
        }

        #[tokio::test]
        async fn returns_not_found_when_settings_do_not_exist() {
            let sub = Uuid::new_v4();

            let repo = MockUserRepository::new();
            let keycloak = MockKeycloakClient::new();
            let service = UserServiceImpl::new(repo, keycloak);

            let result = service.get_user_settings(sub).await;

            assert!(matches!(result, Err(CoreError::NotFound(_))));
        }
    }

    mod update_user_settings {
        use super::*;

        #[tokio::test]
        async fn updates_theme() {
            let sub = Uuid::new_v4();
            let setting = create_test_setting(sub);

            let repo = MockUserRepository::new().with_setting(setting);
            let keycloak = MockKeycloakClient::new();
            let service = UserServiceImpl::new(repo, keycloak);

            let req = UpdateSettingRequest {
                theme: Some("light".to_string()),
                lang: None,
            };

            let result = service.update_user_settings(sub, req).await.unwrap();

            assert_eq!(result.theme, Some("light".to_string()));
            assert_eq!(result.lang, Some("fr".to_string()));
        }

        #[tokio::test]
        async fn updates_lang() {
            let sub = Uuid::new_v4();
            let setting = create_test_setting(sub);

            let repo = MockUserRepository::new().with_setting(setting);
            let keycloak = MockKeycloakClient::new();
            let service = UserServiceImpl::new(repo, keycloak);

            let req = UpdateSettingRequest {
                theme: None,
                lang: Some("en".to_string()),
            };

            let result = service.update_user_settings(sub, req).await.unwrap();

            assert_eq!(result.theme, Some("dark".to_string()));
            assert_eq!(result.lang, Some("en".to_string()));
        }
    }

    mod get_or_create_user {
        use super::*;

        #[tokio::test]
        async fn returns_existing_user() {
            let sub = Uuid::new_v4();
            let user = create_test_user(sub);

            let repo = MockUserRepository::new().with_user(user.clone());
            let keycloak = MockKeycloakClient::new();
            let service = UserServiceImpl::new(repo, keycloak);

            let result = service.get_or_create_user(sub).await.unwrap();

            assert_eq!(result.sub, sub);
            assert_eq!(result.display_name, "Test User");
        }

        #[tokio::test]
        async fn creates_new_user_when_not_exists() {
            let sub = Uuid::new_v4();

            let repo = MockUserRepository::new();
            let keycloak = MockKeycloakClient::new();
            let service = UserServiceImpl::new(repo, keycloak);

            let result = service.get_or_create_user(sub).await.unwrap();

            assert_eq!(result.sub, sub);
            assert_eq!(result.display_name, "");
        }
    }
}
