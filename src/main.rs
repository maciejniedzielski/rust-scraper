use colored::Colorize;
use reqwest;
use select::document::Document;
use select::predicate::{Class, Name};

fn main() {
    let url = "https://www.angular.love/";

    match make_request(url) {
        Ok(res) => handle_success(url, res),
        Err(error) => handle_error(error),
    }
}

fn make_request(url: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
    return reqwest::blocking::get(url);
}

fn handle_error(error: reqwest::Error) {
    match error.url() {
        None => print_error("No URL given!"),
        Some(url) => print_error(&format!("Problem making request to: {}", url.as_str())),
    }
}

fn handle_success(url: &str, res: reqwest::blocking::Response) {
    println!("\nStatus for {}: {}", url, res.status().as_str().green());

    let document = Document::from_read(res).unwrap();

    for node in document.find(Class("entry-title")) {
        let text = node.find(Name("a")).next().unwrap().text();
        println!("\n📃 {}", text.cyan());
        let url = node.find(Name("a")).next().unwrap();
        println!("{}", url.attr("href").unwrap());
    }
}

fn print_error(message: &str) {
    println!("\n{} {}", "ERROR:".white().on_red().bold(), message)
}
