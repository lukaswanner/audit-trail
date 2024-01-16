import type { ProjectPayload, UpdateProjectPayload } from "$lib/types/project/ProjectTypes";

const projectBase = "http://localhost:3000/app";

// crud for api naming convention
// c - create
// r - read
// u - update
// d - delete

export async function createProject(projectTitle: ProjectPayload): Promise<Response> {
	const res = await fetch(`${projectBase}/project`, {
		method: "POST",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		},
		body: JSON.stringify(projectTitle)
	});
	return res;
}

export async function updateProject(updatedProject: UpdateProjectPayload): Promise<Response> {
	console.log(updatedProject);
	const res = await fetch(`${projectBase}/project`, {
		method: "PATCH",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		},
		body: JSON.stringify(updatedProject)
	});
	return res;
}

export async function readProjectList(): Promise<Response> {
	const res = await fetch(`${projectBase}/projects`, {
		method: "GET",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		}
	});
	return res;
}

export async function readProject(projectId: number): Promise<Response> {
	const res = await fetch(`${projectBase}/projects/${projectId}`, {
		method: "GET",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		}
	});
	return res;
}

export async function deleteProject(projectId: number): Promise<Response> {
	const res = await fetch(`${projectBase}/project/${projectId}`, {
		method: "DELETE",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		}
	});
	return res;
}
