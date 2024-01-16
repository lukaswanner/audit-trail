export function handleLoginRedirect(url: URL, message: string) {
	const redirectTo = url.pathname + url.search;

	return `/login?redirectTo=${redirectTo}&message=${message}`;
}

export function handleSuccessfulRedirect(url: URL): string {
	const recirectTo = url.searchParams.get("redirectTo") || "/";
	return recirectTo;
}
