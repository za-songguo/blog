use serde::{Deserialize, Serialize};

use super::user::UserInfo;

/// 评论
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Comment {
    // 评论 ID
    pub id: Option<u32>,
    /// 发表评论的用户的信息
    /// 实现 Serialize 和 Deserialize
    pub user: Option<UserInfo>,
    /// 评论内容
    pub content: String,
    /// 评论日期
    pub date: Option<chrono::NaiveDate>,
    /// 评论的文章
    pub article: Option<u32>,
}
