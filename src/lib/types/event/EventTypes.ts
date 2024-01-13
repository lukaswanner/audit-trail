// type definitions for event
export type EventPayload = {
	icon: string;
	title: string;
	channelTitle: string;
	actorName: string;
	tags: Record<string, unknown>[];
};

export type Event = {
	id: number;
	icon: string;
	title: string;
	channelTitle: string;
	actorName: string;
	projectId: number;
	ts: string;
	tags: Record<string, unknown>[];
};
