import React from 'react';
import logo from './logo.svg';
import './App.css';
// import { connect, send } from '@giantmachines/redux-websocket';
import { useAppDispatch, useAppSelector } from './store';
import { WsMessage } from './interfaces/ws-message.interface';
import { ws, setConnect } from './store/ws';
import { selectPlanets, findPlanets } from './store/planets';

function App() {
  const planets = useAppSelector(selectPlanets);
  const dispatch = useAppDispatch();

  ws.onopen = () => {
    setConnect(true);
    console.info('connected to websocket');
    dispatch(findPlanets());
  };

  ws.onmessage = (e) => {
    console.log('message received', e.data);
    console.log(e.data);
  };

  console.log('planets', planets);

  // dispatch(connect('ws://127.0.0.1:3000/ws/'));
  const wsMessage: WsMessage = {
    event: 'GET_PLANETS'
  };

  // dispatch(send(wsEvent));

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
