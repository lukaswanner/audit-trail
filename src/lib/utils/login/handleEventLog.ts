import type { EventType, PseudoEvent } from "$lib/types/login/login";

export function addToEventLog(events: PseudoEvent[], event: EventType): PseudoEvent[] {
	let newEvent: PseudoEvent;
	switch (event) {
		case 'emailTouched':
			newEvent = {
				icon: '📫',
				title: 'New Email input',
				timeStamp: Date.now()
			};
			break;
		case 'passwordTouched':
			newEvent = {
				icon: '✍🏼',
				title: 'New Password input',
				timeStamp: Date.now()
			};
			break;
		case 'mustLogin':
			newEvent = {
				icon: '🔒',
				title: 'You must be logged in to view this page',
				timeStamp: Date.now()
			};
			break;
		case 'rememberMe':
			newEvent = {
				icon: '✅',
				title: 'Remember me checked',
				timeStamp: Date.now()
			};
			break;
		case 'DontRememberMe':
			newEvent = {
				icon: '❌',
				title: 'Remember me unchecked',
				timeStamp: Date.now()
			};
			break;
		case 'loginSuccess':
			newEvent = {
				icon: '🚀',
				title: 'Login success',
				timeStamp: Date.now()
			};
			break;
		case 'loginFailure':
			newEvent = {
				icon: '🚨',
				title: 'Login failure',
				timeStamp: Date.now()
			};
			break;
		case 'registerSuccess':
			newEvent = {
				icon: '🚀',
				title: 'Register success',
				timeStamp: Date.now()
			};
			break;
		case 'registerFailure':
			newEvent = {
				icon: '❌',
				title: 'Register failure',
				timeStamp: Date.now()
			};
			break;
	}
	events.push(newEvent);
	return events;
}
