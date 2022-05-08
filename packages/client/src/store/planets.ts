import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { createAction } from '@reduxjs/toolkit'
import { RootState } from '.';
import { Planet } from '../interfaces';

export const findPlanets = createAction('planets/findPlanets');

interface PlanetsState {
  planets: Planet[],
}

const initialState: PlanetsState = { planets: [] };

const planetsSlice= createSlice({
  name: 'planets',
  initialState,
  reducers: {
    setPlanets(state, action: PayloadAction<Planet[]>) {
      state.planets = action.payload;
    }
  }
})

export const { setPlanets } = planetsSlice.actions;

export const selectPlanets = (state: RootState) => state.planetsReducer.planets;

export default planetsSlice.reducer;
