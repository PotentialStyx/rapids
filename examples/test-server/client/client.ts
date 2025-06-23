import { WebSocketClientTransport } from '@replit/river/transport/ws/client';
import { createClient, type Client } from '@replit/river';
import { WebSocket } from 'ws';
import { services } from './schema';
import { BinaryCodec } from '@replit/river/codec';
import { customAlphabet } from 'nanoid';

const alphabet = customAlphabet(
    '1234567890abcdefghijklmnopqrstuvxyzABCDEFGHIJKLMNOPQRSTUVXYZ',
);
export const generateId = (length?: number) => alphabet(length || 12);

const DEBUG = false;
const SHOW_PINGS = false;

const ws = new WebSocket('ws://localhost:8080/delta');
// @ts-ignore
ws._send = ws.send

const decoder = new TextDecoder();
const CODEC = BinaryCodec;

ws.send = (...args) => {
    // @ts-ignore
    ws._send(...args);

    try {
        if (DEBUG) {
            // @ts-ignore
            const msg = CODEC.fromBuffer(args[0])

            if (msg.streamId == "heartbeat" && !SHOW_PINGS) return;

            console.log(
                "SEND", msg
            )
        }
    } catch (e) {
        console.error(e);
    }
}

ws.addEventListener("message", (_msg) => {
    if (DEBUG) {
        // console.log(_msg.data, decoder.decode(_msg.data));
        // @ts-ignore
        const msg = CODEC.fromBuffer(_msg.data);

        if (msg.streamId == "heartbeat" && !SHOW_PINGS) return;

        console.log("RECV", msg)
    };
})

const transport = new WebSocketClientTransport(
    async () => ws,
    `client-${generateId(5)}`,
    { codec: CODEC }
);



console.info("River client built");

const client: Client<typeof services> = createClient(
    transport,
    'SERVER', // transport id of the server in the previous step
    { eagerlyConnect: true }, // whether to eagerly connect to the server on creation (optional argument)
);


console.info("River client connected");

{
    console.info("Sending example::add {n: 3}");

    const result = await client.example.add.rpc({ n: 3 });

    if (result.ok) {
        const msg = result.payload;
        console.info("Recieved: " + msg.result);
    }
}

{
    console.info("Sending example::add {n: 6}");

    const result = await client.example.add.rpc({ n: 6 });

    if (result.ok) {
        const msg = result.payload;
        console.info("Recieved: " + msg.result);
    }
}

{
    console.info("Sending example::resetCount(0)");
    const result = await client.example.resetCount.rpc(0);

    if (result.ok) {
        const msg = result.payload;
        console.info("Count reset");
    }
}

{
    const numbers = [1, 2, 3];
    console.info("Sending example::streamAdd [{n: 1}, {n: 2}, {n: 3}]");
    const res = client.example.streamAdd.upload(null);

    for (const n of numbers) {
        res.reqWritable.write({ n });
    }

    res.reqWritable.close()

    const result = await res.finalize();

    if (result.ok) {
        const msg = result.payload;
        console.info("Recieved: " + msg.result);
    }

}
