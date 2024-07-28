<script lang="ts">
	import Highlight, { LineNumbers } from 'svelte-highlight';
	import sql from 'svelte-highlight/languages/sql';
	import atomOneDark from 'svelte-highlight/styles/atom-one-dark-reasonable';
	export let code: string;
</script>

<svelte:head>
	{@html atomOneDark}
</svelte:head>
<Highlight langtag={true} --langtag-right={true} language={sql} {code} let:highlighted>
	<LineNumbers {highlighted} hideBorder />
</Highlight>
