import { createServiceSchema, Procedure, Ok } from '@replit/river';
import { Type } from '@sinclair/typebox';

export const ServiceSchema = createServiceSchema();

export const ExampleService = ServiceSchema.define(
    {
        initializeState: () => ({ count: 0 }),
    },
    {
        add: Procedure.rpc({
            requestInit: Type.Object({ n: Type.Number() }),
            responseData: Type.Object({ result: Type.Number() }),
            async handler({ ctx, reqInit: { n } }) {
                ctx.state.count += n;
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

        streamAdd: Procedure.upload({
            requestInit: Type.Null(),
            requestData: Type.Object({ n: Type.Number() }),
            responseData: Type.Object({ result: Type.Number() }),
            async handler({ ctx, reqReadable }) {
                for await (let item of reqReadable) {
                    if (item.ok) {
                        ctx.state.count += item.payload.n;
                    }
                }

                return Ok({ result: ctx.state.count })
            }
        })
    },
);

export const services = {
    example: ExampleService
};