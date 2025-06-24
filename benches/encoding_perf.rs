use criterion::{Criterion, criterion_group, criterion_main};
use rapids_rs::{
    codecs::{BinaryCodec, Codec, NaiveCodec},
    types::{
        Control, ExpectedSessionState, HandshakeRequest, HandshakeResponse, HandshakeResponseOk,
        Header, ProtocolVersion, RiverResult, TransportControlMessage,
    },
    utils::generate_id,
};
use std::hint::black_box;

static BIN: BinaryCodec = BinaryCodec {};
static NAI: NaiveCodec = NaiveCodec {};

fn criterion_benchmark(c: &mut Criterion) {
    let value = vec![
        TransportControlMessage {
            header: Header {
                id: generate_id(),
                from: generate_id(),
                to: generate_id(),
                seq: 1,
                ack: 2,
                stream_id: generate_id(),
                control_flags: 0,
            },
            payload: Control::HandshakeRequest(HandshakeRequest {
                protocol_version: ProtocolVersion::V2_0,
                session_id: generate_id(),
                expected_session_state: ExpectedSessionState {
                    next_expected_seq: 1,
                    next_sent_seq: 2,
                },
                metadata: Some(serde_json::json!(null)),
            }),
        },
        TransportControlMessage {
            header: Header {
                id: generate_id(),
                from: generate_id(),
                to: generate_id(),
                seq: 1,
                ack: 2,
                stream_id: generate_id(),
                control_flags: 0,
            },
            payload: Control::HandshakeRequest(HandshakeRequest {
                protocol_version: ProtocolVersion::V2_0,
                session_id: generate_id(),
                expected_session_state: ExpectedSessionState {
                    next_expected_seq: 1,
                    next_sent_seq: 2,
                },
                metadata: Some(serde_json::json!(null)),
            }),
        },
        TransportControlMessage {
            header: Header {
                id: generate_id(),
                from: generate_id(),
                to: generate_id(),
                seq: 1,
                ack: 2,
                stream_id: generate_id(),
                control_flags: 0,
            },
            payload: Control::HandshakeResponse(HandshakeResponse {
                status: RiverResult::<HandshakeResponseOk, String>::Ok(HandshakeResponseOk {
                    session_id: generate_id(),
                })
                .into(),
            }),
        },
        TransportControlMessage {
            header: Header {
                id: generate_id(),
                from: generate_id(),
                to: generate_id(),
                seq: 1,
                ack: 2,
                stream_id: generate_id(),
                control_flags: 0,
            },
            payload: Control::HandshakeResponse(HandshakeResponse {
                status: RiverResult::<HandshakeResponseOk, String>::Ok(HandshakeResponseOk {
                    session_id: generate_id(),
                })
                .into(),
            }),
        },
        TransportControlMessage {
            header: Header {
                id: generate_id(),
                from: generate_id(),
                to: generate_id(),
                seq: 1,
                ack: 2,
                stream_id: generate_id(),
                control_flags: 0,
            },
            payload: Control::HandshakeResponse(HandshakeResponse {
                status: RiverResult::<HandshakeResponseOk, String>::Err {
                    message: generate_id(),
                    code: generate_id(),
                }
                .into(),
            }),
        },
    ];

    c.bench_function("msgpack encode", |b| {
        b.iter(|| black_box(rmp_serde::to_vec(&value)))
    });

    c.bench_function("correct msgpack encode", |b| {
        b.iter(|| black_box(BIN.encode_to_vec(&value)))
    });

    c.bench_function("json encode", |b| {
        b.iter(|| black_box(NAI.encode_to_vec(&value)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
