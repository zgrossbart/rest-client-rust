# Simple Rest Client

This simple REST client is written in [Rust](https://www.rust-lang.org/) and was created as a sample and teaching tool for team learning.

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