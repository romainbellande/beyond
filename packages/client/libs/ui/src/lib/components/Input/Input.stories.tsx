import { Story, Meta } from '@storybook/react';
import { Input, InputProps } from './Input';

export default {
  component: Input,
  title: 'Input',
} as Meta;

const Template: Story<InputProps> = (args) => <Input {...args} />;

export const Default = Template.bind({});
Default.args = {};
