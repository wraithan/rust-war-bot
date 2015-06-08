# warlight-starterbot-rust

This is a monte carlo bot that includes the whole framework to get going with rust and theaigames.com Warlight 2 challenge.

## building

This is built as a standard rust project.

```
cargo build --release
```

This creates a binary `./target/release/warbot` which is what should be run as the bot in the tournament.

## api docs

I need to write these and until then this wont be a good starter bot. But it should be good enough to get rust accepted on the platform.

## testing

The integration tests include markup a framework that my other bot [ZenWarBot](https://github.com/wraithan/zenwarbot) uses which was pioneered by [Curious Attempt Bunny](http://curiousattemptbunny.com/) in his [Clojure Bot]((https://github.com/curious-attempt-bunny/warlight2-starterbot-clojure)). You can find the spec for them [here](https://github.com/curious-attempt-bunny/warlight2-starterbot-clojure#create-new-tests).

I didn't fully implement it yet, my rust bot [rust-war-bot](https://github.com/wraithan/rust-war-bot) will have a more complete implementation to pull things from.

## license

[ISC](http://en.wikipedia.org/wiki/ISC_license)

