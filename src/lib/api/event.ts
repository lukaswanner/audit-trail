import type { EventPayload } from "$lib/types/event/EventTypes";

const eventBase = 'http://localhost:3000/app';

// crud for api naming convention
// c - create
// r - read
// u - update
// d - delete

export async function createEvent(eventId: EventPayload): Promise<Response> {
	const res = await fetch(`${eventBase}/event/`, {
		method: 'POST',
		credentials: "include",
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({ eventId }),
	});
	return res;
}

export async function readEventList(): Promise<Response> {
	const res = await fetch(`${eventBase}/events`, {
		method: 'GET',
		credentials: "include",
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return res
}

export async function readEventListWithMatchingTags(key: string, value: string): Promise<Response> {
	const res = await fetch(`${eventBase}/search?key=${key}&value=${value}`, {
		method: 'GET',
		credentials: "include",
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return res
}

export async function readEventListForChannel(channelTitle: string): Promise<Response> {
	const res = await fetch(`${eventBase}/events/${channelTitle}`, {
		method: 'GET',
		credentials: "include",
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return res
}

export async function readEvent(eventId: number): Promise<Response> {
	const res = await fetch(`${eventBase}/event/${eventId}`, {
		method: 'GET',
		credentials: "include",
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return res;
}

export async function deleteEvent(eventId: number): Promise<Response> {
	const res = await fetch(eventBase + '/event/' + eventId, {
		method: 'DELETE',
		credentials: "include",
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return res;
}
