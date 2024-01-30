const eventBase = "http://localhost:3000/app";

// crud for api naming convention
// c - create
// r - read
// u - update
// d - delete

export async function readEventList(): Promise<Response> {
	const res = await fetch(`${eventBase}/events`, {
		method: "GET",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		}
	});
	return res;
}

export async function readEventListWithMatchingTags(key: string, value: string): Promise<Response> {
	const res = await fetch(`${eventBase}/search?key=${key}&value=${value}`, {
		method: "GET",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		}
	});
	return res;
}

export async function readEventListForChannel(channelId: number): Promise<Response> {
	const res = await fetch(`${eventBase}/events/channel/${channelId}`, {
		method: "GET",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		}
	});
	return res;
}

export async function readEventListForActor(actorId: number): Promise<Response> {
	const res = await fetch(`${eventBase}/events/actor/${actorId}`, {
		method: "GET",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		}
	});
	return res;
}

export async function readEvent(eventId: number): Promise<Response> {
	const res = await fetch(`${eventBase}/event/${eventId}`, {
		method: "GET",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		}
	});
	return res;
}

export async function deleteEvent(eventId: number): Promise<Response> {
	const res = await fetch(eventBase + "/event/" + eventId, {
		method: "DELETE",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		}
	});
	return res;
}
