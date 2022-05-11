import { Story, Meta } from '@storybook/react';
import { ErrorBoundary } from './ErrorBoundary';

export default {
  component: ErrorBoundary,
  title: 'ErrorBoundary',
} as Meta;

const thrower = () => {
  throw new Error('an error');
};

const Cmp = () => <div>Hello {thrower()}</div>;

const Template: Story = (args) => (
  <ErrorBoundary {...args}>
    <Cmp />
  </ErrorBoundary>
);
export const Primary = Template.bind({});
Primary.args = {};
