use fugle::websocket::IntradayBuilder;

#[tokio::main]
async fn main() {
    let mut ws = IntradayBuilder::new().symbol_id("2884").odd_lot().build();

    println!("{:?}", ws.async_chart().await.unwrap().recv().await);
    println!("{:?}", ws.async_meta().await.unwrap().recv().await);
    println!("{:?}", ws.async_quote().await.unwrap().recv().await);
}
