import http from 'http';
import { WebSocketServer } from 'ws';
import { WebSocketServerTransport } from '@replit/river/transport/ws/server';
import { createServer } from '@replit/river';
import { services, CODEC } from './schema';

// start websocket server on port 3000
const httpServer = http.createServer();
const port = 8080;
const wss = new WebSocketServer({ server: httpServer });
const transport = new WebSocketServerTransport(wss, 'SERVER', { codec: CODEC });


export const server = createServer(transport, services);

export type ServiceSurface = typeof server;

httpServer.listen(port);