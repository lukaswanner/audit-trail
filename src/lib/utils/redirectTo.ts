
export function handleLoginRedirect(url: URL, message: string) {
	const redirectTo = url.pathname + url.search;

	return `/login?redirectTo=${redirectTo}&message=${message}`
}
