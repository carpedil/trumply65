import { loadDatasourceTables } from '$lib/api/db';
import { ExportSpecInput, TableColumnsInfo } from '$lib/schema';
import { writable } from 'svelte/store';

export let table_list = writable<TableColumnsInfo[]>([]);
export let table_selected = writable<ExportSpecInput>(new ExportSpecInput());

export const fetch_table_list = async () => {
	const res = await loadDatasourceTables();
	table_list.set(res.data);
};

export const set_table_selected = (table: ExportSpecInput) => {
	table_selected.set(table);
};
