use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts.");

        println!("Displayng {} posts.\n", results.len());

        for post in results {
            println!("{} - {}", post.id,post.title);
            println!("----");
            println!("{}", post.body);
        }
}