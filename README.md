[![Crates.io Version](https://img.shields.io/crates/v/rapids-rs)](https://crates.io/crates/rapids-rs) [![docs.rs](https://img.shields.io/docsrs/rapids-rs)](https://docs.rs/rapids-rs/) [![License: AGPL-3.0-only](https://img.shields.io/badge/License-AGPL--3.0--only-93defa)](https://spdx.org/licenses/AGPL-3.0-only.html)

# Rapids.rs
An alpha server implementation of the [River](https://github.com/replit/river) protocol in rust.

## Feature Support
| Feature | Support | Comments |
| --- | --- | --- |
| River Server | ✔️ | |
| River Client | ❌ | |
| Pluggable Codecs | ✔️ | JSON and MessagePack codecs are provided as well as support for custom codecs |
| Pluggable Transports | ❌ | WebSocket support is hardcoded in with no other transport options yet |
| `rpc` procedures | ✔️ | |
| `upload` procedures | ✔️ | |
| `subscription` procedures | ❔ | Mostly supported, however server-side close semantics are not fully correct |
| `stream` procedures | ❔ | Mostly supported, however server-side close semantics are not fully correct |
| Transparent Reconnection | ❌ | See [#1] |
| Strong Typing for procedures | ❌ | Currently only message headers and control messages are strongly typed, procedures get [dynamic values](https://docs.rs/serde_json/latest/serde_json/value/index.html) |
| Heartbeats | ❔ | Server sends heartbeats but does not deal with unresponsive clients yet |
| Error Recovery | ❔ | Unwrap is still widely used internally, better error handling using thiserror (instead of anyhow) is needed |
| Handshake Metadata Validation | ❌ | |


[#1]: https://github.com/potentialstyx/rapids/issues/1