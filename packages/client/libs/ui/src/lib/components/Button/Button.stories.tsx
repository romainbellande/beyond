import { action } from '@storybook/addon-actions';
import { Story, Meta } from '@storybook/react';
import { Button, ButtonProps } from './Button';
import { HeartIcon } from '@heroicons/react/solid';

export default {
  component: Button,
  title: 'Button',
} as Meta;

const Template: Story<ButtonProps> = (args) => (
  <Button {...args}>my button</Button>
);

export const Default = Template.bind({});
Default.args = {
  onClick: action('click'),
};

export const WithIcon = Template.bind({});
WithIcon.args = {
  onClick: action('click'),
  icon: <HeartIcon />,
  iconDirection: 'left',
};
