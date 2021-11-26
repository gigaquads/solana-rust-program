# Solana Router

## Background & Motivation
Developers who choose to explore Solana currently have two options when getting
started: they can either try to learn the [Solana
SDK](https://docs.solana.com/developing/clients/javascript-api) or jump right
into
[Anchor](https://project-serum.github.io/anchor/getting-started/introduction.html),
the only high-level Solana framework (currently available). Those who choose the
former hit a wall when they discover that virtually no online tutorials or
"stack overflows" exist that go beyond the ubiquitous yet overly-simplistic,
inextensible ["Hello, world!"
example](https://github.com/solana-labs/example-helloworld). On the other hand,
those who choose the latter hit a wall as soon as they encounter their first
error while trying to go beyond the ideal use-cases around which Anchor was
designed, because their fundamental understanding is handicapped by Anchor's
all-powerful yet totally opaque macros.

In contrast, `solana-router` is somewhere in the middle. It builds upon the SDK
but isn't a framework. It's more of a scaffold for new projects. It's the 
[Bootstrap](https://getbootstrap.com) of Solana. Unlike Anchor, there are no
magic macros that do all the heavy lifting. Instead, control flow is completely
transparent and customizable, designed around the familiar metaphor of "request
routing" found in traditional web frameworks. This design should make Solana
development more intuitive for engineers comming from a background in full-stack
web development.


## Program Architecture
In a nutshell, the program receives a Solana instruction. Processing of the
instruction is routed to an appropriate handler, based on the contents of its
data.


## Project Layout
- `lib.rs` - Defines main program entrypoint.
- `accounts.rs` - Account helper functions and data structures.
- `auth.rs` - Helper functions for auth-related tasks.
- `router.rs` - Routes instructions to available routes.
- `routes`
  - `mod.rs` - Registry for enabled routes.
  - `create_lucky_number.rs` - Example route to create a "lucky number".
  - `update_lucky_number.rs` - Example route to update a "lucky number".


## Adding Routes
To add a route, either create a new file in `src/routes` or duplicate
`create_lucky_number.rs` as a starting point. After this, register the your new
module in `src/routes/mod.rs` by adding it to the `ROUTES_ENABLED` array. That's
it!

## Build & Deploy
To build and deploy the application, you can use `bin/build` and `bin/deploy`,
respectively. If you want to know what these are doing, have a look inside. It's
relatively straight-forward.