import { readEventListForActor } from "$lib/api/event";
import { readActor } from "$lib/api/actor";
import type { Actor } from "$lib/types/actor/ActorTypes";
import type { PageLoad } from "./$types";
import type { Event } from "$lib/types/event/EventTypes";

async function readEvents(actorId: number) {
	const res = await readEventListForActor(actorId);
	if (!res.ok) {
		return [];
	}
	try {
		const events = await res.json();
		return events.sort(
			(a: Event, b: Event) => new Date(b.ts).getDate() - new Date(a.ts).getDate()
		);
	} catch (e) {
		return [];
	}
}

export const load: PageLoad = async ({ params }) => {
	let error: string = "";
	let actor: Actor | null = null;
	let events: Event[] = [];

	const res = await readActor(params.slug);
	if (!res.ok) {
		error = "Error loading actor data";
		return {
			actor,
			error,
			events
		};
	}

	try {
		actor = await res.json();
		if (!actor) {
			error = "Error loading actor data";
		} else {
			events = await readEvents(actor.id);
		}
	} catch (e) {
		error = "Error parsing actor data";
	}

	return {
		actor,
		error,
		events
	};
};
