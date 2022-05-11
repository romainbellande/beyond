import { TextureLoader, Texture } from 'three';


export const loadTexture = (url: string): Promise<Texture> => {
  const loader = new TextureLoader();
  return new Promise(resolve => loader.load(url, resolve));
}
