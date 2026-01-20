use diesel::prelude::*;
use diesel::result::QueryResult;
use crate::schema::quotes;
use crate::db::PgPool;

pub fn seed_quotes(pool: PgPool) -> QueryResult<()> {
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let quotes = vec![
        ("The only limit to our realization of tomorrow is our doubts of today.", "Franklin D. Roosevelt"),
        ("In the middle of every difficulty lies opportunity.", "Albert Einstein"),
        ("What you get by achieving your goals is not as important as what you become by achieving your goals.", "Zig Ziglar"),
    ];

    for (text, author) in quotes {
        diesel::insert_into(quotes::table)
            .values((
                quotes::text.eq(text),
                quotes::author.eq(author),
            ))
            .execute(&mut conn)?;    
    }
    Ok(())
}