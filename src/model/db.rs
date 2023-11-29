use rbatis::rbatis::RBatis;

// 链接数据库
pub async fn init_db() -> RBatis {
    let rb = RBatis::new();
    rb.init(
        rbdc_mysql::driver::MysqlDriver {},
        "mysql://root:root@127.0.0.1:3306/tc_db",
    )
    .unwrap();
    return rb;
}
