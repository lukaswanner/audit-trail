const apikeyBase = "http://localhost:3000/app";

export async function createApikey(projectId:number) {
	const res = await fetch(`${apikeyBase}/api-token`, {
		method: "POST",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		},
		body: JSON.stringify({projectId})
	});
	return res;
}

export async function readApikeyList() {
	const res = await fetch(`${apikeyBase}/api-tokens`, {
		method: "GET",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		},
	});
	return res;
}

export async function deleteApikey(id: number) {
	const res = await fetch(`${apikeyBase}/api-token/${id}`, {
		method: "DELETE",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		},
	});
	return res;
}
