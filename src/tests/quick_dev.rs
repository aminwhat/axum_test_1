async fn quick_dev() -> Result<(), ()> {
    let hc = httpc_test::new_client("http://localhost:8080");
    // hc.do_get("/hello2/Mike").await?.print().await?;
    // let req_login = hc.do_post("");

    Ok(())
}