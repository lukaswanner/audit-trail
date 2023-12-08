// type definitions for user
export type UserPayload = {
	name: string;
	project_title: string;
	properties: Record<string, unknown>;
};

export type User = {
	id: number;
	name: string;
	project_title: string;
	properties: Record<string, unknown>;
};

