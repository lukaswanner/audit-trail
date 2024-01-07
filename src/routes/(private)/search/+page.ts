
import type { PageLoad } from "./$types"

export const load: PageLoad = ({ url }) => {
	const key = url.searchParams.get("key");
	const value = url.searchParams.get("value");
	return {
		key,
		value
	}
}
