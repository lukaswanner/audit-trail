// type definitions for event

export type EventPayload = {
	icon: string;
	title: string;
	channel_title: string;
	user_name: string;
	tags: Record<string, unknown>[];
};

export type Event = {
	id: number;
	icon: string;
	title: string;
	channelTitle: string;
	userName: string;
	projectId: number;
	ts: string;
	tags: Record<string, unknown>[];
};
