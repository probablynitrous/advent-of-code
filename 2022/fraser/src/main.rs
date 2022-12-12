mod day_1;
#[tokio::main]
async fn main() {
    let day = std::env::args().nth(1).unwrap();
    match day.as_str() {
        "1" => {
            tokio::spawn(async move { day_1::part_1().await });
            tokio::spawn(async move { day_1::part_2().await });
        }
        _ => {
            print!("Couldn't find day {}", day);
        }
    }
}
