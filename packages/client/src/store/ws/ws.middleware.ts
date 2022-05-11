// https://redux-toolkit.js.org/api/createListenerMiddleware
import { createListenerMiddleware } from "@reduxjs/toolkit";
import { w3cwebsocket as W3CWebSocket } from 'websocket';
import { findPlanets } from '../planets';
import { WsMessage } from '../../interfaces/ws-message.interface';

export const wsMiddleware = createListenerMiddleware();

export const ws = new W3CWebSocket('ws://localhost:3000/ws/');

ws.binaryType = 'arraybuffer';

wsMiddleware.startListening({
  actionCreator: findPlanets,
  effect: async (action, store) => {
    let message: WsMessage | undefined;

    if (ws.readyState === ws.OPEN) {

      if (action.type === findPlanets.type) {
        message = new WsMessage('GetPlanets');
      }

      if (message) {
        ws.send(message.encode());
      }
    }
  }
});
