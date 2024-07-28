import { ExportSpecInput, SrvResult, TableColumnsInfo, TableData } from '$lib/schema';
import { handleSrvResult, serializeExportSpecInput } from './utils';

export const loadDatasourceTables = async (): Promise<SrvResult<TableColumnsInfo[]>> => {
	const query = `
        query {
            loadDatasourceTables{
                tableName
                columnInfos {
                    columnName
                    dataType
                    dataLen
                }
            }
        }`;
	console.log('query >> ', query);

	const response = await fetch('/api/graphql', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ query })
	});
	const { data, errors } = await response.json();
	return handleSrvResult(data.loadDatasourceTables, errors);
};

export const getCurrentTableData = async (
	param: ExportSpecInput
): Promise<SrvResult<TableData[]>> => {
	const querySql = param.querySql.replaceAll('\n', ' ');
	const query = `
        query {
			getTableData(tableInfo:${serializeExportSpecInput([param])}) {
				data
			}
		}`;
	console.log('query >> ', query);

	const response = await fetch('/api/graphql', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ query })
	});
	const { data, errors } = await response.json();
	return handleSrvResult(data.getTableData, errors);
};

export const exportAllTableData = async (param: ExportSpecInput[]): Promise<SrvResult<string>> => {
	console.log('pa', param);
	const query = `
	query {
		dumpDatasourceTables(dumpSpec:[${serializeExportSpecInput(param)}])
	}`;

	console.log('query >> ', query);

	const response = await fetch('/api/graphql', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ query })
	});
	const { data, errors } = await response.json();
	console.log(data, errors);
	return handleSrvResult(data.dumpDatasourceTables, errors);
};
