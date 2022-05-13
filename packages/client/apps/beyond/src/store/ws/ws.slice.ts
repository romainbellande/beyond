import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { w3cwebsocket as W3CWebSocket } from 'websocket';
import { RootState } from '..';

interface WsState {
  connected: boolean;
  ws?: W3CWebSocket;
}

const initialState: WsState = { connected: false };

const wsSlice = createSlice({
  name: 'websocket',
  initialState,
  reducers: {
    setConnect(state, action: PayloadAction<boolean>) {
      state.connected = action.payload;
    },
    setWs(state, action: PayloadAction<W3CWebSocket>) {
      state.ws = action.payload;
    },
  },
});

export const { setConnect, setWs } = wsSlice.actions;

export const selectWs = (state: RootState) => state.wsReducer.ws;

export const selectConnected = (state: RootState) => state.wsReducer.connected;

export default wsSlice.reducer;
