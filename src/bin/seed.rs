use quoteyourlife_be::db::establish_connection;
use quoteyourlife_be::seed::seed_quotes;

fn main() {
    let conn = establish_connection();
    seed_quotes(conn).expect("Failed to seed quotes");
    println!("Database seeded successfully.");
}