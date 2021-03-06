// https://redux-toolkit.js.org/api/createListenerMiddleware
import { createListenerMiddleware } from '@reduxjs/toolkit';
import { findPlanets } from '../planets';
import { WsMessage } from '../../interfaces/ws-message.interface';

export const wsMiddleware = createListenerMiddleware();

wsMiddleware.startListening({
  actionCreator: findPlanets,
  effect: async (action, store) => {
    let message: WsMessage | undefined;
    const ws = window.ws;

    if (ws && ws.readyState === ws.OPEN) {
      if (action.type === findPlanets.type) {
        message = new WsMessage('GetPlanets');
      }

      if (message) {
        ws.send(message.encode());
      }
    }
  },
});
