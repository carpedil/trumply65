<script lang="ts">
	import * as Card from '$lib/components/ui/card/index';
	import * as Resizable from '$lib/components/ui/resizable/index';
	import * as Tabs from '$lib/components/ui/tabs/index';
	import * as Table from '$lib/components/ui/table/index';
	import { Button } from '$lib/components/ui/button/index';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index';
	import * as ContextMenu from '$lib/components/ui/context-menu';
	import { Textarea } from '$lib/components/ui/textarea/index';
	import { Ban, ListChecks, ScanSearch, Save } from 'lucide-svelte';
	import SqlBlock from '$lib/components/SqlBlock.svelte';
	import { onMount } from 'svelte';
	import { fetch_table_list, table_list, set_table_selected, table_selected } from '$lib/stores/db';
	import * as Command from '$lib/components/ui/command/index';
	import { copyToClipboard } from '$lib';
	import { ExportSpecInput, TableColumnsInfo, TableData } from '$lib/schema';
	import { exportAllTableData, getCurrentTableData } from '$lib/api/db';
	import { exportBreathingFlag } from '$lib/stores/flags';
	import { toast } from 'svelte-sonner';

	let tableName = '';

	onMount(async () => {
		await fetch_table_list();
	});

	const handleClick = async (e: any) => {
		tableName = e.target.innerHTML;
		await copyToClipboard(tableName);
		const tableInfo = $table_list.filter((item) => item.tableName === tableName)[0];
		let esi = new ExportSpecInput();
		esi.set_table_name(tableInfo.tableName);
		esi.set_headers(tableInfo.columnInfos);
		esi.set_query_sql('');
		set_table_selected(esi);
	};
	let tableData: TableData[] = [];
	const handleDataQuery = async () => {
		let res = await getCurrentTableData($table_selected);
		tableData = res.data;
	};

	const handleExport = async () => {
		exportBreathingFlag.set(true);
		let list: ExportSpecInput[] = [];
		for (const table of $table_list) {
			let esi = new ExportSpecInput();
			esi.set_table_name(table.tableName);
			esi.set_headers(table.columnInfos);
			esi.set_query_sql('');
			list.push(esi);
		}
		let res = await exportAllTableData(list);
		exportBreathingFlag.set(false);
		if (res.code !== 200) {
			toast.error('Data Export Failed', {
				description: res.message,
				position: 'top-right'
			});
			return;
		} else {
			toast.success('success', {
				description: `${JSON.stringify(res.data)}`,
				position: 'top-right'
			});
		}
	};
</script>

