import { PlanetType } from '../interfaces/planet';
import { loadTexture } from './load-texture';
import { Texture } from 'three';

export const loadPlanetTextures = async (): Promise<Record<string, Texture>> => {
  const promises = await Promise.all(Object.values(PlanetType).map(async type => ({
    [type]: await loadTexture(`/images/2k_${type}.jpg`),
  })))

  return promises.reduce((a, b) => ({ ...a, ...b }));
};
