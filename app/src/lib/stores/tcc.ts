import type { ConnectionConfig } from '$lib/schema';
import { writable } from 'svelte/store';
import { getAllConnectionConfigs } from '../../api/tcc';

export let tcc_list = writable<ConnectionConfig[]>([]);

export const fetch_tcc_list = async () => {
	const res = await getAllConnectionConfigs();
	tcc_list.set(res.data);
};
