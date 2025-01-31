import test from 'ava'

import { Library, Language } from '../index.js'

test('Load dependency', async (t) => {
  const library = await Library.loadDependency('tokio', '.', Language.Rust);

  t.is(library.name, 'tokio');
});

test('Load library', async (t) => {
  const library = await Library.load('./', Language.Rust);

  t.is(library.name, 'daipendency_core');
});
