import type { UserPayload } from "$lib/types/user/UserTypes";
const userBase = 'http://localhost:3000/app';

// crud for api naming convention
// c - create
// r - read
// u - update
// d - delete

export async function createUser(user: UserPayload): Promise<Response> {
	const res = await fetch(userBase + '/user', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({ user }),
	});
	return res;
}

export async function readUserList(): Promise<Response> {
	const res = await fetch(userBase + '/users', {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return res;
}

export async function readUser(userName: string): Promise<Response> {
	const res = await fetch(userBase + '/user/' + userName, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return res;
}

export async function deleteUser(userName: string): Promise<Response> {
	const res = await fetch(userBase + '/user/' + userName, {
		method: 'DELETE',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return res;
}
