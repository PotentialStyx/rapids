![Crates.io Version](https://img.shields.io/crates/v/rapids-rs)

# Rapids.rs
An alpha server implementation of the [River](https://github.com/replit/river) protocol in rust.

## Feature Support
| Feature | Support | Comments |
| --- | --- | --- |
| River Server | ✔️ | |
| River Client | ❌ | |
| Pluggable Codecs | ❔ | While JSON and MessagePack codecs are provided, custom codecs are not yet supported |
| Pluggable Transports | ❌ | WebSocket support is hardcoded in with no other transport options yet |
| `rpc` procedures | ✔️ | |
| `upload` procedures | ✔️ | |
| `subscription` procedures | ❌ | |
| `stream` procedures | ❌ | |
| Transparent Reconnection | ❌ | |
| Strong Typing for procedures | ❌ | Currently only message headers and control messages are strongly typed, procedures get [dynamic values](https://docs.rs/serde_json/latest/serde_json/value/index.html) |
| Heartbeats | ❌ | |
| Error Recovery | ❔ | Unwrap is still widely used internally, better error handling using thiserror (instead of anyhow) is needed |

