import { FC } from 'react';
import { Outlet } from 'react-router-dom';
import skybox from '@client/beyond/assets/images/skybox.jpg';

const Portal: FC = () => {
  return (
    <div
      className="h-screen w-screen max-h-screen max-w-screen flex justify-center items-center"
      style={{ backgroundImage: `url(${skybox})` }}
    >
      <div className="w-96">
        <Outlet />
      </div>
    </div>
  );
};

export default Portal;
