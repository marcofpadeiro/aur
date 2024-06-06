use std::io::{self, Write};

use crate::helpers::{self, clone_package};
use async_trait::async_trait;

use crate::theme::{colorize, Type};

use super::handler::CommandHandler;

pub struct SearchHandler;

#[async_trait]
impl CommandHandler for SearchHandler {
    async fn handle(&self, matches: &clap::ArgMatches, config: &crate::config::Config) {
        let search_term = matches.get_one::<String>("search").unwrap();

        let packages = helpers::get_top_packages(&search_term).await;

        let len = packages.len();
        if len == 0 {
            println!("No packages found");
            return;
        }

        // print packages
        packages.iter().rev().enumerate().for_each(|(i, package)| {
            println!(
                "\n{} {}\n  {}",
                colorize(Type::Info, format!("{} ┃", len - i).as_str()),
                colorize(Type::Header, package.get_name()),
                package.get_description()
            );
        });

        print!("\nInstall package(s) (1-10) or (q)uit: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "q" || input == "quit" {
            return;
        }

        let parsed_input: Result<usize, _> = input.parse();

        match parsed_input {
            Ok(i) if i > 0 && i <= packages.len() => {
                match clone_package(&packages[i - 1], &config) {
                    Ok(_) => println!("   {}\n", colorize(Type::Success, "Package installed")),
                    Err(e) => println!("{} {}", colorize(Type::Error, "Error:"), e),
                }
            }
            _ => println!(
                "{}",
                colorize(Type::Warning, "Invalid input or package out of range")
            ),
        }
    }
}