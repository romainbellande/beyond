import { FC } from 'react';
import { Planetarium } from '@client/ui';
import { useAppSelector } from '@client/beyond/store';
import { selectPlanets } from '@client/beyond/store/planets';

const Galaxy: FC = () => {
  const planets = useAppSelector(selectPlanets);
  return planets.length > 0 ? <Planetarium planets={planets} /> : null;
};

export default Galaxy;
