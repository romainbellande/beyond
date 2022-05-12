import { FC, useState } from 'react';
import { Planetarium, PlanetDetails } from '@client/ui';
import { useAppSelector } from '@client/beyond/store';
import { selectPlanets } from '@client/beyond/store/planets';
import { Planet } from '@client/beyond/interfaces';

const Galaxy: FC = () => {
  const planets = useAppSelector(selectPlanets);
  const [planet, setPlanet] = useState<Planet>();

  return (
    <div className="w-full h-full">
      {planets.length > 0 ? (
        <Planetarium planets={planets} onPlanetSelect={setPlanet} />
      ) : null}
      {planet && (
        <div className="absolute top-4 right-4">
          <PlanetDetails planet={planet} />
        </div>
      )}
    </div>
  );
};

export default Galaxy;
