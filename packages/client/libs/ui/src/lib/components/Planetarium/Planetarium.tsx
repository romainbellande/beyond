import { useRef, useState, FC, useCallback, useMemo, useEffect } from 'react';
import ForceGraph3D, { GraphData, NodeObject } from 'react-force-graph-3d';
import { Texture } from 'three';

import {
  getPlanetMaterial,
  loadPlanetTextures,
  loadTexture,
  createSkybox,
} from '@client/beyond/utils';
import { Planet } from '@client/beyond/interfaces';

export interface PlanetariumProps {
  planets: Planet[];
  onPlanetSelect(planet: Planet): void;
}

export const Planetarium: FC<PlanetariumProps> = (props) => {
  const { planets, onPlanetSelect } = props;
  const [textures, setTextures] = useState<Record<string, Texture>>({});
  const [skyboxTexture, setSkyboxTexture] = useState<Texture>();
  const [skyboxCreated, setSkyboxCreated] = useState(false);
  const [height, setHeight] = useState(0);
  const [width, setWidth] = useState(0);
  const fgRef = useRef<any>();

  const parentDiv = useCallback((node) => {
    if (node !== null) {
      setHeight(node.getBoundingClientRect().height);
      setWidth(node.getBoundingClientRect().width);
    }
  }, []);

  useMemo(() => {
    async function loadMaterials() {
      try {
        const texturesTmp = await loadPlanetTextures();
        const skyboxTextureTmp = await loadTexture('/assets/images/skybox.jpg');
        setSkyboxTexture(skyboxTextureTmp);
        setTextures(texturesTmp);
      } catch (error) {
        console.error(error);
      }
    }
    if (!textures || !skyboxTexture) {
      return loadMaterials();
    }
    return false;
  }, [textures, skyboxTexture]);

  const data: GraphData = useMemo(
    () => ({
      nodes: planets.map(({ id, coordinates, type }) => ({
        id,
        ...coordinates,
        type,
      })),
      links: planets.map(({ id }, idx) => {
        const idxTmp = Math.abs(Math.round(Math.random() * (idx - 1)));
        return {
          source: id,
          target: planets[idxTmp].id,
          value: Math.round(Math.random() * 5),
        };
      }),
    }),
    [planets]
  );

  const isReady = useMemo<boolean>(
    () => Boolean(Object.keys(textures).length > 0 && width > 0 && height > 0),
    [textures, width, height]
  );

  useEffect(() => {
    if (fgRef.current && isReady && skyboxTexture && !skyboxCreated) {
      const skybox = createSkybox(skyboxTexture);
      fgRef.current.scene().add(skybox);
      console.log('skybox added to scene');
      setSkyboxCreated(true);
    }
  }, [skyboxCreated, skyboxTexture, fgRef, isReady]);

  const handleClick = useCallback(
    (node: NodeObject) => {
      const planet = planets.find((item) => node.id === item.id);
      if (planet) {
        onPlanetSelect(planet);
      }
      const { x = 0, y = 0, z = 0 } = node;
      // Aim at node from outside it1
      const distance = 40;
      const distRatio = 1 + distance / Math.hypot(x, y, z);

      fgRef.current.cameraPosition(
        { x: x * distRatio, y: y * distRatio, z: z * distRatio }, // new position
        node, // lookAt ({ x, y, z })
        3000 // ms transition duration
      );
    },
    [planets, fgRef, onPlanetSelect]
  );

  const findPlanetTypeById = (id: string): string | undefined =>
    planets.find((planet) => planet.id === id)?.type;

  return (
    <div ref={parentDiv} className="w-full h-full">
      {isReady && (
        <div id="planetarium" className="overflow-hidden">
          <ForceGraph3D
            numDimensions={2}
            ref={fgRef}
            graphData={data}
            enableNodeDrag={false}
            showNavInfo={false}
            onNodeClick={handleClick}
            // onEngineStop={() => fgRef.current.zoomToFit(400)}
            nodeThreeObject={({ id }) => {
              const type = findPlanetTypeById(id as string);
              const texture = type ? textures[type] : textures[0];
              return getPlanetMaterial(texture);
            }}
          />
        </div>
      )}
    </div>
  );
};
