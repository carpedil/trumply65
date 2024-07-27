import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		fs: {
			allow: ['../static/']
		},
		cors: true,
		proxy: {
			// 配置代理规则
			'/api': {
				target: 'http://localhost:18000', // API 服务的地址
				changeOrigin: true,
				rewrite: (path) => path.replace(/^\/api/, '/api')
			}
		}
	}
});
