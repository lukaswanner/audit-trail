export type Notification = {
	id: number;
	name: string;
	phoneNumber: string;
	channelId: number;
};

export type NotificationPayload = {
	name: string;
	phoneNumber: string;
	channelId: number;
};

export type UpdateNotificationPayload = {
	name: string;
	phoneNumber: string;
	channelId: number;
};
