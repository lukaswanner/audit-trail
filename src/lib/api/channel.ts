import type { ChannelPayload } from "$lib/types/channel/ChannelTypes";

const channelBase = 'http://localhost:3000/app';

// crud for api naming convention
// c - create
// r - read
// u - update
// d - delete

export async function createChanenl(channelTitle: ChannelPayload): Promise<Response> {
	const res = await fetch(channelBase + '/channel', {
		method: 'POST',
		credentials: "same-origin",
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({ title: channelTitle }),
	});
	return res;
}

export async function readChannelList(): Promise<Response> {
	const res = await fetch(channelBase + '/channels', {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
		},
	});

	return res;
}

export async function readChannel(channelTitle: string): Promise<Response> {
	const res = await fetch(channelBase + '/channel/' + channelTitle, {
		method: 'GET',
		credentials: "same-origin",
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return res;
}

export async function deleteChannel(channelTitle: string): Promise<Response> {
	const res = await fetch(channelBase + '/channel/' + channelTitle, {
		method: 'DELETE',
		credentials: "same-origin",
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return res;
}
