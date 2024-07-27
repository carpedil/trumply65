<script lang="ts">
	import { Button } from '$lib/components/ui/button/index';
	import * as Card from '$lib/components/ui/card/index';
	import * as Select from '$lib/components/ui/select/index';
	import { Input } from '$lib/components/ui/input/index';
	import { toast } from 'svelte-sonner';
	import * as Form from '$lib/components/ui/form';
	import { type SuperValidated, type Infer, superForm, superValidate } from 'sveltekit-superforms';
	import { dbTypes, envs, type FormSchema, formSchema, validateFormData } from '$lib/schema';
	import { zod, zodClient } from 'sveltekit-superforms/adapters';
	import { ScrollArea } from './ui/scroll-area/index';
	import { isOracle, newConfigFlag } from '$lib/stores/flags';
	import { fetch_tcc_list } from '$lib/stores/tcc';
	import { createConnectionConfig } from '../../api/tcc';
	import { onMount } from 'svelte';

	let data: SuperValidated<Infer<FormSchema>> = {
		id: 'data',
		valid: false,
		posted: false,
		errors: {},
		data: {
			env: '',
			dbType: '',
			url: '',
			username: '',
			password: '',
			isActive: false
		}
	};
	onMount(async () => {
		data = await superValidate(zod(formSchema));
	});

	const form = superForm(data, {
		validators: zodClient(formSchema)
	});

	const { form: formData } = form;

	const handleSave = async () => {
		const validResult = validateFormData($formData);
		if (validResult.length) {
			toast.error('validation error', {
				description: `Check fields【${validResult.join(' | ')}】 ,They can not be null`,
				position: 'top-right'
			});
			return;
		}
		newConfigFlag.set(false);
		let res = await createConnectionConfig({
			id: '',
			dbType: $formData.dbType,
			env: $formData.env,
			url: $formData.url,
			username: $formData.username,
			password: $formData.password,
			isActive: false,
			abandonedTableList: ''
		});
		if (res.code !== 200) {
			toast.error('Data Fetching Failed', {
				description: res.message,
				position: 'top-right'
			});
			return;
		} else {
			toast.success('success', {
				description: `${res.message} | ${JSON.stringify(res.data.id)}`,
				position: 'top-right'
			});
		}
		await fetch_tcc_list();
	};

	$: selectedEnv = {
		label: $formData.env,
		value: $formData.env
	};
	$: selectedDbType = {
		label: $formData.dbType,
		value: $formData.dbType
	};
	$: $formData.dbType === 'Oracle' ? isOracle.set(true) : isOracle.set(false);
</script>

{#if $newConfigFlag}
	<div class="flex">
		<ScrollArea
			class="h-full {$isOracle ? 'flex-1' : 'w-[50%]'} rounded-md pt-1"
			orientation="vertical"
		>
			<Card.Root>
				<Card.Header class="flex flex-row items-center justify-between">
					<div>
						<Card.Title>Create Configs</Card.Title>
						<Card.Description>information about your database connection</Card.Description>
					</div>
					<Button on:click={handleSave} variant="outline">Save</Button>
				</Card.Header>
				<Card.Content>
					<Form.Field {form} name="env">
						<Form.Control let:attrs>
							<Form.Label>Env</Form.Label>
							<Select.Root
								selected={selectedEnv}
								onSelectedChange={(s) => {
									s && ($formData.env = s.value);
								}}
							>
								<Select.Trigger {...attrs}>
									<Select.Value placeholder="In what environment is this configuration used?" />
								</Select.Trigger>
								<Select.Content>
									{#each envs as value}
										<Select.Item {value} label={value} />
									{/each}
								</Select.Content>
							</Select.Root>
						</Form.Control>
					</Form.Field>
					<Form.Field {form} name="dbType">
						<Form.Control let:attrs>
							<Form.Label>dbType</Form.Label>
							<Select.Root
								selected={selectedDbType}
								onSelectedChange={(s) => {
									s && ($formData.dbType = s.value);
								}}
							>
								<Select.Trigger {...attrs}>
									<Select.Value placeholder="What is the database type?" />
								</Select.Trigger>
								<Select.Content>
									{#each dbTypes as value}
										<!-- disabled={value !== "Oracle" ? true:false} -->
										<Select.Item {value} label={value} />
									{/each}
								</Select.Content>
							</Select.Root>
						</Form.Control>
					</Form.Field>
					<Form.Field {form} name="url">
						<Form.Control let:attrs>
							<Form.Label>Url</Form.Label>
							<Input
								{...attrs}
								placeholder="What is the database url? eg. host:port/db_name"
								bind:value={$formData.url}
							/>
						</Form.Control>
						<Form.FieldErrors />
					</Form.Field>
					<Form.Field {form} name="username">
						<Form.Control let:attrs>
							<Form.Label>Username</Form.Label>
							<Input
								autocomplete="true"
								{...attrs}
								placeholder="The username to connect to database"
								bind:value={$formData.username}
							/>
						</Form.Control>
						<Form.FieldErrors />
					</Form.Field>
					<Form.Field {form} name="password">
						<Form.Control let:attrs>
							<Form.Label>Password</Form.Label>
							<Input
								type="password"
								autocomplete="true"
								{...attrs}
								placeholder="The password to connect to database"
								bind:value={$formData.password}
							/>
						</Form.Control>
						<Form.FieldErrors />
					</Form.Field>
				</Card.Content>
			</Card.Root>
		</ScrollArea>
		{#if $isOracle}
			<ScrollArea class="h-full  flex-1 pt-1" orientation="vertical">
				<Card.Root class="h-[53vh]">
					<Card.Content>
						<h2>Oracle Driver Install Guide</h2>
						<p>1. 查看数据库版本</p>
						<!-- <CodeBlock language="sql" class="w-full overflow-x-auto" code={check_db_verson}></CodeBlock> -->
						<p>2. 根据数据库版本下载Oracle DB Driver</p>
						<!-- <CodeBlock language="markdown" class="w-full overflow-x-auto" code={"https://www.oracle.com/database/technologies/instant-client/winx64-64-downloads.html"}></CodeBlock> -->
						<img src="./driver_download.png" alt="" />
						<!-- <p>3. 将下载的*.zip解压到你想存放的路径,例如<code>C:\Program Files\Driver\[your_instantclient_dir]<code></p> -->
						<p>
							4. 在C:\Program
							Files\Driver\[your_instantclient_dir]目录中创建/network/admin/tnsnames.ora等目录以及文件
						</p>
						<p>5. 在tnsnames.ora中配置Oracle数据库连接信息即可</p>
					</Card.Content>
				</Card.Root>
			</ScrollArea>
		{/if}
	</div>
{/if}