<div class="h-full w-full">
	<Resizable.PaneGroup direction="vertical" class="min-h-[100vh]  max-w-[100vw] rounded-lg border ">
		<Resizable.Pane defaultSize={100}>
			<Resizable.PaneGroup direction="horizontal" class="min-h-[90vh] max-w-[100vw] rounded-lg">
				<Resizable.Pane defaultSize={20}>
					<Tabs.Root value="tables" class="h-fit ">
						<Tabs.List class="flex w-full text-xs">
							<Tabs.Trigger value="tables" class="flex-1 text-xs">
								<ListChecks />Tables
							</Tabs.Trigger>
							<Tabs.Trigger value="mutedTables" class="flex-1 text-xs"><Ban />Banned</Tabs.Trigger>
						</Tabs.List>
						<Tabs.Content value="tables" class="p-1">
							<ContextMenu.Root>
								<ContextMenu.Trigger>
									<ScrollArea class="flex-1 pt-1" orientation="vertical">
										<Command.Root>
											<Command.Input placeholder="Type a tableName to search..." />
											<Command.Empty>No results found.</Command.Empty>
											<Command.List>
												<Command.Group heading="Table List">
													<ul>
														{#each $table_list as table}
															<li>
																<Command.Item class="select-text p-0 text-xs dark:text-blue-400">
																	<button
																		class={tableName === table.tableName
																			? '!h-fit w-full rounded-md bg-green-400 p-1 text-left  dark:bg-slate-500'
																			: 'h-fit w-full p-1  text-left'}
																		on:click={handleClick}>{table.tableName}</button
																	>
																</Command.Item>
															</li>
														{/each}
													</ul>
												</Command.Group>
											</Command.List>
										</Command.Root>
									</ScrollArea>
								</ContextMenu.Trigger>
								<ContextMenu.Content>
									<ContextMenu.Item>
										<button class="h-fit w-full p-1 text-left" on:click={handleExport}
											>Export to Excel</button
										>
									</ContextMenu.Item>
								</ContextMenu.Content>
							</ContextMenu.Root>
							<div class="flex h-[45vh] flex-col">
								<span class="select-none border p-1">Total Count:{$table_list.length}</span>
								<ScrollArea class="max-h-[35vh] w-full p-1 pt-1" orientation="both">
									<table class="text-xs text-slate-500">
										<thead>
											<tr class="border bg-slate-300 p-1 dark:bg-slate-600 dark:text-gray-300">
												<td>COLUMN_NAME</td>
												<td>DATA_TYPE</td>
												<td>DATA_LENGTH</td>
											</tr>
										</thead>
										<tbody>
											{#each $table_selected.headers as header}
												<tr>
													<td>{header.columnName}</td>
													<td>{header.dataType}</td>
													<td>{header.dataLen}</td>
												</tr>
											{/each}
										</tbody>
									</table>
								</ScrollArea>
							</div>
						</Tabs.Content>
						<Tabs.Content value="mutedTables" class="p-1">TBD</Tabs.Content>
					</Tabs.Root>
				</Resizable.Pane>
				<Resizable.Handle withHandle />
				<Resizable.Pane defaultSize={80}>
					<Resizable.PaneGroup
						direction="horizontal"
						class="min-h-[100vh]  max-w-[100vw] rounded-lg "
					>
						<Resizable.Pane defaultSize={100}>
							<Resizable.PaneGroup
								direction="vertical"
								class="min-h-[90vh] max-w-[100vw] rounded-lg "
							>
								<Resizable.Pane defaultSize={35}>
									<div class="flex h-full w-full items-start justify-start p-2">
										<div class="h-full flex-1">
											<Textarea
												placeholder="Type your message here."
												bind:value={$table_selected.querySql}
												class="h-full text-slate-400"
											/>
										</div>
										<div class="h-full flex-1 overflow-y-auto p-0">
											<SqlBlock code={$table_selected.querySql} />
										</div>
									</div>
								</Resizable.Pane>
								<Resizable.Handle withHandle />
								<Resizable.Pane defaultSize={65}>
									<Card.Root class="flex flex-row items-center gap-1 rounded-sm border p-1">
										<Button size="icon" variant="outline" class="p-0" on:click={handleDataQuery}
											><ScanSearch /></Button
										>
										<Button
											size="icon"
											variant="outline"
											class="p-0"
											on:click={() => console.log('clicked')}><Save /></Button
										>
									</Card.Root>
									<ScrollArea
										class="h-[55vh] w-[72vw] flex-1 overflow-x-auto whitespace-nowrap  border p-2"
										orientation="both"
									>
										<Table.Root class="text-xs text-slate-500">
											<Table.Header class="border">
												<Table.Row>
													{#each $table_selected.headers as heaser}
														<Table.Head>{heaser.columnName}</Table.Head>
													{/each}
												</Table.Row>
											</Table.Header>
											<Table.Body>
												{#each tableData as item}
													<Table.Row>
														{#each item.data as value}
															<Table.Cell class="font-medium">{value}</Table.Cell>
														{/each}
													</Table.Row>
												{/each}
											</Table.Body>
										</Table.Root>
									</ScrollArea>
								</Resizable.Pane>
							</Resizable.PaneGroup>
						</Resizable.Pane>
					</Resizable.PaneGroup>
				</Resizable.Pane>
			</Resizable.PaneGroup>
		</Resizable.Pane>
	</Resizable.PaneGroup>
</div>
