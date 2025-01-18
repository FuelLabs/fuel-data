// rollup.config.js
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import typescript from '@rollup/plugin-typescript';
import json from '@rollup/plugin-json';
import  terser from '@rollup/plugin-terser';

export default {
  input: 'src/index.ts', // Entry point of your application
  output: [
    {
      dir: 'dist',
      format: 'esm', // Output format for browsers (ES Module)
      sourcemap: true,
      entryFileNames: '[name].[hash].js',
      chunkFileNames: '[name].[hash].js',
    },
    {
      dir: 'dist/cjs',
      format: 'cjs', // Output format for Node.js (CommonJS)
      sourcemap: true,
      entryFileNames: '[name].cjs.js',
      chunkFileNames: '[name].cjs.js',
    },
  ],
  plugins: [
    resolve(), // Allows Rollup to resolve node modules
    commonjs(), // Converts CommonJS modules to ES6
    typescript(), // Transpiles TypeScript to JavaScript
    json(), // Allows importing JSON files
    terser(), // Minifies the code for optimization
  ],
  preserveEntrySignatures: 'strict',
};