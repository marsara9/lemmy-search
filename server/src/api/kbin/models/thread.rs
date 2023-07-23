// use chrono::NaiveDateTime;
// use serde::{
//     Serialize, 
//     Deserialize
// };

// /api/magazines

// /api/magazine/{magazine_id}/posts/{sort}/{time}?p=#&perPage=#
// #[derive(Debug, Serialize, Deserialize, Clone, Default)]
// pub struct PostListRequest {
//     pub sort : KbinSort,
//     pub time : Option<KbinTime>,
//     pub p : Option<i64>,
//     pub per_page : Option<i64>
// }

// pub enum KbinSort {
//     Oldest // 'oldest'
// }

// pub enum KbinTime {
//     All // '∞'
// }

// pub struct KbinPost {
//     pub id : i64,
//     pub user : KbinUser,
//     pub magazine : Option<KbinMagazine>,
//     pub image : Option<KbinImage>,
//     pub slug : String,
//     pub body : Option<String>,
//     pub score : i64,
//     pub is_adult : bool
// }
