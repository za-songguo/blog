use crate::models::user::User;

use serde::{Deserialize, Serialize};

/// 评论
// 记得为 User 实现 Serialize
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Comment {
    /// 评论 ID
    pub id: Option<u32>,
    /// 发表评论的用户的信息
    /// 实现 Serialize 和 Deserialize
    pub user: Option<User>,
    /// 评论内容
    pub content: String,
    /// 评论日期
    pub date: Option<String>,
    /// 评论的文章
    pub article: Option<u32>,
}
