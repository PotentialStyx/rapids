import { createServiceSchema, Procedure, Ok } from '@replit/river';
import { BinaryCodec } from '@replit/river/codec';
import { Type } from '@sinclair/typebox';

export const CODEC = BinaryCodec;

export const ServiceSchema = createServiceSchema();

export const AdderService = ServiceSchema.define(
    {
        initializeState: () => ({ count: 0 }),
    },
    {
        add: Procedure.rpc({
            requestInit: Type.Object({ n: Type.Number() }),
            responseData: Type.Object({ result: Type.Number() }),
            async handler({ ctx, reqInit: { n } }) {
                ctx.state.count += n;
                if (n === 6) {
                    throw new Error("test");
                }
                return Ok({ result: ctx.state.count });
            },
        }),

        resetCount: Procedure.rpc({
            requestInit: Type.Number(),
            responseData: Type.Null(),
            async handler({ ctx, reqInit }) {
                ctx.state.count = reqInit;
                return Ok(null);
            },
        }),

        uploadAdd: Procedure.upload({
            requestInit: Type.Null(),
            requestData: Type.Object({ n: Type.Number() }),
            responseData: Type.Object({ result: Type.Number() }),
            async handler({ ctx, reqReadable }) {
                for await (const item of reqReadable) {
                    if (item.ok) {
                        ctx.state.count += item.payload.n;
                    }
                }

                return Ok({ result: ctx.state.count })
            }
        }),

        streamAdd: Procedure.stream({
            requestInit: Type.Null(),
            requestData: Type.Object({ n: Type.Number() }),
            responseData: Type.Object({ result: Type.Number() }),
            async handler({ ctx, reqReadable, resWritable }) {
                for await (const item of reqReadable) {
                    if (item.ok) {
                        ctx.state.count += item.payload.n;
                        resWritable.write(Ok({ result: ctx.state.count }));
                    }
                }

                resWritable.close;
            }
        }),

        subscriptionAdd: Procedure.subscription({
            requestInit: Type.Array(Type.Number()),
            responseData: Type.Object({ result: Type.Number() }),
            async handler({ ctx, resWritable, reqInit }) {
                for (const item of reqInit) {
                    ctx.state.count += item;
                    resWritable.write(Ok({ result: ctx.state.count }));
                }

                resWritable.close();

            }
        })
    },
);

export const services = {
    adder: AdderService
};