import { render } from '@testing-library/react';
import { Button, ButtonProps } from './Button';
import '@testing-library/jest-dom';

test('it can render', async () => {
  const props: ButtonProps = {};

  const { baseElement } = render(<Button {...props} />);

  expect(baseElement).toMatchSnapshot();
});
