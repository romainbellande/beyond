import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { RootState } from '.';

interface PlayerState {
  jwt?: string;
}

const initialState: PlayerState = {
  jwt: localStorage.getItem('jwt') || undefined,
};

const playerSlice = createSlice({
  name: 'player',
  initialState,
  reducers: {
    setJwt(state, action: PayloadAction<string | undefined>) {
      const jwt = action.payload;
      if (jwt) {
        localStorage.setItem('jwt', jwt);
      } else {
        localStorage.clear();
      }
      state.jwt = jwt;
    },
  },
});

export const { setJwt } = playerSlice.actions;

export const selectJwt = (state: RootState) => state.playerReducer.jwt;

export default playerSlice.reducer;
