// type definitions for user
export type ActorPayload = {
	name: string;
	projectId: number;
	properties: Map<string, string>;
};

export type UpdateActorPayload = {
	id: number;
	projectId: number;
	name: string;
	properties: Record<string, unknown>;
};

export type Actor = {
	id: number;
	name: string;
	projectTitle: string;
	properties: Record<string, unknown>;
};
