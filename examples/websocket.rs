use fugle::websocket::IntradayBuilder;

fn main() {
    let mut ws = IntradayBuilder::new().symbol_id("2884").odd_lot().build();

    println!("{:?}", ws.chart().unwrap().recv().unwrap());
    println!("{:?}", ws.meta().unwrap().recv().unwrap());
    println!("{:?}", ws.quote().unwrap().recv().unwrap());
}
