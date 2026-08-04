fn main() {
    env_logger::init();
    log::info!("应用启动");
    log::warn!("磁盘空间不足");
    log::error!("数据库连接失败");

    tracing::info!(
        user_id = 42,
        action = "login",
        "用户登录成功"
    );
}
