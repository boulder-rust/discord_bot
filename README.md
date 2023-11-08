# discord_bot

This is the Discord bot for the Boulder Rust meetup!

## Usage

Right now you need the Discord token for the Discord server, and only @zmitchell has that.

## Development

### Discord documentation
The Discord documentation is...extensive, and sometimes feels circular. You can find it here:
- [Discord Developer Portal - Documentation](https://discord.com/developers/docs/intro)

### Bot framework
We're using the [serenity](https://github.com/serenity-rs/serenity) crate to build the bot. As far as Discord bot frameworks go, this one appears to be somewhere in the middle in terms of complexity. This framework allows you to respond to all kinds of events, allowing you to respond whenever an interesting event comes by. As with all things, there's a trade-off. Being able to respond to all events means that you could create your own command framework by treating all messages that start with `!` as commands, or it could mean that you watch for certain phrases and send a reply in response. The downside is that I don't think `serenity` out of the box supports "slash commands". `serenity` comes with sparse _usage_ documentation, but does come with a solid set of examples in the repo.

The [poise](https://github.com/serenity-rs/poise) crate is opinionated and higher level. It pretty much only handles prefixed commands (e.g. `!foo`) and slash commands. I want to be able to do all sorts of shenanigans without needing to call a named command, so this doesn't fit our needs.

### Development environment

I've set this up so that you can more or less use whatever tools you want. If you'd like to use Nix, that's already set up for you. If you'd like to use a Docker-based setup, feel free, and please submit a PR with whatever Docker Compose setup you make for yourself.

The bot has access to a Postgres database in "production", so it will look for database credentials on startup. You can see the full list of environment variables you can set [here](https://docs.rs/sqlx-postgres/0.7.2/sqlx_postgres/struct.PgConnectOptions.html#method.new). If you're using something like Docker or Docker Compose, make sure you set these environment variables in both the bot container _and_ the database container so that they match up.

If you're an enlightened Nix user like me, just `nix develop` and it will set up the database in the repo along with a few tools to make it easy to manage the database and bot processes. Getting things up and running looks like this:
- In one terminal, enter the environment with `nix develop`. This will initialize Postgres.
- In that same terminal, call `hivemind`. `hivemind` is a process manager, meaning that it creates a collection of processes defined in a `Procfile`. This one starts a `bot` process that uses `cargo-watch` to restart the bot whenever you save changes, and starts a `db` process for Postgres.
- In a _**different**_ terminal call `just init-db`. This uses `just` to actually create the `bot_db` database, but it needs to talk to a running database to do so. This is why you have to start `hivemind` in a different terminal.

### Adding bot functionality
I've set things up so that bot functionality falls into three categories:
- "named" commands
- "implicit" commands
- "background" commands

This structure is partially so that we can all work on this at the same time with as few merge conflicts as possible :)

"Named" commands are those that start with `!`, where the command name is assumed to be the word immediately following the `!`. An example of this kind of command could be something like `!age` that displays the age of your account. These commands should live in `src/commands/named_commands.rs`.

"Implicit" commands are those that start with `~` and have no name as seen by the user. An example of this kind of command could be something that interprets messages of the form `~<key> is <value>` as "facts" and stores the key-value pairs in the database for recall with the `~key` command. For example, if you send a message saying `~@zmitchell is the greatest` then send a second message saying `~@zmitchell`, the bot would respond with `~@zmitchell is the greatest`. These commands should live in `src/commands/implicit_commands.rs`.

"Background" commands are those that don't have a prefix, and are just automated bot responses to certain events. An example of this kind of command would be something that scans each message for the mention of `@RoboFerris` and responds with `Beep boop to you, <author>`. These commands should live in `src/commands/background_commands.rs`.

### CI/CD
We use GitHub Actions to build and test. Currently we use Nix to do all of this in CI because it has pretty good caching (CI takes about 5 minutes right now if there's an rebuild that needs to happen). We also use Nix to build the container image since it can reuse all of the build artifacts from earlier stages of CI.

The container is deployed to DigitalOcean, where it's currently running on a toaster. The database is a generic Postgres container supplied by DigitalOcean.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
