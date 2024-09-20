import { playwrightLauncher } from '@web/test-runner-playwright'

/**
 * @type {import('@web/test-runner').TestRunnerConfig}
 */
export default {
  files       : ['./tests/compiled/browser/**/*.test.js'],
  playwright  : true,
  // nodeResolve : true,
  browsers    : [
    playwrightLauncher({
      product: 'chromium',
    }),
    playwrightLauncher({
      product: 'firefox',
    }),
    playwrightLauncher({
      product: 'webkit',
    }),
  ],
  testsFinishTimeout : 300000,
  concurrentBrowsers : 1,
  testFramework      : {
    config: {
      timeout: '15000',
    },
  },
};
