import { render } from '@testing-library/react';
import { Planetarium, PlanetariumProps } from './Planetarium';
import '@testing-library/jest-dom';

test('it can render', async () => {
  const props: PlanetariumProps = {};

  const { baseElement } = render(<Planetarium {...props} />);

  expect(baseElement).toMatchSnapshot();
});
