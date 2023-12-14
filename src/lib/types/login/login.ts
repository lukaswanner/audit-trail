
export type PseudoEvent = {
	icon: string;
	title: string;
	timeStamp: number;
};

export type EventType =
	| 'emailTouched'
	| 'passwordTouched'
	| 'mustLogin'
	| 'rememberMe'
	| 'DontRememberMe'
	| 'loginSuccess'
	| 'loginFailure'
	| 'registerSuccess'
	| 'registerFailure'
	;
