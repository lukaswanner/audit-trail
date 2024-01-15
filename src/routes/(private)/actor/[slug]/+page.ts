import { readActor } from "$lib/api/actor"
import type { Actor } from "$lib/types/actor/ActorTypes";
import type { PageLoad } from "./$types"

export const load: PageLoad = async ({ params }) => {
	let error: string = "";
	let actor: Actor | null = null;

	const res = await readActor(params.slug)
	if (!res.ok) {
		error = 'Error loading actor data'
		return {
			actor,
			error,
		}
	}

	try {
		actor = await res.json()
	} catch (e) {
		error = 'Error parsing actor data'
	}

	return {
		actor,
		error,
	}
}
