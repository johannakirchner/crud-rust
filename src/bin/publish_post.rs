use self::models::Post;
use diesel::{prelude::*, connection};
use diesel_demo::{*, schema::posts::published};
use std::env::args;

fn main() {
    use self::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requiresb a post id.")
        .parse::<i32>()
        .expect("invalid ID.");

    let connection = &mut establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(connection)
        .unwrap();

    println!("Published post {}", post.title);
}