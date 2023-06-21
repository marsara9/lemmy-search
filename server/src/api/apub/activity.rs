// use url::Url;
// use serde::{
//     Deserialize, 
//     Serialize
// };
// use serde_json::{
//     Map, 
//     Value
// };
// use activitypub_federation::{
//     fetch::object_id::ObjectId,
//     kinds::activity::AnnounceType,
//     protocol::helpers::deserialize_one_or_many,
// };

// use crate::api::lemmy::models::community::Community;


// #[derive(Clone, Debug)]
// pub struct ApubCommunity(Community);

// #[derive(Clone, Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct AnnounceActivity {
//   pub(crate) actor: ObjectId<ApubCommunity>,
//   #[serde(deserialize_with = "deserialize_one_or_many")]
//   pub(crate) to: Vec<Url>,
//   pub(crate) object: IdOrNestedObject<RawAnnouncableActivities>,
//   #[serde(deserialize_with = "deserialize_one_or_many")]
//   pub(crate) cc: Vec<Url>,
//   #[serde(rename = "type")]
//   pub(crate) kind: AnnounceType,
//   pub(crate) id: Url,
// }

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct RawAnnouncableActivities {
//     pub id : Url,
//     pub actor : Url,
//     #[serde(flatten)]
//     pub other : Map<String, Value>
// }
