import { ColumnData, ExportSpecInput, SrvResult } from '$lib/schema';
import { onDestroy } from 'svelte';

export function onInterval(callback: () => void, milliseconds: number | undefined) {
	const interval = setInterval(callback, milliseconds);

	onDestroy(() => {
		clearInterval(interval);
	});
}

export const handleSrvResult = <T>(data: T, errors: string): SrvResult<T> => {
	console.log(data, errors);
	if (errors) {
		console.error('GraphQL errors:', errors);
		return new SrvResult(500, errors, null);
	}
	console.log('response from backend: ', data);
	return new SrvResult(200, 'success', data);
};

export const serializeHeaders = (headers: ColumnData[]): string => {
	return headers
		.map((header) => {
			return `{columnName: "${header.columnName}",dataType:"${header.dataType}",dataLen:${header.dataLen}}`;
		})
		.join(',');
};

export const serializeExportSpecInput = (spec: ExportSpecInput[]): string => {
	return spec
		.map((s) => {
			return `{ 
		  tableName: "${s.tableName}", 
		  headers: [${serializeHeaders(s.headers)}], 
		  querySql: "${s.querySql.replaceAll('\n', ' ')}" 
		}`;
		})
		.join(', ');
};
