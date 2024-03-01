mod data_types;

fn main() {

    let test_campaign = data_types::campaign::Campaign{name: "test name".to_string()};

    println!("Hello, world!");
    println!("{}", test_campaign.name);
}
