In most real-world libraries, services are what you will probably create the most. This folder
shows various ways to author them.

| File                                     | Description                                           |
|------------------------------------------|-------------------------------------------------------|
| [`basic.rs`](basic.rs)                   | A very simple service, start here. 🍼                 |
| [`callback.rs`](callback.rs)             | Passing callbacks to and invoking them from services. |
| [`ignored.rs`](ignored.rs)               | Ignoring methods.                                     |
| [`lifetime.rs`](lifetime.rs)             | Services utilizing lifetimes. Slightly dangerous ⚠️   |
| [`multiple_ctors.rs`](multiple_ctors.rs) | Providing multiple constructors.                      |
| [`on_panic.rs`](on_panic.rs)             | Specifying panic behavior.                            |
| [`slice.rs`](slice.rs)                   | Sending and receiving slices.                         |
| [`string.rs`](string.rs)                 | Sending and receiving strings.                        |
