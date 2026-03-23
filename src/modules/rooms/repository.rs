/*
room -> create, update, delete, get, get all, get all members
message -> create, update, delete, get all(perticular channel in descending)
member -> join, remove, unjoin
*/

use sqlx::{PgPool, Result, query_as};
use uuid::Uuid;

use crate::{
    common::error::AppError,
    modules::rooms::model::{Members, MessageDto, MessageResponse, Room, RoomDto},
};

pub struct RoomRepo;

impl RoomRepo {
    pub async fn create_room(pool: &PgPool, room: RoomDto, user_id: Uuid) -> Result<Room> {
        let room = sqlx::query_as!(
            Room,
            r#"
            INSERT INTO rooms (owner_id, name, description, profile_Pic)
            VALUES ($1, $2, $3, $4)
            RETURNING id, owner_id, name, description, profile_Pic, created_at
            "#,
            user_id,
            room.name,
            room.description,
            room.profile_pic
        )
        .fetch_one(pool)
        .await?;

        Ok(room)
    }
    //TODO: implment update, delete
    pub async fn get_room(pool: &PgPool, room_id: Uuid) -> Result<Option<Room>> {
        let room = sqlx::query_as!(
            Room,
            r#"
            SELECT id, owner_id, name, description, profile_pic, created_at
            FROM rooms
            WHERE id = $1
            "#,
            room_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(room)
    }

    pub async fn get_all_rooms(pool: &PgPool) -> Result<Vec<Room>> {
        let rooms = sqlx::query_as!(
            Room,
            r#"
            SELECT id, owner_id, name, description, profile_pic, created_at
            FROM rooms
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(rooms)
    }

    pub async fn get_all_joined_rooms(pool: &PgPool, user_id: &Uuid) -> Result<Vec<Room>> {
        let rooms = sqlx::query_as!(
            Room,
            r#"
                SELECT 
                    m.room_id AS id,
                    r.owner_id,
                    r.name,
                    r.description,
                    r.profile_pic,
                    r.created_at
                FROM members m
                JOIN rooms r ON m.room_id = r.id
                WHERE m.user_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(rooms)
    }

    pub async fn create_message(
        pool: &PgPool,
        message: MessageDto,
        user_id: &Uuid,
        parent_id: Option<Uuid>
    ) -> Result<MessageResponse> {
        let message: MessageResponse = sqlx::query_as!(
            MessageResponse,
            r#"
            WITH inserted AS (
                INSERT INTO user_messages (user_id, room_id, content, parent_id)
                VALUES ($1, $2, $3, $4)
                RETURNING id, user_id, content, created_at, parent_id
            )
            
            SELECT 
                inserted.id,
                u.username as "user_name",
                inserted.content,
                inserted.created_at,
                inserted.parent_id
            FROM inserted
            JOIN users u ON inserted.user_id = u.id
            "#,
            user_id,
            message.room_id,
            message.content,
            parent_id
        )
        .fetch_one(pool)
        .await?;

        Ok(message)
    }

    pub async fn load_recent_messages(
        pool: &PgPool,
        room_id: Uuid,
    ) -> Result<Vec<MessageResponse>> {
        let message: Vec<MessageResponse> = sqlx::query_as!(
            MessageResponse,
            r#"
        SELECT 
            m.id,
            u.username as "user_name!",
            m.content,
            m.created_at,
            m.parent_id
        FROM user_messages m
        JOIN users u ON m.user_id = u.id
        WHERE m.room_id = $1
        ORDER BY m.created_at DESC
        LIMIT 50
            "#,
            room_id
        )
        .fetch_all(pool)
        .await?;

        Ok(message)
    }

    pub async fn join_room(pool: &PgPool, room_id: &Uuid, user_id: &Uuid) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
            INSERT INTO members (user_id, room_id)
            VALUES ($1, $2)
            "#,
            user_id,
            room_id
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::Failed("Failed to leave room".into()));
        }

        Ok(())
    }

    pub async fn leave_room(pool: &PgPool, room_id: &Uuid, user_id: &Uuid) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
            DELETE from members
            WHERE room_id = $1 AND user_id = $2
            "#,
            room_id,
            user_id
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::Failed("Failed to leave room".into()));
        }

        Ok(())
    }

    pub async fn is_member(
        pool: &PgPool,
        user_id: &Uuid,
        room_id: &Uuid,
    ) -> Result<Option<bool>, AppError> {
        let exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS (
                SELECT 1 FROM members
                WHERE room_id = $1 AND user_id = $2
            )
            "#,
            room_id,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(exists)
    }

    pub async fn get_room_members(pool: &PgPool, room_id: &Uuid) -> Result<Vec<Members>, AppError> {
        let members = sqlx::query_as!(
            Members,
            r#"
            SELECT m.user_id, u.name, u.username
            FROM members m
            JOIN users u ON u.id = m.user_Id
            WHERE room_id = $1
            "#,
            room_id
        ).fetch_all(pool).await?;

        Ok(members)
    }
}