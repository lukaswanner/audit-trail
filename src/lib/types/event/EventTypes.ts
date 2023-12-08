// type definitions for event
export type EventPayload = {
	icon: string;
	title: string;
	channel_title: string;
	user_name: string;
};

export type Event = {
	id: number;
	icon: string;
	title: string;
	channel_title: string;
	user_name: string;
	ts: string;
};
