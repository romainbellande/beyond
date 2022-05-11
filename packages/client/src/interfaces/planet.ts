import { Coordinates } from "./coordinates";
// https://www.solarsystemscope.com/textures/

export enum PlanetType {
  Ocean = 'ocean',
  Ice = 'ice',
  Carbon = 'carbon',
  Desert = 'desert',
  Laval = 'lava',
  Iron = 'iron',
  Gas = 'hydrogen',
  Silicate = 'silicate',
  Telluric = 'telluric',
  Helium = 'helium',
  Exoplanet = 'exoplanet',
}

export interface Planet {
  id: string;
  name: string;
  resources: [];
  coordinates: Coordinates;
  type: PlanetType;
}
