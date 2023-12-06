use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
	pub uploader: Uploader,
	pub name: String,
	pub date_uploaded: DateUploaded,
	pub unsubmitted_beatmap_count: i64,
	pub id: i64,
	pub description: String,
	pub date_last_modified: DateLastModified,
	pub unknown_ids: Option<Vec<Value>>,
	pub modes: Modes,
	pub beatmap_count: i64,
	pub difficulty_spread: DifficultySpread,
	pub bpm_spread: BpmSpread,
	pub beatmapsets: Vec<Beatmapset>,
	pub unknown_checksums: Vec<Value>,
	pub favourited_by: Option<Vec<i64>>,
	pub favourites: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uploader {
	#[serde(rename = "avatarURL")]
	pub avatar_url: String,
	pub rank: Option<i64>,
	pub id: i64,
	pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateUploaded {
	#[serde(rename = "_seconds")]
	pub seconds: i64,
	#[serde(rename = "_nanoseconds")]
	pub nanoseconds: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateLastModified {
	#[serde(rename = "_seconds")]
	pub seconds: i64,
	#[serde(rename = "_nanoseconds")]
	pub nanoseconds: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Modes {
	pub osu: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DifficultySpread {
	#[serde(rename = "1")]
	pub n1: i64,
	#[serde(rename = "2")]
	pub n2: i64,
	#[serde(rename = "3")]
	pub n3: i64,
	#[serde(rename = "4")]
	pub n4: i64,
	#[serde(rename = "5")]
	pub n5: i64,
	#[serde(rename = "6")]
	pub n6: i64,
	#[serde(rename = "7")]
	pub n7: i64,
	#[serde(rename = "8")]
	pub n8: i64,
	#[serde(rename = "9")]
	pub n9: i64,
	#[serde(rename = "10")]
	pub n10: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmSpread {
	#[serde(rename = "150")]
	pub n150: i64,
	#[serde(rename = "160")]
	pub n160: i64,
	#[serde(rename = "170")]
	pub n170: i64,
	#[serde(rename = "180")]
	pub n180: i64,
	#[serde(rename = "190")]
	pub n190: i64,
	#[serde(rename = "200")]
	pub n200: i64,
	#[serde(rename = "210")]
	pub n210: i64,
	#[serde(rename = "220")]
	pub n220: i64,
	#[serde(rename = "230")]
	pub n230: i64,
	#[serde(rename = "240")]
	pub n240: i64,
	#[serde(rename = "250")]
	pub n250: i64,
	#[serde(rename = "260")]
	pub n260: i64,
	#[serde(rename = "270")]
	pub n270: i64,
	#[serde(rename = "280")]
	pub n280: i64,
	#[serde(rename = "290")]
	pub n290: i64,
	#[serde(rename = "300")]
	pub n300: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beatmapset {
	pub beatmaps: Vec<Beatmap>,
	pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beatmap {
	pub checksum: String,
	pub id: i64,
}
