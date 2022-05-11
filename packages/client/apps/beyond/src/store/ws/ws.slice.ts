import { createSlice, PayloadAction } from '@reduxjs/toolkit';

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

export default wsSlice.reducer;
