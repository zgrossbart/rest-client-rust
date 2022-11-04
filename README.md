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

**Rust is a specialized lanauge**.  It provides a lot of safety and speed, but it asks a lot up front to provide that.

**All values are required** in our `User` struct.  It takes a little more work to support optional parameters and I'll show you how that works in the next section.

**Rust has great support for threading**.  This example creates a simple client for a threaded application with the `async` keyword and the `tokio` package.  Compare that to the threading model of a language like Java where you would need to manage your own threading in the `main` method.

## Add an Optional Parameter

Now we'll expand the example to add an optional parameter to our client.  The REST API has a field called `email`, but it's `null` for many users.  In a language like Python we could just include that, but with the extra type checking of Rust it takes a little more work.  

You can see this example by checking out the `with-option` branch with a command like this:

```
git checkout with-option
```

This branch has a few key changes.  The first is that we've added a new crate called `serde_with`.  This give us support for optional parameters.  

With that support we've added the `email` field to our `User` struct like this:

```rust
struct User {
    login: String,
    name: String,
    id: u32,
    followers: u32,
    public_repos: u32,
    email: Option<String>,
}
```

Next we've added the `email` field to our output like this:

```rust
Email:  {email}
```

Lastly we need to pass in the value for this field like this:

```rust
email = user.email.unwrap_or("<The email isn't set>".to_string())
```

It's interesting to see that we need to use the `unwrap_or` function here or the compiler will give us an error because the `user.email` field could be null.  The option option would be to specify the `email` paramater with some extra details to allow the compiler to know it could be null like this:

```rust
Email:  {email:?}
```
