import { writable, derived } from 'svelte/store';

export const isAllowed = writable(true);
export const isRunning = writable(false);
export const currentTitleName = writable('');
export const previousTitleName = writable('');

// Define the possible states for the timer
export const TimerState = {
	STOPPED: 'STOPPED',
	RUNNING: 'RUNNING',
	PAUSED: 'PAUSED'
};

// A derived store that computes the state based on isAllowed and isRunning
export const state = derived([isAllowed, isRunning], ([$isAllowed, $isRunning]) => {
	if (!$isRunning) {
		return TimerState.STOPPED;
	} else if ($isAllowed) {
		return TimerState.RUNNING;
	} else {
		return TimerState.PAUSED;
	}
});
