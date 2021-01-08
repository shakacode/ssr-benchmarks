# SSR benchmarks

Benchmarks of [`ssr`](https://crates.io/crates/ssr) crate against Node.js and Next.js servers.

## Prerequisites
You will need [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html), [`npm`](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm), [`postgres`](https://www.postgresql.org/download/) & [`sqlx` cli](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli) installed on your system.

## Setup and usage
Make sure you have Postgres instance running. If you use Docker, you can use provided `docker-compose.yml`:

```sh
docker-compose up
```

Setup environment:

```sh
scripts/setup
```

After that, you can fine-tune some values in `.env` file, if you'd like to.

Then in separate terminals, run 3 servers:

```sh
scripts/run-rust-server
scripts/run-node-server
scripts/run-next-server
```

And finally, run benchmarks:

```sh
scripts/bench
```

By default, benchmark script executes 10 concurrent connections. But you can pass your number to the script:

```sh
scripts/bench 25
```
