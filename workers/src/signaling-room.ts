interface Device {
  id: string;
  name: string;
  joinedAt: number;
  webSocket: WebSocket;
}

interface SignalMessage {
  type: 'offer' | 'answer' | 'ice-candidate' | 'sync-request' | 'sync-response' | 'presence' | 'secret-updated';
  from: string;
  to?: string;
  payload: unknown;
}

interface Env {}

export class SignalingRoom {
  private devices: Map<string, Device> = new Map();
  private state: DurableObjectState;

  constructor(state: DurableObjectState, _env: Env) {
    this.state = state;
  }

  async fetch(request: Request): Promise<Response> {
    const url = new URL(request.url);

    // HTTP: List online devices
    if (url.pathname === '/list-devices') {
      const devices = this.getDeviceList();
      return new Response(JSON.stringify({ devices }), {
        headers: { 'Content-Type': 'application/json' },
      });
    }

    // WebSocket upgrade
    const deviceId = request.headers.get('X-Device-Id') || 'unknown';
    const deviceName = request.headers.get('X-Device-Name') || 'unknown';

    const pair = new WebSocketPair();
    const [client, server] = Object.values(pair);

    this.handleSession(server, deviceId, deviceName);

    return new Response(null, {
      status: 101,
      webSocket: client,
    });
  }

  private handleSession(webSocket: WebSocket, deviceId: string, deviceName: string): void {
    webSocket.accept();

    // Register device
    const device: Device = {
      id: deviceId,
      name: deviceName,
      joinedAt: Date.now(),
      webSocket,
    };
    this.devices.set(deviceId, device);

    // Notify others about new device
    this.broadcastPresence();

    // Send current device list to the new device
    this.sendToDevice(deviceId, {
      type: 'presence',
      from: 'server',
      payload: {
        action: 'device-list',
        devices: this.getDeviceList(),
      },
    });

    // Handle incoming messages
    webSocket.addEventListener('message', (event) => {
      try {
        const message: SignalMessage = JSON.parse(event.data as string);
        this.handleMessage(deviceId, message);
      } catch (e) {
        this.sendToDevice(deviceId, {
          type: 'presence',
          from: 'server',
          payload: { error: 'Invalid message format' },
        });
      }
    });

    // Handle disconnection
    webSocket.addEventListener('close', () => {
      this.devices.delete(deviceId);
      this.broadcastPresence();
    });

    webSocket.addEventListener('error', () => {
      this.devices.delete(deviceId);
      this.broadcastPresence();
    });
  }

  private handleMessage(fromId: string, message: SignalMessage): void {
    switch (message.type) {
      // WebRTC signaling - route to specific peer
      case 'offer':
      case 'answer':
      case 'ice-candidate':
        if (message.to) {
          this.sendToDevice(message.to, {
            ...message,
            from: fromId,
          });
        }
        break;

      // Secret sync - broadcast to all other devices
      case 'secret-updated':
        this.broadcastExcept(fromId, {
          type: 'secret-updated',
          from: fromId,
          payload: message.payload,
        });
        break;

      // Sync request - ask all peers for their state
      case 'sync-request':
        this.broadcastExcept(fromId, {
          type: 'sync-request',
          from: fromId,
          payload: { requestedBy: fromId },
        });
        break;

      // Sync response - send state to specific device
      case 'sync-response':
        if (message.to) {
          this.sendToDevice(message.to, {
            ...message,
            from: fromId,
          });
        }
        break;

      default:
        break;
    }
  }

  private sendToDevice(deviceId: string, message: unknown): void {
    const device = this.devices.get(deviceId);
    if (device && device.webSocket.readyState === WebSocket.READY_STATE_OPEN) {
      device.webSocket.send(JSON.stringify(message));
    }
  }

  private broadcastExcept(excludeId: string, message: unknown): void {
    const payload = JSON.stringify(message);
    for (const [id, device] of this.devices) {
      if (id !== excludeId && device.webSocket.readyState === WebSocket.READY_STATE_OPEN) {
        device.webSocket.send(payload);
      }
    }
  }

  private broadcastPresence(): void {
    const devices = this.getDeviceList();
    const message = JSON.stringify({
      type: 'presence',
      from: 'server',
      payload: {
        action: 'device-list',
        devices,
      },
    });

    for (const device of this.devices.values()) {
      if (device.webSocket.readyState === WebSocket.READY_STATE_OPEN) {
        device.webSocket.send(message);
      }
    }
  }

  private getDeviceList(): { id: string; name: string; joinedAt: number }[] {
    return Array.from(this.devices.values()).map((d) => ({
      id: d.id,
      name: d.name,
      joinedAt: d.joinedAt,
    }));
  }
}
