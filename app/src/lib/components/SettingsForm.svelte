<script lang="ts">
	import { Button } from '$lib/components/ui/button/index';
	import * as Card from '$lib/components/ui/card/index';
	import * as Select from '$lib/components/ui/select/index';
	import { Input } from '$lib/components/ui/input/index';
	import { toast } from 'svelte-sonner';
	import * as Form from '$lib/components/ui/form';
	import { type SuperValidated, type Infer, superForm } from 'sveltekit-superforms';
	import { dbTypes, envs, type FormSchema, formSchema } from '$lib/schema';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { ScrollArea } from './ui/scroll-area/index';
	import { isOracle, newConfigFlag } from '$lib/stores/flags';

	let data: SuperValidated<Infer<FormSchema>> = {
		id: 'data',
		valid: true,
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

	const form = superForm(data, {
		validators: zodClient(formSchema)
	});

	const { form: formData } = form;

	const handleSave = () => {
		newConfigFlag.set(false);
		toast.success('well done', {
			description: 'sss',
			position: 'top-right'
		});
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
					<Button on:click={handleSave}>Save</Button>
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
									<Select.Value placeholder="Select Env" />
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
									<Select.Value placeholder="Select dbType" />
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
							<Input {...attrs} bind:value={$formData.url} />
						</Form.Control>
					</Form.Field>
					<Form.Field {form} name="username">
						<Form.Control let:attrs>
							<Form.Label>Username</Form.Label>
							<Input {...attrs} bind:value={$formData.username} />
						</Form.Control>
					</Form.Field>
					<Form.Field {form} name="password">
						<Form.Control let:attrs>
							<Form.Label>Password</Form.Label>
							<Input type="password" {...attrs} bind:value={$formData.password} />
						</Form.Control>
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
