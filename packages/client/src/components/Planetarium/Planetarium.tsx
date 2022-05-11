import { useEffect, useRef, useState, FC, useCallback, useMemo } from 'react';
import ForceGraph3D, { GraphData, NodeObject } from 'react-force-graph-3d';
import { Texture } from 'three';
import { useAppSelector } from '@/store';
import { selectPlanets } from '@/store/planets';
import {
  getPlanetMaterial,
  loadPlanetTextures,
  loadTexture,
  createSkybox,
} from '@/utils';
import { Planet } from '@/interfaces';

export const Planetarium: FC = () => {
  const planets = useAppSelector(selectPlanets);
  const [textures, setTextures] = useState<Record<string, Texture>>();
  const [skyboxTexture, setSkyboxTexture] = useState<Texture>();
  const [skyboxCreated, setSkyboxCreated] = useState(false);
  const [selectedPlanet, setSelectedPlanet] = useState<Planet>();
  const fgRef = useRef<any>();

  useMemo(() => {
    async function loadMaterials() {
      try {
        const texturesTmp = await loadPlanetTextures();
        setSkyboxTexture(await loadTexture('/images/skybox.jpg'));
        console.log(`textures loaded: ${Object.keys(texturesTmp).length}`);
        setTextures(texturesTmp);
      } catch (error) {
        console.error(error);
      }
    }

    if (!textures || !skyboxTexture) {
      loadMaterials();
    }
  }, [textures, skyboxTexture]);

  const data: GraphData = {
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
  };

  useMemo(() => {
    console.log('fgRef.current', fgRef.current);
    if (fgRef.current && skyboxTexture && !skyboxCreated) {
      const skybox = createSkybox(skyboxTexture);
      fgRef.current.scene().add(skybox);
      setSkyboxCreated(true);
    }
  }, [skyboxCreated, skyboxTexture]);

  const handleClick = useCallback(
    (node: NodeObject) => {
      const planet = planets.find((item) => node.id === item.id);
      setSelectedPlanet(planet);
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
    [fgRef]
  );

  const findPlanetTypeById = (id: string): string | undefined =>
    planets.find((planet) => planet.id === id)?.type;

  return (
    <div>
      {textures && skyboxTexture && (
        <div id="planetarium">
          <ForceGraph3D
            ref={fgRef}
            graphData={data}
            onNodeClick={handleClick}
            // linkDirectionalParticles="value"
            // linkDirectionalParticleSpeed={(d) => (d as any).value * 0.001}
            // onRenderFramePre={onRenderFramePre}
            nodeThreeObject={({ id }) => {
              const type = findPlanetTypeById(id as string);
              const texture = type ? textures[type] : textures[0];
              return getPlanetMaterial(texture);
            }}
          />
          <div>Hello!</div>
        </div>
      )}
    </div>
  );
};
