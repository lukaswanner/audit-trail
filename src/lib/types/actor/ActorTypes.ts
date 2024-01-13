// type definitions for user
export type ActorPayload = {
	name: string;
	projectId: number;
	properties: Record<string, unknown>;
};

export type UpdateActorPayload = {
	id: number;
	name: string;
	projectId: number;
	properties: Record<string, unknown>;
};

export type Actor = {
	id: number;
	name: string;
	projectTitle: string;
	properties: Record<string, unknown>;
};

