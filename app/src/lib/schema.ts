import { z } from 'zod';
import { format } from 'sql-formatter';

export const formSchema = z.object({
	env: z.enum(['', 'Test', 'Dev', 'Prod']).default('Test'),
	dbType: z.enum(['', 'Postgress', 'Oracle', 'MySQL']),
	url: z.string().max(20),
	username: z.string().max(15),
	password: z.string().max(15),
	isActive: z.boolean().default(false)
});

export type FormSchema = typeof formSchema;

export const validateFormData = (form: Object): string[] => {
	const emptyFields: string[] = [];
	// // 遍历对象的所有键
	// for (const key in form) {
	// 	if (form.hasOwnProperty(key)) {
	// 		const value = form[key as keyof Object]; // 使用 keyof 操作符
	// 		if (typeof value === 'string' && value === '') {
	// 			emptyFields.push(key);
	// 		}
	// 	}
	// }
	// 检查对象中的每个属性
	for (const [key, value] of Object.entries(form)) {
		// 如果属性是字符串类型，并且值为空字符串，则添加到emptyFields数组中
		if (typeof value === 'string' && value.trim() === '') {
			emptyFields.push(key);
		}
	}
	return emptyFields;
};

export const envs = ['Test', 'Dev', 'Prod'];
export const dbTypes = ['Postgress', 'Oracle', 'MySQL'];

export class ConnectionConfig {
	public id: string = '';
	public dbType: string = '';
	public env: string = '';
	public url: string = '';
	public username: string = '';
	public password: string = '';
	public isActive: boolean = false;
	public abandonedTableList: string = '';
}

export class TableColumnsInfo {
	tableName: string = '';
	columnInfos: ColumnData[] = [];
}

export class ColumnData {
	public columnName: string = '';
	public dataType: string = '';
	public dataLen: number = 0;
}

export class ExportSpecInput {
	public tableName: string = '';
	public headers: ColumnData[] = [];
	public querySql: string = '';

	constructor() {
		this.tableName = '';
		this.headers = [];
		this.querySql = '';
	}

	set_table_name(table_name: string) {
		this.tableName = table_name;
	}

	set_headers(column_infos: ColumnData[]) {
		this.headers = column_infos;
	}

	set_query_sql(where_clause: string) {
		const select_fields = this.headers.map((header) => header.columnName).join(', ');
		console.log('select_fields', select_fields);
		where_clause === ''
			? (this.querySql = `SELECT ${select_fields} FROM ${this.tableName} ORDER BY ${this.headers[1].columnName}`)
			: (this.querySql = `SELECT ${select_fields} FROM ${this.tableName} ${where_clause.toUpperCase()} ORDER BY ${this.headers[1].columnName} `);

		this.querySql = format(this.querySql.trim(), { language: 'sql' });
	}
}

export class SrvResult<T> {
	public code: number = 200;
	public message: string = 'success';
	public data: T;

	constructor(code: number, message: string, data: any) {
		this.code = code;
		this.message = message;
		this.data = data;
	}
}

export class GlobalMessage {
	public code: number | undefined = undefined;
	public message: string | undefined = undefined;
	public open: boolean = false;

	constructor(code: number, message: string) {
		this.code = code;
		this.message = message;
		this.open = true;
	}
}

export class BannedTableInfo {
	public tableName: string = '';
	public tableList: string[] = [];
	constructor(tableName: string, tableList: string[]) {
		this.tableName = tableName;
		const notExists = !tableList.includes(tableName);
		this.tableList = notExists && tableName != '' ? [...tableList, tableName] : tableList;
	}
}

export class TableData {
	data: string[] = [];
}
