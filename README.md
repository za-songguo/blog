# 一个博客项目
分为 client 和 server，基本全部使用 Rust 语言

client 使用 Yew 编写（WebAssembly），server 则是使用了 ntex 这个高性能的 Web 框架

后端使用 PostgreSQL 数据库，建表语句在项目根目录下

前端使用 Picnic CSS 这个 CSS 框架

包含增删改查和用户鉴权等功能，支持使用 Github 登录，但是得先在 Github 上注册一个自己的应用，然后替换掉服务端代码里面的 `CLIENT_ID` 和 `CLIENT_SECRET` 这两个常量（`constants.rs`），客户端代码里的 `CLIENT_ID`（`constants.rs`） 和 `test.html` 里的 client_id，具体操作步骤可以去看视频