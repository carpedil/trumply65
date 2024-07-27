<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index';
	import { Button } from '$lib/components/ui/button';
	import SettingsForm from '$lib/components/SettingsForm.svelte';
	import SettingsInfo from '$lib/components/SettingsInfo.svelte';
	import { newConfigFlag } from '$lib/stores/flags';
</script>

<div class="w-[90vw]">
	<Card.Root class="h-full w-full">
		<Card.Header>
			<Card.Title class="flex items-center justify-between rounded-sm border pl-1">
				<span>Connection Configs</span>
				<Button variant="outline" on:click={() => newConfigFlag.set(true)}>NEW</Button>
			</Card.Title>
		</Card.Header>
		<Card.Content>
			<ScrollArea class="flex h-[250px]  w-full rounded-md border p-4 " orientation="horizontal">
				<div class="flex flex-row justify-start gap-4">
					<SettingsInfo />
				</div>
			</ScrollArea>
		</Card.Content>
	</Card.Root>
	<SettingsForm />
</div>
