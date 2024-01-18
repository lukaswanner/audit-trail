// type definitions for channel
export type ChannelPayload = {
	title: string;
	projectId: number;
};

export type UpdateChannelPayload = {
	id: number;
	title: string;
	projectId: number;
};

export type Channel = {
	id: number;
	title: string;
	projectTitle: string;
};
