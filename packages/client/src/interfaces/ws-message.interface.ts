export interface WsMessage<T = void> {
  event: string;
  data?: T;
}
