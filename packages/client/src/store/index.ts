import reduxWebsocket from '@giantmachines/redux-websocket';
import { configureStore } from '@reduxjs/toolkit'
import planetsReducer from './planets';
import { TypedUseSelectorHook, useDispatch, useSelector } from 'react-redux'
import wsReducer from './ws/ws.slice';
import { wsMiddleware } from './ws';

// const reduxWebsocketMiddleware = reduxWebsocket();

export const store = configureStore({
  reducer: {
    // [api.reducerPath]: api.reducer,
    wsReducer,
    planetsReducer
  },
  middleware: (getDefaultMiddleware) => getDefaultMiddleware().concat(wsMiddleware.middleware),
});

// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>;
// Inferred type: {posts: PostsState, comments: CommentsState, users: UsersState}
export type AppDispatch = typeof store.dispatch;

// Use throughout your app instead of plain `useDispatch` and `useSelector`
export const useAppDispatch = () => useDispatch<AppDispatch>()
export const useAppSelector: TypedUseSelectorHook<RootState> = useSelector
