import { SrvResult, type ConnectionConfig } from '$lib/schema';
import { handleSrvResult } from './index';

export const getAllConnectionConfigs = async (): Promise<SrvResult<ConnectionConfig[]>> => {
	const query = `
          query {
				getAllTcc {
					id
					env
					dbType
					url
					username
					password
					isActive
					abandonedTableList
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
	return handleSrvResult(data.getAllTcc, errors);
};

export const createConnectionConfig = async (
	rc: ConnectionConfig
): Promise<SrvResult<ConnectionConfig>> => {
	const mutation = `
    mutation {
        createTcc(input:{
            env:"${rc.env}",
            dbType: "${rc.dbType}",
            databaseUrl:"${rc.url}",
            username:"${rc.username}",
            password:"${rc.password}",
            isActive: false
        }) {
            id
        }
    }
    `;
	console.log('mutation >> ', mutation);
	const response = await fetch('/api/graphql', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ query: mutation })
	});
	const { data, errors } = await response.json();
	return handleSrvResult(data.createTcc, errors);
};

export const activeConnectionConfig = async (id: number): Promise<SrvResult<ConnectionConfig>> => {
	const mutation = `
        mutation {
            activeTccById(id: "${id}") {
                id
                dbType
                env
                url
                username
                password
                isActive
                abandonedTableList
            }
        }
    `;
	console.log('mutation >> ', mutation);

	const response = await fetch('/api/graphql', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ query: mutation })
	});
	const { data, errors } = await response.json();
	console.log(data, errors);
	return handleSrvResult(data.activeTccById, errors);
};

export const deleteConnectionConfig = async (id: number): Promise<SrvResult<Number>> => {
	const mutation = `
        mutation {
            deleteTccById(id: "${id}") {
                rowsAffected
            }
        }
    `;
	console.log('mutation >> ', mutation);

	const response = await fetch('/api/graphql', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ query: mutation })
	});
	const { data, errors } = await response.json();
	return handleSrvResult(data.deleteTccById, errors);
};

export const updateBannedTableList = async (table_name: string): Promise<SrvResult<String>> => {
	const mutation = `
        mutation {
            updateBanTableList(bannedTable:{
                tableName:"${table_name}"
            }){
                bannedTableList
            }
        }
    `;
	console.log('mutation >> ', mutation);

	const response = await fetch('/api/graphql', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ query: mutation })
	});
	const { data, errors } = await response.json();
	return handleSrvResult(data.updateBannedTableList, errors);
};
