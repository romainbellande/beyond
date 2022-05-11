import { render } from '@testing-library/react';
import { ThemedSuspense } from './ThemedSuspense';
import '@testing-library/jest-dom';

test('it can render', async () => {
  const { baseElement } = render(<ThemedSuspense />);

  expect(baseElement).toMatchSnapshot();
});
