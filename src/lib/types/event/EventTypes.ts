// type definitions for event
export type Event = {
	id: number;
	icon: string;
	title: string;
	description: string;
	channelTitle: string;
	actorName: string;
	projectId: number;
	ts: string;
	tags: Record<string, unknown>[];
};
