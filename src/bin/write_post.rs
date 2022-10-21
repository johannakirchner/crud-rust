use diesel::connection;
use diesel_demo::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("What would you like the title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    println!(
        "Let's write {} (Press CTRL+Z when fineished)\n",
        title
    );

    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(connection, title, &body);
    println!("\nSaved {} with id {}", post.title, post.id);

}