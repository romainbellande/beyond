import { Story, Meta } from '@storybook/react';
import { ThemedSuspense } from './ThemedSuspense';

export default {
  component: ThemedSuspense,
  title: 'ThemedSuspense',
} as Meta;

const Template: Story = (args) => <ThemedSuspense {...args} />;

export const Primary = Template.bind({});
Primary.args = {};
