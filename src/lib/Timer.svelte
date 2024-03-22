<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { writable } from 'svelte/store';
	import { isRunning, state, TimerState } from './timerStore';
	import { ask } from '@tauri-apps/api/dialog';

	let intervalId: ReturnType<typeof setInterval>;
	const time = writable(0);

	state.subscribe((value) => {
		if (value == TimerState.STOPPED || value == TimerState.PAUSED) {
			pauseTimer();
		} else {
			startTimer();
		}
	});

	function formatTime(seconds: number): string {
		const hours = Math.floor(seconds / 3600);
		const minutes = Math.floor((seconds % 3600) / 60);
		const secs = seconds % 60;

		return [hours, minutes, secs].map((v) => (v < 10 ? '0' + v : v)).join(':');
	}

	function startTimer() {
		intervalId = setInterval(() => {
			time.update((n) => n + 1);
		}, 1000);
	}

	function pauseTimer() {
		clearInterval(intervalId);
	}

	async function tryReset(): Promise<void> {
		const isReset = await ask('Reset?');
		if (isReset) {
			resetTimer();
		}
	}

	const toggleTimer = () => {
		$isRunning = !$isRunning;
	};

	const resetTimer = () => {
		time.set(0);
		$isRunning = false;
	};

	onMount(() => {
		window.addEventListener('click', toggleTimer);
		return () => {
			window.removeEventListener('click', toggleTimer);
		};
	});
</script>

<div
	class="timer"
	style="background-color: {$state === TimerState.RUNNING
		? 'var(--running-color)'
		: $state === TimerState.STOPPED
			? 'var(--stopped-color)'
			: 'var(--paused-color)'}"
>
	{formatTime($time)}
</div>

<div class="reset-button" on:click|stopPropagation={tryReset}>Reset</div>

<style>
	:global(html, body) {
		height: 100%;
		margin: 0;
		overflow: hidden;
	}

	:root {
		--running-color: rgba(0, 128, 0, 0.5);
		--stopped-color: rgba(255, 0, 0, 0.5);
		--paused-color: rgba(255, 255, 0, 0.5);
	}

	.timer {
		display: flex;
		flex-direction: column;
		height: 100%;
		justify-content: center;
		align-items: center;
		font-size: 15vw;
		user-select: none;
		transition: background-color 0.3s;
		font-family: 'Digital-7 Mono', monospace;
	}

	.reset-button {
		position: absolute;
		bottom: 0;
		left: 0;
		width: 100%;
		padding: 10px 0;
		font-size: 1rem;
		text-align: center;
		cursor: pointer;
		background-color: rgba(0, 0, 0, 0.1);
		color: white;
	}
</style>
