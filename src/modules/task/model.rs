use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::common::error::{AppError, ValidationError};

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub category_id: Option<Uuid>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct TaskCred {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category_id: Option<Uuid>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(Debug, Clone, Serialize)]
pub struct TaskResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category: CreateLabelDto,
    pub tags: Vec<CreateLabelDto>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(Serialize)]
pub struct LabelResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateLabelDto {
    pub name: String,
    pub slug: String,
}

#[derive(FromRow, Serialize)]
pub struct Label {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub slug: String,
}

pub struct NewTask {
    pub task: String,
    pub description: String,
    pub category_id: Uuid,
    pub tags: Vec<String>,
}

pub struct TaskDto {
    pub task: String,
    pub description: String,
    pub category_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTaskCredentials {
    pub task: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTaskDto {
    pub task: String,
    pub description: String,
    pub is_done: bool,
    pub tags_slug: Vec<String>,
    pub category_slug: String,
    pub category_id: Uuid,
}

impl CreateLabelDto {
    pub fn validate(dto: CreateLabelDto) -> Result<Self, AppError> {
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

impl CreateLabelDto {
    pub fn validation(dto: CreateLabelDto) -> Result<Self, AppError> {
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

impl TryFrom<CreateTaskDto> for NewTask {
    type Error = ValidationError;

    fn try_from(value: CreateTaskDto) -> Result<Self, Self::Error> {
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
            tags: value.tags_slug,
        });
    }
}
