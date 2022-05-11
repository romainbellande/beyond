import { Mesh, SphereBufferGeometry, MeshBasicMaterial, Texture } from 'three';

export const getPlanetMaterial = (map: Texture) => new Mesh(new SphereBufferGeometry(1, 32, 32), new MeshBasicMaterial({ map }));
