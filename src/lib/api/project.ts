import type { ProjectPayload } from "$lib/types/project/ProjectTypes";

const projectBase = 'http://localhost:3000/app';

// crud for api naming convention
// c - create
// r - read
// u - update
// d - delete

export async function createProject(projectTitle: ProjectPayload): Promise<Response> {
	const res = await fetch(projectBase + '/project/', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({ projectTitle }),
	});
	return res;
}

export async function readProjectList(): Promise<Response> {
	const res = await fetch(projectBase + '/projects', {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return res;
}

export async function readProject(projectTitle: string): Promise<Response> {
	const res = await fetch(projectBase + '/project/' + projectTitle, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return res;
}

export async function deleteProject(projectTitle: string): Promise<Response> {
	const res = await fetch(projectBase + '/project/' + projectTitle, {
		method: 'DELETE',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return res;
}
