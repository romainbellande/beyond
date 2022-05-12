import type { FC } from 'react';
import { Planet } from '@client/beyond/interfaces';
import { Button } from '../Button';

export interface PlanetDetailsProps {
  planet: Planet;
}

export const PlanetDetails: FC<PlanetDetailsProps> = (props) => {
  const { planet } = props;

  return (
    <div className="bg-slate-700 shadow-md w-max">
      <h3 className="capitalize-first bg-slate-800 font-semibold px-4 py-2 text-lg">
        {planet.name}
      </h3>
      <div className="px-4 py-2">
        <div className="flex justify-between space-x-4">
          <span className="font-semibold">type:</span>
          <span>{planet.type}</span>
        </div>
        <div className="flex justify-between space-x-4">
          <span className="font-semibold">coordinates:</span>
          <span>
            [
            <span className="cursor-pointer text-yellow-500 hover:text-yellow-400">
              {planet.coordinates.x};{planet.coordinates.y};
              {planet.coordinates.z}
            </span>
            ]
          </span>
        </div>
        <hr className="my-2" />
        <div>
          {planet.resources.map((resource) => (
            <div
              key={resource.reference}
              className="flex justify-between space-x-4 items-center"
            >
              <span>{resource.reference}</span>
              <div className="w-11 bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
                <div
                  className="bg-blue-600 h-2.5 rounded-full"
                  style={{ width: `${resource.density}%` }}
                ></div>
              </div>
            </div>
          ))}
        </div>
        <Button className="mt-4 mx-auto" size="sm">
          create base
        </Button>
      </div>
    </div>
  );
};
