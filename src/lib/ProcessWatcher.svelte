<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { listen } from '@tauri-apps/api/event';
	import { readTextFile } from '@tauri-apps/api/fs';
	import { onMount, onDestroy } from 'svelte';
	import { isAllowed, currentTitleName, previousTitleName } from './timerStore';
	import { message } from '@tauri-apps/api/dialog';
	import yaml from 'yaml';

	interface WindowPosition {
		x: number;
		y: number;
		width: number;
		height: number;
	}

	interface ActiveWindow {
		title: string;
		process_path: string;
		app_name: string;
		window_id: string;
		process_id: number;
		position: WindowPosition;
	}

	interface WhitelistItem {
		title?: string;
		process_path?: string;
		app_name?: string;
		window_id?: string;
		process_id?: number;
	}

	let whitelist: WhitelistItem[] = [];

	listen('tauri://file-drop', (event) => {
		apply_settings((event.payload as string[])[0]);
	});

	async function apply_settings(path: string): Promise<void> {
	try {
		const contents = await readTextFile(path);
		let settings: any;
		try {
			settings = yaml.parse(contents);
		} catch (error) {
			message('Invalid YAML syntax');
			return;
		}

		if (!settings.whitelist || !Array.isArray(settings.whitelist)) {
			message('Invalid whitelist format');
			return;
		}

		const allowedKeys = ['title', 'process_path', 'app_name', 'window_id', 'process_id'];
		let invalidKeys = new Set<string>();
		let hasValuesWithoutKeys = false;
		const invalidItems = settings.whitelist.filter((item: any) => {
			if (item && typeof item === 'object' && !Array.isArray(item)) {
				const itemKeys = Object.keys(item);
				const hasInvalidKeys = itemKeys.some((key) => {
					const isInvalidKey = !allowedKeys.includes(key);
					if (isInvalidKey) {
						invalidKeys.add(key);
					}
					return isInvalidKey;
				});
				return hasInvalidKeys;
			} else {
				hasValuesWithoutKeys = true;
				return true;
			}
		});

		if (hasValuesWithoutKeys) {
			message('Whitelist contains values without keys. Make sure you add a space after the ":" character.');
			return;
		}

		if (invalidItems.length > 0) {
			message(`Invalid fields in whitelist items: ${[...invalidKeys].join(', ')}`);
			return;
		}

		whitelist = settings.whitelist;
		message('Settings applied');
	} catch (error) {
		message('Settings Error');
	}
}

	async function get_window(): Promise<void> {
		try {
			const result = await invoke<ActiveWindow>('get_window');
			previousTitleName.set($currentTitleName);
			currentTitleName.set(JSON.stringify(result));
			set_allowed(result);
		} catch (error) {
			console.error('Error occurred while getting the active window:', error);
		}
	}

	function set_allowed(window: ActiveWindow) {
		const isWindowAllowed = whitelist.some((item) => {
			return (
				(!item.title || new RegExp(item.title).test(window.title)) &&
				(!item.process_path || new RegExp(item.process_path).test(window.process_path)) &&
				(!item.app_name || new RegExp(item.app_name).test(window.app_name)) &&
				(!item.window_id || new RegExp(item.window_id).test(window.window_id)) &&
				(!item.process_id || item.process_id === window.process_id)
			);
		});
		isAllowed.set(isWindowAllowed);
	}

	let intervalId: number | undefined;

	onMount(() => {
		intervalId = window.setInterval(get_window, 100);

		return () => {
			if (intervalId !== undefined) {
				clearInterval(intervalId);
			}
		};
	});

	onDestroy(() => {
		if (intervalId !== undefined) {
			clearInterval(intervalId);
		}
	});
</script>
