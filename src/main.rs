use reqwest::ClientBuilder;
use reqwest::Result;
use serde::Deserialize;
use std::env;
use std::time::Duration;
use colored::Colorize;

/*
 * This structure defines our User object.  The names of the fields
 * must match the fields from the JSON data returned by the API.
 */
#[derive(Deserialize, Debug)]
struct User {
    login: String,
    name: String,
    id: u32,
    followers: u32,
    public_repos: u32,
}

/*
 * Our main function is the entry point to our application and also
 * asynchronous.
 */
#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    // We want to validate that we were passed in the username
    if args.len() != 2 {
        panic!("You must pass in a username");
    }

    let user_name = &args[1];
    println!("\n\nGetting GitHub user data for: {}", user_name);

    // Format the URL
    let request_url = format!("https://api.github.com/users/{}", user_name);
    // println!("request_url: {}", request_url);

    // Build the request so we can call the API
    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new()
        .timeout(timeout)
        .user_agent("application/rust")
        .build()?;

    // Send the request
    let response = client.get(&request_url).send().await?;

    // Look at the response
    if response.status().is_success() {
        // Uncomment the next two lines to print out the raw text of the response,
        // but you must comment out the JSON data response or you will get a
        // compiler error.
        // let text:String = response.text().await?;
        // println!("text: {}", text);

        // Parse the response as JSON data
        let user: User = response.json().await?;

        // Print out data about the user
        println!(
            "
{title}

ID:        {id}
Login:     {login}
Name:      {name}

{username} has {repos} repositories and {followers} followers

",
            title=format!("User Data For: {}", user_name).bright_blue().bold(),
            username = user.name,
            id = user.id,
            login = user.login,
            name = user.name,
            followers = user.followers,
            repos = user.public_repos
        );
    } else {
        println!("{} is not a user!", user_name);
    }

    Ok(())
}
