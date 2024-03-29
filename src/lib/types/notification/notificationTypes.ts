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

export type CreateNotificationPayload = {
	name: string;
  phoneNumber: string;
	channelId: number;
};

export type UpdateNotificationPayload = {
  id: number;
	name: string;
	channelId: number;
};
