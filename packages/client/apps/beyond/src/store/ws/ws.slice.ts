import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { RootState } from '..';

interface WsState {
  connected: boolean;
}

const initialState: WsState = { connected: false };

const wsSlice = createSlice({
  name: 'websocket',
  initialState,
  reducers: {
    setConnect(state, action: PayloadAction<boolean>) {
      state.connected = action.payload;
    },
  },
});

export const { setConnect } = wsSlice.actions;

export const selectConnected = (state: RootState) => state.wsReducer.connected;

export default wsSlice.reducer;
