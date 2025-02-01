import test from 'ava'

import { Library, Language } from '../index.js'

test('Load dependency', async (t) => {
  if (process.platform === 'win32') {
    t.pass('Skipped on Windows, where it hangs intermittently on CI');
    return;
  }

  const library = await Library.loadDependency('tokio', '.', Language.Rust);

  t.is(library.name, 'tokio');
});

test('Load library', async (t) => {
  const library = await Library.load('./', Language.Rust);

  t.is(library.name, 'daipendency_core');
});
