use serde::{Deserialize, Serialize};

/// 用户信息
// 注意要实现 PartialEq
#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct User {
    /// Github 用户 ID
    pub id: u32,
    /// 用户名(不是昵称)
    pub login: String,
    /// 用户头像的地址
    pub avatar_url: String,
    /// 是否为管理员
    pub is_admin: bool,
}

/// 用于 OAuth 登录时从路径中提取 query 参数和向后端发起请求
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Login {
    pub code: String,
}
