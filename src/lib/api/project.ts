import type { Project, ProjectPayload } from "$lib/types/project/ProjectTypes";

const projectBase = 'http://localhost:3000/app';

// crud for api naming convention
// c - create
// r - read
// u - update
// d - delete

export async function createProject(projectTitle: ProjectPayload) {
	const res = await fetch(projectBase + '/project/', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({ projectTitle }),
	});
	return await res.json();
}

export async function readProjectList(): Promise<Project[]> {
	const res = await fetch(projectBase + '/projects', {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return await res.json();
}

export async function readProject(projectTitle: string): Promise<Project> {
	const res = await fetch(projectBase + '/project/' + projectTitle, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return await res.json();
}

export async function deleteProject(projectTitle: string) {
	const res = await fetch(projectBase + '/project/' + projectTitle, {
		method: 'DELETE',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return await res.json();
}
