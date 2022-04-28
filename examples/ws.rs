use fugle::ws::Intraday;

fn main() {
    let mut lis = Intraday::new("demo");

    println!("{:?}", lis.chart("2884", true).unwrap().recv().unwrap());
    println!("{:?}", lis.meta("2884", false).unwrap().recv().unwrap());
    println!("{:?}", lis.quote("2884", true).unwrap().recv().unwrap());
}
