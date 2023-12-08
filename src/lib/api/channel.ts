import type { Channel, ChannelPayload } from "$lib/types/channel/ChannelTypes";

const channelBase = 'http://localhost:3000/app';

// crud for api naming convention
// c - create
// r - read
// u - update
// d - delete

export async function createChanenl(channelTitle: ChannelPayload) {
	const res = await fetch(channelBase + '/channel', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({ title: channelTitle }),
	});
	return await res.json();
}

export async function readChannelList(): Promise<Channel[]> {
	const res = await fetch(channelBase + '/channels', {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return await res.json();
}

export async function readChannel(channelTitle: string): Promise<Channel> {
	const res = await fetch(channelBase + '/channel/' + channelTitle, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return await res.json();
}

export async function deleteChannel(channelTitle: string) {
	const res = await fetch(channelBase + '/channel/' + channelTitle, {
		method: 'DELETE',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return await res.json();
}
