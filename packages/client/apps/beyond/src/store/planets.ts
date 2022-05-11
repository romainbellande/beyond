import { createSlice } from '@reduxjs/toolkit';
import { createAction } from '@reduxjs/toolkit';
import { RootState } from '.';
import { Planet } from '../interfaces';
import { WsEvent } from '../enums';

export const findPlanets = createAction('planets/findPlanets');
export const getPlanetsResponse = createAction<Planet[]>('GetPlanetsResponse');

interface PlanetsState {
  planets: Planet[];
}

const initialState: PlanetsState = { planets: [] };

const planetsSlice = createSlice({
  name: 'planets',
  initialState,
  reducers: {
    setPlanets(state, action) {
      state.planets = action.payload;
    },
  },
});

export const selectPlanets = (state: RootState) => state.planetsReducer.planets;
export const { setPlanets } = planetsSlice.actions;

export const actions = {
  [WsEvent.GetPlanetsResponse]: setPlanets,
};

export default planetsSlice.reducer;
