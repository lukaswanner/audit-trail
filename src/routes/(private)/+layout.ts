import { redirect } from "@sveltejs/kit";
import type { LayoutLoad } from "./$types";
import { handleLoginRedirect } from "$lib/utils/redirectTo";

export const load: LayoutLoad = ({ data, url }) => {
	const accountId = data.accountId;
	if (!accountId) {
		redirect(302, handleLoginRedirect(url, "You must be logged in to view this page"));
	}
};
