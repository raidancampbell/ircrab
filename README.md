# ircrab

The fifth installment in my lineage of IRC bots with varying intelligence.
1. [Swiggityspeare](https://github.com/raidancampbell/swiggityspeare): a Java bot with exec wrappers around [Karpathy's char-rnn](https://github.com/karpathy/char-rnn) LSTM neural network.  It was incredibly fragile.
2. [Stupidspeare](https://github.com/raidancampbell/stupidspeare): a Python reimplementation with none of the neural network intelligence.
3. [Sequelspeare](https://github.com/raidancampbell/sequelspeare): Stupidspeare, but with a Tensorflow implementation of `char-rnn`.
4. [gossip](https://github.com/raidancampbell/gossip): a Golang reimplementation.  No neural networks again, this incarnation focuses on implementing  an IRC bot without an IRC framework library.  [sorcix/irc.v2](https://github.com/sorcix/irc/tree/v2) is used for protocol parsing.
5. ircrab: a very basic Rust reimplementation focusing on as much of the stdlib as possible. Protocol parsing is done by hand.

### Usage
1. modify `main.rs` to the desired configuration parameters
2. `cargo run`

### To-Do
 - [ ] support SSL
 - [ ] make the startup channel accessible to the `on_connect` trigger
 - [ ] look into removing the `once_cell::sync::Lazy` dependency
 - [ ] replace many of the String usages with `&str`
 - [ ] stop `unwrap`ing with reckless abandon
 - [ ] move the config to a dedicated file
 - [ ] add support for reading HTML titles from hyperlinks
 - [ ] respond to INVITE commands
 - [ ] parse the prefix (name, user, host) into the `message` struct
 - [ ] add support for sending logs to splunk