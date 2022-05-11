import { Story, Meta } from '@storybook/react';
import { Planetarium, PlanetariumProps } from './Planetarium';

export default {
  component: Planetarium,
  title: 'Planetarium',
} as Meta;

const Template: Story<PlanetariumProps> = (args) => <Planetarium {...args} />;

export const Default = Template.bind({});
Default.args = {};
