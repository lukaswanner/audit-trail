import type { Event, EventPayload } from "$lib/types/event/EventTypes";

const eventBase = 'http://localhost:3000/app';

// crud for api naming convention
// c - create
// r - read
// u - update
// d - delete

export async function createEvent(eventId: EventPayload) {
	const res = await fetch(eventBase + '/event/', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({ eventId }),
	});
	return await res.json();
}

export async function readEventList(): Promise<Event[]> {
	const res = await fetch(eventBase + '/events', {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return await res.json();
}

export async function readEvent(eventId: number): Promise<Event> {
	const res = await fetch(eventBase + '/event/' + eventId, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return await res.json();
}

export async function deleteEvent(eventId: number) {
	const res = await fetch(eventBase + '/event/' + eventId, {
		method: 'DELETE',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return await res.json();
}
