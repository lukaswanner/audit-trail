const userBase = "http://localhost:3000/app";

// crud for api naming convention
// c - create
// r - read
// u - update
// d - delete
//

export async function readUser(): Promise<Response> {
	const res = await fetch(`${userBase}/user`, {
		method: "GET",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		}
	});

	return res;
}

export async function deleteUser(): Promise<Response> {
	const res = await fetch(`${userBase}/user`, {
		method: "DELETE",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		}
	});

	return res;
}
