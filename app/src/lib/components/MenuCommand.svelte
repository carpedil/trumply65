<script lang="ts">
	import * as Command from '$lib/components/ui/command';
</script>

<Command.Root class="w-[15vw]">
	<Command.List>
		<Command.Group heading="Settings">
			<Command.Item>
				<a href="/">Configuation</a>
			</Command.Item>
		</Command.Group>
		<Command.Separator />
		<Command.Group heading="Functions">
			<Command.Item>
				<a href="/query">Query</a>
			</Command.Item>
			<!-- <Command.Item>
				<a href="/export">Export</a>
			</Command.Item>
			<Command.Item>
				<a href="/playground">Playground</a>
			</Command.Item> -->
		</Command.Group>
	</Command.List>
</Command.Root>
