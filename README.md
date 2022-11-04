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

**Rust is a specialized lanauge**.  It provides a lot of safety and speed, but it asks a lot up front.

**Rust isn't a good first programming language**.  Rust expects you to have a lot of knowledge of programming languages.  An easy example of that is the `&` operator and the need to understand pointers.

**All values are required** in our `User` struct.  It takes a little more work to support optional parameters and I'll show you how that works in the next section.

**Rust has great support for threading**.  This example creates a simple client for a threaded application with the `async` keyword and the `tokio` package.  Compare that to the threading model of a language like Java where you would need to manage your own threading in the `main` method.

## Learning Rust with Compiler Errors

A good way to get an introduction to Rust is understanding compiler errors.  Let's take a look at a few from this project.

### `not found in this scope`

What happens if you comment out one of the parameters to the `println!` macro.  Let's comment out line 77 so the call looks like this:

```rust
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
//            id = user.id,
            login = user.login,
            name = user.name,
            followers = user.followers,
            repos = user.public_repos
        );
```

Now run the `cargo build` command and you'll get the following output.  

```
error[E0425]: cannot find value `id` in this scope
  --> src/main.rs:68:13
   |
68 | ID:        {id}
   |             ^^ not found in this scope
   |
help: consider importing this function
   |
1  | use std::process::id;
   |
```

This error is telling us that the parameter `id` wasn't passed in.  In a language like Python this code would be allowed and would just create an unexpected output at runtime.  Even Java which is much more statically typed would still allow this.

Rust is using the compiler to make sure that this program produces the expected result.

### `borrow of moved value: 'args'`

The first thing we do in the `main` function is to get the arguments passed in and validate that we were sent the correct number of arguments like this:

```rust
let args: Vec<String> = env::args().collect();
// dbg!(&args);

// We want to validate that we were passed in the username
if args.len() != 2 {
    panic!("You must pass in a username");
}
```

The call to the `dbg!` macro is commmented out, but if we uncommented it then we would see the list of arguments when we ran the program.  What would happen if we uncommented the line and removed the `&` like this:

```rust
let args: Vec<String> = env::args().collect();
dbg!(args);

// We want to validate that we were passed in the username
if args.len() != 2 {
    panic!("You must pass in a username");
}
```

Building this code produces the following error:

```
error[E0382]: borrow of moved value: `args`
  --> src/main.rs:31:8
   |
27 |     let args: Vec<String> = env::args().collect();
   |         ---- move occurs because `args` has type `Vec<std::string::String>`, which does not implement the `Copy` trait
28 |     dbg!(args);
   |     ---------- value moved here
...
31 |     if args.len() != 2 {
   |        ^^^^^^^^^^ value borrowed here after move
```

This is an interesting error.  The problem is that we're passing the variable `args` directly to the `dbg!` macro.  We don't know if that variable will be changed by the function and the Rust runtime can't copy the value because `Vec` doesn't support it.  That would be OK if we never use the `args` variable again, but when we use it a couple of lines later to check the length of the arguments passed in then the Rust compiler throws an error.

I've had this exact experience in Java.  I passed an `ArrayList` to another method and didn't expect the other method to change the list.  It did and then my program manfunctioned.  The Rust compiler is making sure you don't run into this problem and turning a potential run-time error into a compile-time error.

### `value used here after move`

What if we wanted to see the full text output of the REST API for debugging?  In that case we could uncomment line 57 and 58 like this:

```rust
let text:String = response.text().await?;
println!("text: {}", text);
```

If we compile that change we'll get the following error:

```
error[E0382]: use of moved value: `response`
   --> src/main.rs:61:26
    |
50  |     let response = client.get(&request_url).send().await?;
    |         -------- move occurs because `response` has type `Response`, which does not implement the `Copy` trait
...
57  |         let text:String = response.text().await?;
    |                                    ------ `response` moved due to this method call
...
61  |         let user: User = response.json().await?;
    |                          ^^^^^^^^ value used here after move
    |
note: this function takes ownership of the receiver `self`, which moves `response`
   --> /Users/zackgrossbart/.cargo/registry/src/github.com-1ecc6299db9ec823/reqwest-0.11.12/src/async_impl/response.rs:146:23
    |
146 |     pub async fn text(self) -> crate::Result<String> {
    |
```

This error in Rust is magical.  Basically it's tell us that the `response` object is being accessed after the contents have already been moved.  This happens because we're trying to access fields and functions in that object again when we try to get the JSON data like this:

```rust
let user: User = response.json().await?;
```

The concern is that one of the calls may be changing the object in a way that the other doesn't anticiapte.  This may seem like an unlikely case, but I've had to debug this specific problem and it was very difficult to understand what was wrong.


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
