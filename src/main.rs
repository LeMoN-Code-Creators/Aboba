use weber::{HttpServer, parser::ContentType};

fn main() {
    let mut hui = HttpServer::new(1);
    hui.add_resource("/","./src/main.html",ContentType::Html);
    hui.run("127.0.0.1:8080");
    println!("Hello, world!");
}
