use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::{common::error::{AppError, ValidationError}};

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct task {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub title: String,
    pub description: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct taskCred {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime
}

#[derive(Debug, Clone, Serialize)]
pub struct taskResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category: CreateCategoryDto,
    pub tags: Vec<CreateTagDto>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime
}

#[derive(FromRow, Serialize)]
pub struct Tags {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub slug: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTagDto {
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug,)]
pub struct TagDtoWithId {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCategoryDto {
    pub name: String,
    pub slug: String,
}

#[derive(FromRow, Serialize)]
pub struct Category {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub slug: String,
}

#[derive(FromRow)]
pub struct Tagtask {
    pub tag_id: Uuid,
    pub name: String,
    pub slug: String
}

pub struct Newtask {
    pub task: String,
    pub description: String,
    pub category_id: Uuid,
    pub tags: Vec<String>
}

pub struct taskDto {
    pub task: String,
    pub description: String,
    pub category_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatetaskCredentials {
    pub task: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatetaskDto {
    pub task: String,
    pub description: String,
    pub is_done: bool,
    pub tags_slug: Vec<String>, 
    pub category_slug: String,
    pub category_id: Uuid
}

impl CreateTagDto {
    pub fn validate(dto: CreateTagDto) -> Result<Self, AppError> {
        let name = dto.name.trim();
        let slug = dto.slug.trim();

        if name.len() < 2 {
            return Err(AppError::Validation(ValidationError::InvalidTag));
        }
        if slug.len() < 3 {
            return Err(AppError::Validation(ValidationError::InvalidTag));
        }

        Ok(Self {
            name: name.to_string(),
            slug: slug.to_string(),
        })
    }
}

impl CreateCategoryDto {
    pub fn validation(dto: CreateCategoryDto) -> Result<Self, AppError> {
         let name = dto.name.trim();
        let slug = dto.slug.trim();

        if name.len() < 3 {
            return Err(AppError::Validation(ValidationError::InvalidTag));
        }
        if slug.len() < 3 {
            return Err(AppError::Validation(ValidationError::InvalidTag));
        }

        Ok(Self {
            name: name.to_string(),
            slug: slug.to_string(),
        })
    }
}

impl TryFrom<CreatetaskDto> for Newtask {
    type Error = ValidationError;

    fn try_from(value: CreatetaskDto) -> Result<Self, Self::Error> {
        let task = value.task.trim();
        let description = value.description.trim();

        if task.len() < 5 {
            return Err(ValidationError::taskTooShort);
        };

        if description.len() < 5 {
            return Err(ValidationError::DescriptionTooShort);
        };

        return Ok(Self {
            task: task.to_string(),
            description: description.to_string(),
            category_id: value.category_id,
            tags: value.tags_slug
        });
    }
}
