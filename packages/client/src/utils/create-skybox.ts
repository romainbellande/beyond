import { Texture, BoxGeometry, Mesh, BackSide, MeshBasicMaterial } from 'three';
import { times } from 'lodash';


export const createSkybox = (texture: Texture):  Mesh<BoxGeometry, MeshBasicMaterial[]> => {
  const textures = times(6).map(() => texture);
  const materials = textures.map(map => new MeshBasicMaterial({ map, side: BackSide }));
  const skyboxGeo = new BoxGeometry(10000, 10000, 10000);
  const skybox = new Mesh(skyboxGeo, materials);
  return skybox;
};
