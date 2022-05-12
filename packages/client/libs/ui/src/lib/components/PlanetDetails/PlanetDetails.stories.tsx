import { Story, Meta } from '@storybook/react';
import { PlanetType, ResourceRef } from '@client/beyond/interfaces';
import { PlanetDetails, PlanetDetailsProps } from './PlanetDetails';

export default {
  component: PlanetDetails,
  title: 'PlanetDetails',
} as Meta;

const Template: Story<PlanetDetailsProps> = (args) => (
  <PlanetDetails {...args} />
);

export const Primary = Template.bind({});
Primary.args = {
  planet: {
    id: '123123123',
    name: 'Terra',
    type: PlanetType.Desert,
    coordinates: {
      x: 123,
      y: -312,
      z: 21,
    },
    resources: [
      {
        reference: ResourceRef.Iron,
        density: 80,
      },
      {
        reference: ResourceRef.Oil,
        density: 24,
      },
    ],
  },
};
