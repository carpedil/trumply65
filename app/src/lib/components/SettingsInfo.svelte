<script lang="ts">
	import * as Accordion from '$lib/components/ui/accordion';
	import * as Card from '$lib/components/ui/card/index';
	import { Trash2, BugPlay } from 'lucide-svelte';
	const frameworks = [
		{
			env: 'Test',
			dbType: 'Oracle',
			url: '127.0.0.1:1521/testdb',
			username: 'demo',
			password: 'demo123',
			isActive: true
		},
		{
			env: 'Dev',
			dbType: 'Oracle',
			url: '127.0.0.1:1521/devdb',
			username: 'demo',
			password: 'demo123',
			isActive: false
		},
		{
			env: 'Prod',
			dbType: 'Oracle',
			url: '127.0.0.1:1521/proddb',
			username: 'admin',
			password: 'admin123',
			isActive: false
		}
	];

	const handleActive = () => {};
	const handleDeActive = () => {};
</script>

{#each frameworks as data}
	<Card.Root
		class="h-[210px] w-[300px] p-0 text-sm {data.isActive ? 'border-2 border-green-500' : ''}"
	>
		<Card.Header class="flex flex-row items-center justify-between">
			<div>
				<Card.Title>{data.env}</Card.Title>
				<Card.Description>{data.url}</Card.Description>
			</div>
			<div class="flex">
				<BugPlay color={data.isActive ? '' : 'green'} on:click={handleActive} /><Trash2
					color={data.isActive ? '' : 'gray'}
					on:click={handleDeActive}
				/>
			</div>
		</Card.Header>
		<Card.Content>
			<Accordion.Root>
				<Accordion.Item value="item-1">
					<Accordion.Trigger>Account Infos</Accordion.Trigger>
					<Accordion.Content>
						{data.username}/{data.password}
					</Accordion.Content>
				</Accordion.Item>
			</Accordion.Root>
		</Card.Content>
	</Card.Root>
{/each}
