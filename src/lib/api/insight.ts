
import type { InsightPayload } from "$lib/types/insight/Insight";

const insightBase = 'http://localhost:3000/app';
const insightBaseApi = 'http://localhost:3000/api';

// crud for api naming convention
// c - create
// r - read
// u - update
// d - delete

export async function createInsight(insight: InsightPayload): Promise<Response> {
	const res = await fetch(insightBaseApi + '/insight', {
		method: 'POST',
		credentials: "include",
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify(insight),
	});
	return res;
}

export async function readInsightList(projectTitle: string): Promise<Response> {
	const res = await fetch(insightBase + '/insights/' + projectTitle, {
		method: 'GET',
		credentials: "include",
		headers: {
			'Content-Type': 'application/json',
		},
	});

	return res;
}

export async function readInsight(projectTitle: string, insightTitle: string): Promise<Response> {
	const res = await fetch(`${insightBase}/insight/${projectTitle}/${insightTitle}`, {
		method: 'GET',
		credentials: "include",
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return res;
}
