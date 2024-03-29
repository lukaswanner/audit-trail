import type { UserCredentials } from "$lib/types/account/AccountTypes";

const authBase = "http://localhost:3000/auth";

export async function logout() {
	const res = await fetch(`${authBase}/logout`, {
		method: "POST",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		},
	});
	return res;
}

export async function login(creds: UserCredentials) {
	const res = await fetch(`${authBase}/login`, {
		method: "POST",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		},
		body: JSON.stringify(creds)
	});
	return res;
}

export async function register(creds: UserCredentials) {
	const res = await fetch(`${authBase}/register`, {
		method: "POST",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		},
		body: JSON.stringify(creds)
	});
	return res;
}
