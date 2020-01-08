# Rust Trait Message Threaded

This repository is just a POC of how we can use traits to define behaviours of messages being sent through a channel.

The idea is to ensure each message structure implements its own "process" function, instead of using Enums to manually
match elements and dispatch them accordingly.

This repository might be updated if I find a better way to do it, but with current Rust features (not nightly),
I don't think there is an easier way.
