import { w3cwebsocket as W3CWebSocket } from 'websocket';

declare global {
  interface Window {
    ws?: W3CWebSocket;
  }
}
