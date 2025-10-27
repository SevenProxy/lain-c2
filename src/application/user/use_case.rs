use crate::domain::user::{
    UserRepository,
    entity,
};

use sea_orm::{
    DbErr,
    ActiveValue::Set,
};

pub struct UserUseCase<R: UserRepository> {
    repo: R,
}

pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub avatar: String,
}

impl<R: UserRepository> UserUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self {
            repo,
        }
    }

    pub async fn create(&self, user: User) -> Result<entity::Model, DbErr> {
        let active = entity::ActiveModel {
            id: Set(user.id),
            username: Set(user.username),
            password: Set(user.password),
            avatar: Set(user.avatar),
            ..Default::default()
        };

        self.repo.create(active).await
    }

    pub async fn find_user(&self, user_id: i32) -> Result<Option<entity::Model>, DbErr> {
        self.repo.find_user(user_id).await
    }
}
