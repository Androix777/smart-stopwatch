<script lang="ts">
	import { appWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';
	import { previousTitleName } from './timerStore';
	import { writeText } from '@tauri-apps/api/clipboard';

	function setupTitlebarButtons() {
		const minimizeButton = document.getElementById('titlebar-minimize');
		const maximizeButton = document.getElementById('titlebar-maximize');
		const closeButton = document.getElementById('titlebar-close');
		const copyButton = document.getElementById('titlebar-copy');

		if (minimizeButton) {
			minimizeButton.addEventListener('click', () => appWindow.minimize());
		}

		if (maximizeButton) {
			maximizeButton.addEventListener('click', () => appWindow.toggleMaximize());
		}

		if (closeButton) {
			closeButton.addEventListener('click', () => appWindow.close());
		}

		if (copyButton) {
			copyButton.addEventListener('click', () => writeText($previousTitleName));
		}
	}

	onMount(setupTitlebarButtons);
</script>

<div data-tauri-drag-region class="titlebar">
	<div class="titlebar-buttons">
		<div class="titlebar-button" id="titlebar-copy">C</div>
		<div class="titlebar-button" id="titlebar-minimize">
			<img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize" />
		</div>
		<div class="titlebar-button" id="titlebar-maximize">
			<img src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize" />
		</div>
		<div class="titlebar-button" id="titlebar-close">
			<img src="https://api.iconify.design/mdi:close.svg" alt="close" />
		</div>
	</div>
</div>

<style>
	.titlebar {
		height: 30px;
		background: #00000033;
		user-select: none;
		display: flex;
		justify-content: flex-end;
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
	}

	.titlebar-buttons {
		display: flex;
	}

	.titlebar-button {
		display: inline-flex;
		justify-content: center;
		align-items: center;
		width: 30px;
		height: 30px;
	}
	.titlebar-button:hover {
		background: #ffffff23;
	}
</style>
