import { FC, useEffect } from 'react';
import { Outlet, useNavigate } from 'react-router-dom';
import { useAppDispatch, useAppSelector } from '@client/beyond/store';
import { WsMessage } from '@client/beyond/interfaces/ws-message.interface';
import { w3cwebsocket as W3CWebSocket } from 'websocket';
import { findPlanets } from '@client/beyond/store/planets';
import { config } from '@client/beyond/config';
import { Claims } from '@client/beyond/interfaces/claims';
import { isPast } from 'date-fns';
import { parseJwt } from '@client/beyond/utils/parse-jwt';
import { selectJwt, setJwt } from '@client/beyond/store/player';

const Dashboard: FC = () => {
  const dispatch = useAppDispatch();
  const jwt = useAppSelector(selectJwt);
  const navigate = useNavigate();

  useEffect(() => {
    if (jwt) {
      const claims = parseJwt<Claims>(jwt);
      if (!claims || (claims && isPast(new Date(claims?.exp)))) {
        console.info('jwt expired or invalid');
        dispatch(setJwt());
        navigate('/portal/login');
      }
    } else if (!jwt) {
      console.info('user not logged');
      navigate('/portal/login');
    }
  }, []);

  useEffect(() => {
    if (jwt) {
      const ws = new W3CWebSocket(`${config.wsUrl}?jwt=${jwt}`);

      ws.binaryType = 'arraybuffer';

      ws.onopen = () => {
        console.info('connected to websocket');
        dispatch(findPlanets());
      };

      ws.onmessage = (e) => {
        const { data } = e;
        const message = WsMessage.decode(data);
        dispatch(message.getAction());
      };

      window.ws = ws;
    }
  }, []);

  return <Outlet />;
};

export default Dashboard;
