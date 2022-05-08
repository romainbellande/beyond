// https://redux-toolkit.js.org/api/createListenerMiddleware
import { createListenerMiddleware } from "@reduxjs/toolkit";
import { w3cwebsocket as W3CWebSocket } from 'websocket';
import { AppDispatch, RootState } from "..";
import { findPlanets } from '../planets';
import { WsMessage } from '../../interfaces/ws-message.interface';

export const wsMiddleware = createListenerMiddleware<RootState, AppDispatch>();

export const ws = new W3CWebSocket('ws://localhost:3000/ws/');

wsMiddleware.startListening({
  actionCreator: findPlanets,
  effect: async (action, store) => {
    if (action.type === findPlanets.type) {
      const message: WsMessage = {
        event: 'GetPlanets'
      };

      ws.send(message);
    }
  }
});
