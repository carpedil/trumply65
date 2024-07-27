import { z } from 'zod';

export const formSchema = z.object({
	env: z.enum(['', 'Test', 'Dev', 'Prod']).default('Test'),
	dbType: z.enum(['', 'Postgress', 'Oracle', 'MySQL']),
	url: z.string().max(20),
	username: z.string().max(15),
	password: z.string().max(15),
	isActive: z.boolean().default(false)
});

export type FormSchema = typeof formSchema;

export const envs = ['Test', 'Dev', 'Prod'];
export const dbTypes = ['Postgress', 'Oracle', 'MySQL'];
