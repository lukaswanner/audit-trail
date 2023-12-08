import type { UserCredentials } from "$lib/types/account/AccountTypes";


const authBase = 'http://localhost:3000/auth';

export async function login(creds: UserCredentials) {
	const res = await fetch(authBase + '/login', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify(creds),
	});
	return await res.json();
}

export async function register(creds: UserCredentials) {
	const res = await fetch(authBase + '/register', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify(creds),
	});
	return await res.json();
}
