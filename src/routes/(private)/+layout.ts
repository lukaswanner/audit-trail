import { readChannelList } from "$lib/api/channel"
import { readProjectList } from "$lib/api/project";
import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = async () => {
	const readChannels = async () => {
		const channelRes = await readChannelList();
		if (channelRes.status === 200) {
			return await channelRes.json();
		} else {
			return [];
		}
	}

	const readProjects = async () => {
		const projectRes = await readProjectList();
		try {
			if (projectRes.status === 200) {
				return await projectRes.json();
			} else {
				return [];
			}
		} catch (e) {
			console.error(e);
			return [];
		}
	}

	return {
		channels: await readChannels(),
		projects: await readProjects(),
	}
}
