import { WsEvent } from '../enums';
import { actions } from '../store';

export class WsMessage<T = void> {
  constructor(private event: string, private data?: T) {}

  public encode(): Uint8Array {
    return new TextEncoder().encode(JSON.stringify(this.get()));
  }

  public getAction(): any {
    return actions[this.event as WsEvent](this.data as any);
  }

  private get(): Record<string, T | null> {
    return {
      [this.event]: this.data || null,
    };
  }

  public static decode(buffer: any): WsMessage<unknown> {
    const stringValue = new TextDecoder().decode(buffer as ArrayBuffer);
    const response = JSON.parse(stringValue);
    const event = Object.keys(response)[0];
    const data = response[event];
    return new WsMessage<typeof data>(event, data);
  }
}
