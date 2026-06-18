export { SignalingRoom } from './signaling-room';

interface Env {
  SIGNALING_ROOM: DurableObjectNamespace;
}

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    const url = new URL(request.url);

    // Health check
    if (url.pathname === '/health') {
      return new Response(JSON.stringify({ status: 'ok', service: 'envsec-signaling' }), {
        headers: { 'Content-Type': 'application/json' },
      });
    }

    // WebSocket upgrade for signaling
    if (request.headers.get('Upgrade') === 'websocket') {
      const room = url.searchParams.get('room') || 'default';
      const deviceId = url.searchParams.get('device') || crypto.randomUUID();
      const deviceName = url.searchParams.get('name') || `device-${deviceId.slice(0, 8)}`;

      // Get the Durable Object for this room
      const roomId = env.SIGNALING_ROOM.idFromName(room);
      const stub = env.SIGNALING_ROOM.get(roomId);

      // Forward the request with device info in headers
      const headers = new Headers(request.headers);
      headers.set('X-Device-Id', deviceId);
      headers.set('X-Device-Name', deviceName);

      const newRequest = new Request(request.url, {
        method: request.method,
        headers,
        body: request.body,
      });

      return stub.fetch(newRequest);
    }

    // List online devices for a room (HTTP)
    if (url.pathname === '/api/rooms' && request.method === 'GET') {
      const room = url.searchParams.get('room') || 'default';
      const roomId = env.SIGNALING_ROOM.idFromName(room);
      const stub = env.SIGNALING_ROOM.get(roomId);
      return stub.fetch(new Request('https://internal/list-devices', { method: 'GET' }));
    }

    return new Response('Not found', { status: 404 });
  },
};
