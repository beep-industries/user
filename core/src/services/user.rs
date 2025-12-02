use crate::error::CoreError;
use crate::models::{
    Setting, UpdateSettingRequest, UpdateUserRequest, User, UserBasicInfo, UserFullInfo,
};
use crate::repository::UserRepository;
use crate::services::KeycloakService;
use std::future::Future;
use uuid::Uuid;

pub trait UserService: Send + Sync {
    fn get_user_by_id(&self, user_id: Uuid) -> impl Future<Output = Result<UserBasicInfo, CoreError>> + Send;
    fn get_current_user_info(&self, user: &User, full_info: bool) -> impl Future<Output = Result<serde_json::Value, CoreError>> + Send;
    fn update_user(&self, user: &User, req: UpdateUserRequest) -> impl Future<Output = Result<UserBasicInfo, CoreError>> + Send;
    fn get_user_settings(&self, user_id: Uuid) -> impl Future<Output = Result<Setting, CoreError>> + Send;
    fn update_user_settings(&self, user_id: Uuid, req: UpdateSettingRequest) -> impl Future<Output = Result<Setting, CoreError>> + Send;
    fn get_or_create_user(&self, sub: &str) -> impl Future<Output = Result<User, CoreError>> + Send;
}

#[derive(Clone)]
pub struct UserServiceImpl<R: UserRepository> {
    user_repo: R,
    keycloak_service: KeycloakService,
}

impl<R: UserRepository> UserServiceImpl<R> {
    pub fn new(user_repo: R, keycloak_service: KeycloakService) -> Self {
        Self {
            user_repo,
            keycloak_service,
        }
    }
}

impl<R: UserRepository + Clone> UserService for UserServiceImpl<R> {
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<UserBasicInfo, CoreError> {
        let user = self
            .user_repo
            .get_user_by_id(user_id)
            .await?
            .ok_or_else(|| CoreError::NotFound("User not found".to_string()))?;

        Ok(UserBasicInfo {
            id: user.id,
            display_name: user.display_name,
            profile_picture: user.profile_picture,
            description: user.description,
            sub: user.sub,
        })
    }

    async fn get_current_user_info(&self, user: &User, full_info: bool) -> Result<serde_json::Value, CoreError> {
        if full_info {
            let keycloak_info = self
                .keycloak_service
                .get_user_info(&user.sub)
                .await
                .map_err(|e| CoreError::KeycloakError(e.to_string()))?;

            let full = UserFullInfo {
                id: user.id,
                display_name: user.display_name.clone(),
                profile_picture: user.profile_picture.clone(),
                description: user.description.clone(),
                sub: user.sub.clone(),
                username: keycloak_info.username,
                email: keycloak_info.email,
                first_name: keycloak_info.first_name,
                last_name: keycloak_info.last_name,
            };
            serde_json::to_value(full).map_err(|e| CoreError::InternalError(e.to_string()))
        } else {
            let basic = UserBasicInfo {
                id: user.id,
                display_name: user.display_name.clone(),
                profile_picture: user.profile_picture.clone(),
                description: user.description.clone(),
                sub: user.sub.clone(),
            };
            serde_json::to_value(basic).map_err(|e| CoreError::InternalError(e.to_string()))
        }
    }

    async fn update_user(&self, user: &User, req: UpdateUserRequest) -> Result<UserBasicInfo, CoreError> {
        // Update Keycloak first (if it fails, we don't touch the local DB)
        if req.has_keycloak_fields() {
            self.keycloak_service
                .update_user_info(&user.sub, &req)
                .await
                .map_err(|e| CoreError::KeycloakError(e.to_string()))?;
        }

        // Update local DB
        let updated_user = if req.has_local_fields() {
            self.user_repo.update_user(user.id, req).await?
        } else {
            user.clone()
        };

        Ok(UserBasicInfo {
            id: updated_user.id,
            display_name: updated_user.display_name,
            profile_picture: updated_user.profile_picture,
            description: updated_user.description,
            sub: updated_user.sub,
        })
    }

    async fn get_user_settings(&self, user_id: Uuid) -> Result<Setting, CoreError> {
        self.user_repo
            .get_setting_by_user_id(user_id)
            .await?
            .ok_or_else(|| CoreError::NotFound("Settings not found".to_string()))
    }

    async fn update_user_settings(&self, user_id: Uuid, req: UpdateSettingRequest) -> Result<Setting, CoreError> {
        Ok(self.user_repo.update_setting(user_id, req).await?)
    }

    async fn get_or_create_user(&self, sub: &str) -> Result<User, CoreError> {
        Ok(self.user_repo.get_or_create_user(sub).await?)
    }
}
