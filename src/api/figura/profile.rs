use crate::api::figura::types::badges::PrideBadges;
use axum::{
    body::Bytes,
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::{
    fs,
    io::{self, AsyncReadExt, BufWriter},
};
use tracing::debug;
use uuid::Uuid;

use super::types::{badges::SpecialBadges, S2CMessage};
use crate::{
    api::errors::internal_and_log,
    auth::Token,
    utils::{calculate_file_sha256, format_uuid},
    ApiError, ApiResult, AppState,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EquippedBadges {
    pub special: super::types::badges::SpecialBadges,
    pub pride: super::types::badges::PrideBadges,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub uuid: Uuid,
    pub rank: String,
    pub last_used: String,
    // todo: type this properly
    pub equipped: Vec<Value>,
    pub equipped_badges: EquippedBadges,
    pub version: String,
    pub banned: bool,
}
impl User {
    pub async fn user_info(uuid: Uuid, state: &AppState) -> Result<Self, ApiError> {
        let formatted_uuid = format_uuid(&uuid);

        let avatar_file = format!("avatars/{}.moon", formatted_uuid);

        let userinfo = if let Some(info) = state.user_manager.get_by_uuid(&uuid) {
            info
        } else {
            return Err(ApiError::BadRequest); // NOTE: Not Found (404) shows badge
        };
        let mut user_info = User {
            uuid,
            rank: userinfo.rank.clone(),
            last_used: userinfo.last_used.clone(),
            equipped: vec![],
            equipped_badges: EquippedBadges::default(),
            version: userinfo.version.clone(),
            banned: userinfo.banned,
        };

        if let Some(settings) = state.config.read().await.advanced_users.get(&uuid) {
            user_info.equipped_badges.special = SpecialBadges::from(
                settings
                    .special
                    .iter()
                    .map(|&x| x != 0)
                    .collect::<Vec<bool>>(),
            );
            user_info.equipped_badges.pride = PrideBadges::from(
                settings
                    .pride
                    .iter()
                    .map(|&x| x != 0)
                    .collect::<Vec<bool>>(),
            );
        }
        // Ok(user_info)

        if fs::metadata(&avatar_file).await.is_ok() {
            match calculate_file_sha256(&avatar_file) {
                Ok(hash) => user_info.equipped.push(json!({
                    "id": "avatar",
                    "owner": &formatted_uuid,
                    "hash": hash
                })),
                Err(e) => {
                    tracing::error!("Failed to calculate SHA256 of avatar: {:?}", e)
                }
            }
        }

        Ok(user_info)
    }

    pub async fn upload_avatar(
        token: String,
        state: &AppState,
        body: Bytes,
    ) -> Result<(), ApiError> {
        let request_data = body;

        if let Some(user_info) = state.user_manager.get(&token) {
            tracing::info!(
                "{} ({}) trying to upload an avatar",
                user_info.uuid,
                user_info.username
            );
            let avatar_file = format!("avatars/{}.moon", user_info.uuid);
            let mut file = BufWriter::new(
                fs::File::create(&avatar_file)
                    .await
                    .map_err(internal_and_log)?,
            );
            io::copy(&mut request_data.as_ref(), &mut file)
                .await
                .map_err(internal_and_log)?;
        }
        Ok(())
    }
}

pub async fn user_info(
    Path(uuid): Path<Uuid>,
    State(state): State<AppState>,
) -> ApiResult<Json<Value>> {
    tracing::info!("Receiving profile information for {}", uuid);

    let user_info = User::user_info(uuid, &state).await?;

    Ok(Json(
        serde_json::to_value(user_info).map_err(internal_and_log)?,
    ))
}

pub async fn download_avatar(Path(uuid): Path<Uuid>) -> ApiResult<Vec<u8>> {
    let uuid = format_uuid(&uuid);
    tracing::info!("Requesting an avatar: {}", uuid);
    let mut file = if let Ok(file) = fs::File::open(format!("avatars/{}.moon", uuid)).await {
        file
    } else {
        return Err(ApiError::NotFound);
    };
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .await
        .map_err(internal_and_log)?;
    Ok(buffer)
}

pub async fn upload_avatar(
    Token(token): Token,
    State(state): State<AppState>,
    body: Bytes,
) -> ApiResult<String> {
    User::upload_avatar(token, &state, body).await?;
    Ok("ok".to_string())
}

pub async fn equip_avatar(
    Token(token): Token,
    State(state): State<AppState>,
) -> ApiResult<&'static str> {
    debug!("[API] S2C : Equip");
    let uuid = state
        .user_manager
        .get(&token)
        .ok_or(ApiError::Unauthorized)?
        .uuid;
    send_event(&state, &uuid).await;
    Ok("ok")
}

pub async fn delete_avatar(
    Token(token): Token,
    State(state): State<AppState>,
) -> ApiResult<String> {
    if let Some(user_info) = state.user_manager.get(&token) {
        tracing::info!(
            "{} ({}) is trying to delete the avatar",
            user_info.uuid,
            user_info.username
        );
        let avatar_file = format!("avatars/{}.moon", user_info.uuid);
        fs::remove_file(avatar_file)
            .await
            .map_err(internal_and_log)?;
        send_event(&state, &user_info.uuid).await;
    }
    // let avatar_file = format!("avatars/{}.moon",user_info.uuid);
    Ok("ok".to_string())
}

pub async fn send_event(state: &AppState, uuid: &Uuid) {
    // To user subscribers
    if let Some(broadcast) = state.broadcasts.get(uuid) {
        if broadcast.send(S2CMessage::Event(*uuid).to_vec()).is_err() {
            debug!("[WebSocket] Failed to send Event! There is no one to send. UUID: {uuid}")
        };
    } else {
        debug!("[WebSocket] Failed to send Event! Can't find UUID: {uuid}")
    };
    // To user
    if let Some(session) = state.session.get(uuid) {
        if session
            .send(S2CMessage::Event(*uuid).to_vec())
            .await
            .is_err()
        {
            debug!("[WebSocket] Failed to send Event! WS doesn't connected? UUID: {uuid}")
        };
    } else {
        debug!("[WebSocket] Failed to send Event! Can't find UUID: {uuid}")
    };
}
