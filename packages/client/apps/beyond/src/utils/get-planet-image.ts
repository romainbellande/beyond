import { PlanetType } from '../interfaces';

export const getPlanetImage = (type: PlanetType): string =>
  `/assets/images/2k_${type}.jpg`;
