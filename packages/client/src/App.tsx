import React from 'react';
import logo from './logo.svg';
import './App.css';
// import { connect, send } from '@giantmachines/redux-websocket';
import { useAppDispatch, useAppSelector } from './store';
import { WsMessage } from './interfaces/ws-message.interface';
import { ws, setConnect } from './store/ws';
import { selectPlanets, findPlanets } from './store/planets';
import init, { greet } from './wasm/wasm';

function App() {
  const planets = useAppSelector(selectPlanets);

  ws.onopen = () => {
    setConnect(true);
    findPlanets();
  };

  console.log('planets', planets);
  init().then(() => {
    greet('toto');
  });

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
