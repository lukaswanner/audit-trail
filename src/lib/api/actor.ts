import type { ActorPayload, UpdateActorPayload } from "$lib/types/actor/ActorTypes";
const actorBase = "http://localhost:3000/app";

// crud for api naming convention
// c - create
// r - read
// u - update
// d - delete

export async function createActor(actor: ActorPayload): Promise<Response> {
	const res = await fetch(`${actorBase}/actor`, {
		method: "POST",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		},
		body: JSON.stringify(actor)
	});
	return res;
}

export async function updateActor(actor: UpdateActorPayload): Promise<Response> {
	const res = await fetch(`${actorBase}/actor`, {
		method: "PATCH",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		},
		body: JSON.stringify(actor)
	});
	return res;
}

export async function readActorList(): Promise<Response> {
	const res = await fetch(`${actorBase}/actors`, {
		method: "GET",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		}
	});
	return res;
}

export async function readActorListForProject(id: number): Promise<Response> {
	const res = await fetch(`${actorBase}/actors/${id}`, {
		method: "GET",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		}
	});
	return res;
}

export async function readActor(actorName: string): Promise<Response> {
	const res = await fetch(`${actorBase}/actor/${actorName}`, {
		method: "GET",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		}
	});
	return res;
}

export async function deleteActor(actorId: number): Promise<Response> {
	const res = await fetch(`${actorBase}/actor/${actorId}`, {
		method: "DELETE",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		}
	});
	return res;
}
