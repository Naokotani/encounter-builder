import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
    proxy: {
      // string shorthand: http://localhost:5173/foo -> http://localhost:4567/foo
      '/foo': 'http://:8081/',
      // with options: http://localhost:5173/api/bar-> http://jsonplaceholder.typicode.com/bar
      '/encounter': {
        target: 'http://127.0.0.1:8081/',
        changeOrigin: false,
        rewrite: (path) => path.replace(/^\/api/, ''),
      },
      '/monster': {
        target: 'http://127.0.0.1:8081/',
        changeOrigin: false,
        rewrite: (path) => path.replace(/^\/api/, ''),
      },
      // with RegEx: http://localhost:5173/fallback/ -> http://jsonplaceholder.typicode.com/
      '^/fallback/.*': {
        target: 'http://jsonplaceholder.typicode.com',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/fallback/, ''),
      },
      // Proxying websockets or socket.io: ws://localhost:5173/socket.io -> ws://localhost:5174/socket.io
      '/socket.io': {
        target: 'ws://localhost:5174',
        ws: true,
      },
    },
  },
});
