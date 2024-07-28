import { toast } from 'svelte-sonner';

export const copyToClipboard = async (text: string) => {
	try {
		await navigator.clipboard.writeText(text.trim());
		toast.success('success', {
			description: `【${text}】has been copied to the Clipboard`,
			position: 'bottom-right'
		});
	} catch (error) {
		console.error('Failed to copy: ', error);
	}
};
