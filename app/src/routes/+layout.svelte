<script lang="ts">
	import '../app.css';
	import * as Resizable from '$lib/components/ui/resizable/index';
	import MenuCommand from '$lib/components/MenuCommand.svelte';
	import ModelWatcher from '$lib/components/ModelWatcher.svelte';
	import * as Avatar from '$lib/components/ui/avatar/index';
	import AppToaster from '$lib/components/AppToaster.svelte';
	import ExportBreathingLight from '$lib/components/ExportBreathingLight.svelte';
	import { exportBreathingFlag } from '$lib/stores/flags';
</script>

<!-- toaster -->
<AppToaster />

<Resizable.PaneGroup direction="vertical" class="min-h-[100vh]  max-w-[100vw] rounded-lg border">
	<Resizable.Pane defaultSize={5}>
		<div class="flex h-full w-full items-center justify-between p-4">
			<div class="flex w-full flex-row items-center p-1.5">
				<Avatar.Root>
					<Avatar.Image
						src="https://avatars.githubusercontent.com/u/12119342?v=4"
						alt="@shadcn"
						sizes="sm"
					/>
					<Avatar.Fallback>CPX</Avatar.Fallback>
				</Avatar.Root>
			</div>
			<div class="card flex flex-row">
				{#if $exportBreathingFlag}
					<ExportBreathingLight />
				{/if}
				<span class="font-semibold"><ModelWatcher /></span>
			</div>
		</div>
	</Resizable.Pane>
	<Resizable.Handle />
	<Resizable.Pane defaultSize={95}>
		<Resizable.PaneGroup
			direction="horizontal"
			class="min-h-[90vh] max-w-[100vw] rounded-lg border"
		>
			<Resizable.Pane defaultSize={15}>
				<div class="flex h-full w-full items-start justify-center p-2">
					<span class="font-semibold">
						<MenuCommand />
					</span>
				</div>
			</Resizable.Pane>
			<Resizable.Handle withHandle />
			<Resizable.Pane defaultSize={85}>
				<div class="flex h-full items-start justify-start p-2">
					<slot />
				</div>
			</Resizable.Pane>
		</Resizable.PaneGroup>
	</Resizable.Pane>
</Resizable.PaneGroup>
