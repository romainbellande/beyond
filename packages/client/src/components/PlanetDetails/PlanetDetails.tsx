import { FC } from 'react';
import { Planet } from '@/interfaces';

interface Props {
  planet: Planet;
}

export const PlanetDetails: FC<Props> = (props: Props) => {
  const { planet } = props;
  return <div>{planet.name}</div>;
};
