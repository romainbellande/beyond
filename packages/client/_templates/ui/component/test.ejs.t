---
to: libs/ui/src/lib/components/<%=name%>/<%=name%>.test.tsx
---
import { render } from '@testing-library/react';
import { <%=name%>, <%=name%>Props } from './<%=name%>';
import '@testing-library/jest-dom';

test('it can render', async () => {
  const props: <%=name%>Props = {};

  const { baseElement } = render(<<%=name%> {...props} />);

  expect(baseElement).toMatchSnapshot();
});
