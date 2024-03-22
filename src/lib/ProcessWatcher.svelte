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
			const settings = yaml.parse(contents) as { whitelist: WhitelistItem[] };

			if (!settings.whitelist || !Array.isArray(settings.whitelist)) {
				message('Error');
			} else {
				whitelist = settings.whitelist;
				message('Settings applied');
			}
		} catch (error) {
			message('Error');
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
