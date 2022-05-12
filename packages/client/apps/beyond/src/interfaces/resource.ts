export enum ResourceRef {
  Iron = 'iron',
  Wood = 'wood',
  Sand = 'sand',
  Oil = 'oil',
}

export interface Resource {
  reference: ResourceRef;
  density: number;
}
