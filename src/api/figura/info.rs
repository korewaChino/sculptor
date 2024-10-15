use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::error;

use crate::{
    utils::{get_figura_versions, get_motd, FiguraVersions},
    AppState, FIGURA_DEFAULT_VERSION,
};

use super::types::badges::Badges;

pub async fn version(State(state): State<AppState>) -> Json<FiguraVersions> {
    let res = state.figura_versions.read().await.clone();
    if let Some(res) = res {
        Json(res)
    } else {
        let actual = get_figura_versions().await;
        if let Ok(res) = actual {
            let mut stored = state.figura_versions.write().await;
            *stored = Some(res);
            return Json(stored.clone().unwrap());
        } else {
            error!("get_figura_versions: {:?}", actual.unwrap_err());
        }
        Json(FiguraVersions {
            release: FIGURA_DEFAULT_VERSION.to_string(),
            prerelease: FIGURA_DEFAULT_VERSION.to_string(),
        })
    }
}

pub async fn motd(State(state): State<AppState>) -> String {
    serde_json::to_string_pretty(&get_motd(state).await).unwrap()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Limits {
    pub max_avatar_size: u64,
    pub max_avatars: u64,
    pub allowed_badges: Badges,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub ping_size: u64,
    pub ping_rate: u64,
    pub equip: u64,
    pub download: u64,
    pub upload: u64,
}

impl Default for RateLimit {
    fn default() -> Self {
        RateLimit {
            ping_size: 1024,
            ping_rate: 32,
            equip: 1,
            download: 50,
            upload: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServerLimits {
    pub rate: RateLimit,
    pub limits: Limits,
}

// todo: use the above code to implement this
pub async fn limits(State(state): State<AppState>) -> Json<Value> {
    let state = &state.config.read().await.limitations;

    let limits = ServerLimits {
        rate: RateLimit::default(),
        limits: Limits {
            max_avatar_size: state.max_avatar_size,
            max_avatars: state.max_avatars,
            allowed_badges: Badges::default(),
        },
    };

    Json(serde_json::to_value(limits).unwrap())

    // Json(json!({
    //     "rate": {
    //         "pingSize": 1024,
    //         "pingRate": 32,
    //         "equip": 1,
    //         "download": 50,
    //         "upload": 1
    //     },
    //     "limits": {
    //         "maxAvatarSize": state.max_avatar_size,
    //         "maxAvatars": state.max_avatars,
    //         "allowedBadges": {
    //             "special": [0,0,0,0,0,0],
    //             "pride": [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
    //         }
    //     }
    // }))
}

#[cfg(test)]
mod tests {
    use crate::api::figura::types::badges::PrideBadges;

    use super::*;
    use serde_json::json;
    #[test]
    fn test_deserialize() {
        let data = json!({
            "rate": {
                "pingSize": 1024,
                "pingRate": 32,
                "equip": 1,
                "download": 50,
                "upload": 1
            },
            "limits": {
                "maxAvatarSize": 1024,
                "maxAvatars": 50,
                "allowedBadges": {
                    "special": [0,0,0,0,0,0],
                    "pride": [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
                }
            }
        });

        let res: ServerLimits = serde_json::from_value(data).unwrap();

        println!("{:#?}", res);

        assert!(res.limits.allowed_badges.pride.agender);
    }

    #[test]
    fn test_deserialize_pride() {
        let data =
            json!([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1]);
        let res: PrideBadges = serde_json::from_value(data).unwrap();

        println!("{:#?}", res);

        assert!(res.pride_flag);
        assert!(res.trans);
        assert!(res.agender);
    }

    #[test]
    fn test_serialize() {
        let mut srv_lm = ServerLimits::default();

        srv_lm.limits.allowed_badges.special.figura_dev = true;

        let res = serde_json::to_value(srv_lm).unwrap();

        println!("{:#?}", res);

        assert_eq!(res["limits"]["allowedBadges"]["special"][0], 1);
    }
}
