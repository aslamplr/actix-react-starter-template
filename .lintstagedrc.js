module.exports = {
  'api/**/*': [
    () => 'npm run lint --prefix api',
    () => 'npm run format --prefix api',
  ],
  'app/**/*': [
    () => 'npm run lint --prefix app',
    () => 'npm run format --prefix app',
  ],
};
