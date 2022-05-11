const { createGlobPatternsForDependencies } = require('@nrwl/react/tailwind');
const { join } = require('path');

module.exports = (dir) => ({
  content: [
    join(dir, 'src/**/!(*.stories|*.spec).{ts,tsx,html}'),
    ...createGlobPatternsForDependencies(dir),
  ],
  theme: {},
  plugins: [],
});
