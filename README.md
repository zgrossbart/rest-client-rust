# Simple Rest Client

This simple REST client is written in [Rust](https://www.rust-lang.org/) and was created as a sample and teaching tool.  I wrote this sample for a learning session I did with my team on Rust.  

This sample encompasses what a room full of Java, Python, and JavaScript programmers should know as an introduction to Rust.  ...but before we look at what we can learn from this example here's how to get it up and running.

## Running the Rest Client

Start by [installing Rust](https://www.rust-lang.org/tools/install) locally.

Once you have Rust installed you can use the [Cargo](https://doc.rust-lang.org/cargo/) build tool to build and run this project.

Start build building the project with this command:

```
cargo build
```

This will download all of the dependencies and build the project.

Once the build is complete you can run the project with this command:

```
cargo run zgrossbart
```

This will run the application and produce an output like this:

```
Compiling simple-rest-client v0.1.1 (/Users/zackgrossbart/rest-client-rust)
Finished dev [unoptimized + debuginfo] target(s) in 1.08s
Running `target/debug/rest-client-rust zgrossbart`

Getting GitHub user data for: zgrossbart
request_url: https://api.github.com/users/zgrossbart


User Data For: zgrossbart

ID:     664044
Login:  zgrossbart
Name:   Zack Grossbart

zgrossbart has 35 repositories and 103 followers
```

Try it out with any other username on github.com.  

## Key Points of This Sample

Here are some key points to take away from this sample:

**Rust is a specialized lanauge**.  It provides a lot of safety and speed, but it asks a lot up front to provide that.  Everything in Rust must be 