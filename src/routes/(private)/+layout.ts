import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = ({ data, url }) => {
	const accountId = data.accountId;
	const fromUrl = url.pathname + url.search
	if (!accountId) {
		throw redirect(302, `/login?redirectTo=${fromUrl}`);
	}
};
