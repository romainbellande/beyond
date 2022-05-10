// https://redux-toolkit.js.org/api/createListenerMiddleware
import { createListenerMiddleware } from "@reduxjs/toolkit";
import { w3cwebsocket as W3CWebSocket } from 'websocket';
import { AppDispatch, RootState } from "..";
import { findPlanets } from '../planets';
import { WsMessage } from '../../interfaces/ws-message.interface';
import init, { into_u8_array } from '../../wasm/wasm';

export const wsMiddleware = createListenerMiddleware();

export const ws = new W3CWebSocket('ws://localhost:3000/ws/');

ws.binaryType = 'arraybuffer';

wsMiddleware.startListening({
  actionCreator: findPlanets,
  effect: async (action, store) => {
    await init();
    console.log('action', action);

    if (action.type === findPlanets.type) {
      const message: WsMessage = {
        event: 'GetPlanets'
      };

      if (ws.readyState === ws.OPEN) {
        console.info('sending message', message);
        const bytearray: Uint8Array = new TextEncoder().encode(JSON.stringify(message));
        ws.send(bytearray);
      }
    }
  }
});
