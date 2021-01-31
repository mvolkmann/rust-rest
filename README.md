# rust-rest

The repository demonstrates and benchmarks popular approaches
for implement REST services.
HTTP server libraries that can be used to implement REST services
exist for many programming languages.
We will limit our focus to Rust, Node.js, and Deno.

Popular Rust frameworks for implementing REST services include:

- [actix-web](https://crates.io/crates/actix-web)
- [rocket](https://crates.io/crates/rocket)
- [warp](https://crates.io/crates/warp)
- [tide](https://crates.io/crates/tide)

These are Rust frameworks are compared at {% aTargetBlank
[Choosing a Rust web framework, 2020 edition](https://www.lpalmieri.com/posts/2020-07-04-choosing-a-rust-web-framework-2020-edition/).

The most popular Node.js library for implementing REST services is
[express](https://expressjs.com).

The most popular Deno library for implementing REST services is
[oak](https://oakserver.github.io/oak/).

In order to demonstrate using each of these frameworks,
we will implement the same set of CRUD REST services
that operate on a collection of dog descriptions.
Each dog is described by an id, name, and breed.
Typically the data would be persisted to a database, but
to keep the focus on the frameworks we will just hold the data in memory.
This requires learning how each framework manages application state,
which is a side benefit of this approach.

Each of the servers will run on localhost using port 1234
to avoid conflicting with other commonly used ports.
The endpoints exposed by each framework will be:

- GET /dog - to retrieve all the dogs
- GET /dog/{id} - to retrieve a specific dog
- POST /dog - to create a dog
- PUT /dog/{id} - to update a dog
- DELETE /dog/{id} - to delete a dog

Rust-based REST services are implemented in the
`actix-web`, `rocket`, `tide`, and `warp` directories.
Deno-based REST services are implemented in the `oak` directory.
Node-based REST services are implemented in the `express` directory.

The `benchmark` directory contains the file `tests/test.rs`
which is a Rust program that uses the `reqwest` crate
to send HTTP requests to the currently running server implementation.
It deletes any existing dogs, creates two dogs, deletes the first dog,
updates the name of the second dog, and verifies that all of these
operations result in the expected response status and body.

The `benchmark` directory also contains the file `src/main.rs`
which is a Rust program that, like the test program, uses the `reqwest` crate
to send HTTP requests to the currently running server implementation.
It deletes any existing dogs, starts a timer, creates 10,000 dogs
retrieves all of them in a single request,
updates all of them, deletes all of them, and reports the elapsed time.

The latest benchmark results using release builds
of the servers and the benchmark code were:

- Rust/actix-web: 3.062 seconds
- Rust/warp: 3.229 seconds
- Rust/tide: 4.112 seconds
- Rust/rocket-web: 4.248 seconds
- Node/express: 7.550 seconds
- Deno/oak: 8.725 seconds

To build a release version of a Rust-based server,
cd to its directory and enter `cargo build --release`.
To run this, enter `./target/release/{executable-name}`.

To install the dependencies needed by the `express` server,
cd to its directory and enter `npm install`.
To run this, enter `npm start`.

To install the dependencies needed by the `oak` server
and run it, cd to its directory and enter `./run`.

To run the tests against the currently running server implementation,
open a terminal in the `benchmark` directory and enter `cargo test`.

To build a release version of the Rust-based benchmark program,
cd to the `benchmark` directory and enter `cargo build --release`.
To run this, enter `./target/release/benchmark`.

If you are a maintainer or user of any of the frameworks
demonstrated here and feel that the way I have used them
is not idiomatic or not as efficient as it could be
PLEASE let me know!
I'm happy to make changes in order to
show the framework in the best possible light.

What kind of people goes to a forum page nearly every day
to see of someone has a question they can answer?
There are many very helpful people that do this
on the [Rust Forum](https://users.rust-lang.org).
I could not have implemented all of this without their help.
Thank you to these and more,
some of whom I could not determine their real name!

- Alice Ryhl (Alice)
- Cole Miller (cole-miller)
- Dan Bruder (danbruder)
- H2CO3
- John Chabot (jonnyboyC)
- Kestrer
- Kornel
- Matt Brubeck (mbrubeck)
- Patrik Buhring (OptimisticPeach)
- Quine Dot (quinedot)
- scottmcm
- ZiCog
- 17cupsofcoffee
- 2e71828
