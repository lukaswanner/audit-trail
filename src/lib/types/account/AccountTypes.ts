// type definitions for account
export type UserCredentials = {
	email: string;
	password: string;
};

export type UserCredentialsLogin = UserCredentials & { rememberMe: boolean };
