import test from 'ava'

import { Library, Language } from '../index.js'

test('Load dependency', {
  skip: process.platform === 'win32', // This hangs intermittently on CI
}, async (t) => {
  const library = await Library.loadDependency('tokio', '.', Language.Rust);

  t.is(library.name, 'tokio');
});

test('Load library', async (t) => {
  const library = await Library.load('./', Language.Rust);

  t.is(library.name, 'daipendency_core');
});
