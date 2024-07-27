<script lang="ts">
	import * as Accordion from '$lib/components/ui/accordion';
	import * as Card from '$lib/components/ui/card/index';
	import { Button } from '$lib/components/ui/button/index';
	import * as AlertDialog from '$lib/components/ui/alert-dialog/index';
	import { Trash2, BugPlay } from 'lucide-svelte';
	import { onMount } from 'svelte';
	import {
		activeConnectionConfig,
		createConnectionConfig,
		deleteConnectionConfig,
		getAllConnectionConfigs
	} from '../../api/tcc';
	import { fetch_tcc_list, tcc_list } from '$lib/stores/tcc';
	import { toast } from 'svelte-sonner';

	onMount(async () => {
		await fetch_tcc_list();
	});

	const handleActive = async (e: any) => {
		// console.log(e.target.id)
		let res = await activeConnectionConfig(e.target.id);
		if (res.code !== 200) {
			toast.error('Data Fetching Failed', {
				description: res.message,
				position: 'top-right'
			});
			return;
		} else {
			toast.success('success', {
				description: `isActive | ${JSON.stringify(res.data.isActive)}`,
				position: 'top-right'
			});
		}
		await fetch_tcc_list();
	};
	const handleDelete = async (e: any) => {
		let res = await deleteConnectionConfig(e.target.id);
		if (res.code !== 200) {
			toast.error('Data Fetching Failed', {
				description: res.message,
				position: 'top-right'
			});
			return;
		} else {
			toast.success('success', {
				description: `${res.message} | ${JSON.stringify(res.data)}`,
				position: 'top-right'
			});
		}
		await fetch_tcc_list();
	};
</script>

{#each $tcc_list as data}
	<Card.Root
		class="h-[210px] w-[300px] p-0 text-sm {data.isActive ? 'border-2 border-green-500' : ''}"
	>
		<Card.Header class="flex flex-row items-center justify-between">
			<div class="flex flex-col gap-1">
				<Card.Title>{data.env}</Card.Title>
				<Card.Description>{data.id}</Card.Description>
			</div>
			<div class="flex">
				{#if !data.isActive}
					<AlertDialog.Root>
						<AlertDialog.Trigger asChild let:builder>
							<Button variant="outline" size="icon" builders={[builder]}>
								<BugPlay id={data.id} color="green" /></Button
							>
						</AlertDialog.Trigger>
						<AlertDialog.Content>
							<AlertDialog.Header>
								<AlertDialog.Title>Are you absolutely sure to activate it?</AlertDialog.Title>
								<AlertDialog.Description>
									This action will effective immediately and Will disable the currently activated
									version .
								</AlertDialog.Description>
							</AlertDialog.Header>
							<AlertDialog.Footer>
								<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
								<AlertDialog.Action>
									<Button id={data.id} on:click={handleActive} class="w-full">Continue</Button>
								</AlertDialog.Action>
							</AlertDialog.Footer>
						</AlertDialog.Content>
					</AlertDialog.Root>
					<AlertDialog.Root>
						<AlertDialog.Trigger asChild let:builder>
							<Button variant="outline" size="icon" id={data.id} builders={[builder]}>
								<Trash2 id={data.id} color="gray" /></Button
							>
						</AlertDialog.Trigger>
						<AlertDialog.Content>
							<AlertDialog.Header>
								<AlertDialog.Title>Are you absolutely sure?</AlertDialog.Title>
								<AlertDialog.Description>
									This action cannot be undone. This will permanently delete this data from servers.
								</AlertDialog.Description>
							</AlertDialog.Header>
							<AlertDialog.Footer>
								<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
								<AlertDialog.Action>
									<Button id={data.id} on:click={handleDelete} class="w-full">Continue</Button>
								</AlertDialog.Action>
							</AlertDialog.Footer>
						</AlertDialog.Content>
					</AlertDialog.Root>
				{/if}
			</div>
		</Card.Header>
		<Card.Content>
			<Accordion.Root>
				<Accordion.Item value="content">
					<Accordion.Trigger><span class="text-gray-400">{data.url}</span></Accordion.Trigger>
					<Accordion.Content>
						{data.username}/{data.password}
					</Accordion.Content>
				</Accordion.Item>
			</Accordion.Root>
		</Card.Content>
	</Card.Root>
{/each}
