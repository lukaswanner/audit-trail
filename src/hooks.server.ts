import type { Handle } from "@sveltejs/kit";
import { jwtDecode } from "jwt-decode";

type UserJwt = {
	account_id: number;
	exp: number;
};

async function getUserInformation(cookie: string | undefined) {
	if (!cookie) return undefined;

	const decoded = jwtDecode<UserJwt>(cookie);

	if (decoded.exp < Date.now() / 1000) return undefined;
	return decoded.account_id;
}

export const handle: Handle = async ({ event, resolve }) => {
	event.locals.account_id = await getUserInformation(event.cookies.get("__audit"));

	return await resolve(event);
};
