import type { ChannelPayload } from "$lib/types/channel/ChannelTypes";

const channelBase = 'http://localhost:3000/app';

// crud for api naming convention
// c - create
// r - read
// u - update
// d - delete

export async function createChannel(channel: ChannelPayload): Promise<Response> {
	const res = await fetch(`${channelBase}/channel`, {
		method: 'POST',
		credentials: "include",
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify(channel),
	});
	return res;
}

export async function readChannelList(): Promise<Response> {
	const res = await fetch(`${channelBase}/channels`, {
		method: 'GET',
		credentials: "include",
		headers: {
			'Content-Type': 'application/json',
		},
	});

	return res;
}

export async function readChannelListForProject(id: number): Promise<Response> {
	const res = await fetch(`${channelBase}/channels/${id}`, {
		method: 'GET',
		credentials: "include",
		headers: {
			'Content-Type': 'application/json',
		},
	});

	return res;
}

export async function readChannel(channelId: number): Promise<Response> {
	const res = await fetch(`${channelBase}/channel/${channelId}`, {
		method: 'GET',
		credentials: "include",
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return res;
}

export async function deleteChannel(channelId: number): Promise<Response> {
	const res = await fetch(`${channelBase}/channel/${channelId}`, {
		method: 'DELETE',
		credentials: "include",
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return res;
}
