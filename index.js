const rust = import('./pkg');

rust
  .then(m => m.periodogram_json(0.1, 1.0, 0.1, 1.01))
  .catch(console.error);
