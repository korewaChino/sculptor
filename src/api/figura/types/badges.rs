// todo: turn this into array of bools/ints

use serde::{de::IntoDeserializer, Deserialize, Serialize};
use struct_as_array::AsArray;

#[derive(Debug, Clone, AsArray, Default)]
pub struct SpecialBadges {
    pub figura_dev: bool,
    pub figura_mod: bool,
    pub contest_winner: bool,
    pub supporter: bool,
    pub translator: bool,
    pub texture_artist: bool,
}

impl From<Vec<bool>> for SpecialBadges {
    fn from(v: Vec<bool>) -> Self {
        let json = serde_json::to_value(v).unwrap();
        Self::deserialize(json.into_deserializer()).unwrap()
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

impl From<Vec<bool>> for PrideBadges {
    fn from(v: Vec<bool>) -> Self {
        let json = serde_json::to_value(v).unwrap();
        Self::deserialize(json.into_deserializer()).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]

// todo: Properly label each fields in the struct
pub struct Badges {
    // todo turn into vec of int on ser/de
    pub special: SpecialBadges,
    pub pride: PrideBadges,
}
