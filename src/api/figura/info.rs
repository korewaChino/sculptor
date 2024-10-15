use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use struct_as_array::AsArray;
use tracing::error;

use crate::{
    utils::{get_figura_versions, get_motd, FiguraVersions},
    AppState, FIGURA_DEFAULT_VERSION,
};

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

trait AsIntArray {
    fn as_int_array(&self) -> Vec<u8>;
}

pub async fn motd(State(state): State<AppState>) -> String {
    serde_json::to_string_pretty(&get_motd(state).await).unwrap()
}

// todo: turn this into array of bools/ints
#[derive(Debug, Clone, AsArray, Default)]
pub struct SpecialBadges {
    pub figura_dev: bool,
    pub figura_mod: bool,
    pub contest_winner: bool,
    pub supporter: bool,
    pub translator: bool,
    pub texture_artist: bool,
}
impl AsIntArray for SpecialBadges {
    fn as_int_array(&self) -> Vec<u8> {
        self.clone().to_array().iter().map(|x| *x as u8).collect()
    }
}

// Serialize to array of ints
impl Serialize for SpecialBadges {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.clone()
            .to_array()
            .iter()
            .map(|&x| x as u8)
            .collect::<Vec<u8>>()
            .serialize(serializer)
    }
}

// Deserialize from array of ints
impl<'de> Deserialize<'de> for SpecialBadges {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let arr: Vec<u8> = Vec::deserialize(deserializer)?;
        if arr.len() != 6 {
            return Err(serde::de::Error::custom("expected array of length 6"));
        }
        Ok(SpecialBadges {
            figura_dev: arr[0] != 0,
            figura_mod: arr[1] != 0,
            contest_winner: arr[2] != 0,
            supporter: arr[3] != 0,
            translator: arr[4] != 0,
            texture_artist: arr[5] != 0,
        })
    }
}

#[derive(Debug, Clone, AsArray, Default)]
pub struct PrideBadges {
    pub agender: bool,
    pub aroace: bool,
    pub aromantic: bool,
    pub asexual: bool,
    pub bigender: bool,
    pub bisexual: bool,
    pub demiboy: bool,
    pub demigender: bool,
    pub demigirl: bool,
    pub demiromantic: bool,
    pub demisexual: bool,
    pub disabled: bool,
    pub finsexual: bool,
    pub gay: bool,
    pub genderfae: bool,
    pub genderfluid: bool,
    pub genderqueer: bool,
    pub intersex: bool,
    pub lesbian: bool,
    pub non_binary: bool,
    pub pansexual: bool,
    pub plural: bool,
    pub poly: bool,
    pub pride_flag: bool,
    pub trans: bool,
}

impl Serialize for PrideBadges {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.clone()
            .to_array()
            .iter()
            .map(|&x| x as u8)
            .collect::<Vec<u8>>()
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PrideBadges {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let arr: Vec<u8> = Vec::deserialize(deserializer)?;
        if arr.len() != 25 {
            return Err(serde::de::Error::custom("expected array of length 25"));
        }
        Ok(PrideBadges {
            agender: arr[0] != 0,
            aroace: arr[1] != 0,
            aromantic: arr[2] != 0,
            asexual: arr[3] != 0,
            bigender: arr[4] != 0,
            bisexual: arr[5] != 0,
            demiboy: arr[6] != 0,
            demigender: arr[7] != 0,
            demigirl: arr[8] != 0,
            demiromantic: arr[9] != 0,
            demisexual: arr[10] != 0,
            disabled: arr[11] != 0,
            finsexual: arr[12] != 0,
            gay: arr[13] != 0,
            genderfae: arr[14] != 0,
            genderfluid: arr[15] != 0,
            genderqueer: arr[16] != 0,
            intersex: arr[17] != 0,
            lesbian: arr[18] != 0,
            non_binary: arr[19] != 0,
            pansexual: arr[20] != 0,
            plural: arr[21] != 0,
            poly: arr[22] != 0,
            pride_flag: arr[23] != 0,
            trans: arr[24] != 0,
        })
    }
}

impl AsIntArray for PrideBadges {
    fn as_int_array(&self) -> Vec<u8> {
        self.clone().to_array().iter().map(|x| *x as u8).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]

// todo: Properly label each fields in the struct
pub struct Badges {
    // todo turn into vec of int on ser/de
    pub special: SpecialBadges,
    pub pride: PrideBadges,
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
    Json(json!({
        "rate": {
            "pingSize": 1024,
            "pingRate": 32,
            "equip": 1,
            "download": 50,
            "upload": 1
        },
        "limits": {
            "maxAvatarSize": state.max_avatar_size,
            "maxAvatars": state.max_avatars,
            "allowedBadges": {
                "special": [0,0,0,0,0,0],
                "pride": [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
            }
        }
    }))
}

#[cfg(test)]
mod tests {
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
