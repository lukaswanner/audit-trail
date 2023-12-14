import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';
import { handleSuccessfulRedirect } from '$lib/utils/redirectTo';

export const load: LayoutLoad = ({ data, url }) => {
	const accountId = data.accountId;
	if (accountId) {
		throw redirect(302, handleSuccessfulRedirect(url));
	}
};
