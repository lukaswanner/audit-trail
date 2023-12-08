import type { User, UserPayload } from "$lib/types/user/UserTypes";
const userBase = 'http://localhost:3000/app';

// crud for api naming convention
// c - create
// r - read
// u - update
// d - delete

export async function createUser(user: UserPayload) {
	const res = await fetch(userBase + '/user', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({ user }),
	});
	return await res.json();
}

export async function readUserList(): Promise<User[]> {
	const res = await fetch(userBase + '/users', {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return await res.json();
}

export async function readUser(userName: string): Promise<User> {
	const res = await fetch(userBase + '/user/' + userName, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return await res.json();
}

export async function deleteUser(userName: string) {
	const res = await fetch(userBase + '/user/' + userName, {
		method: 'DELETE',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return await res.json();
}
