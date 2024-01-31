import type { NotificationPayload, UpdateNotificationPayload } from "$lib/types/notification/notificationTypes";

const notificationBase = "http://localhost:3000/app";

// crud for api naming convention
// c - create
// r - read
// u - update
// d - delete
//


export async function createNotification(payload: NotificationPayload): Promise<Response> {
	return await fetch(`${notificationBase}/notification`, {
		method: "POST",
		credentials: "include",
		body: JSON.stringify(payload),
		headers: {
			"Content-Type": "application/json"
		}
	});

}

export async function readNotification(id: number): Promise<Response> {
	return await fetch(`${notificationBase}/notification/${id}`, {
		method: "GET",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		}
	});
}

export async function readNotificationList(): Promise<Response> {
	return await fetch(`${notificationBase}/notifications`, {
		method: "GET",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		}
	});
}

export async function updateNotification(id: number, payload: UpdateNotificationPayload): Promise<Response> {
	return await fetch(`${notificationBase}/notification/${id}`, {
		method: "PUT",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		},
		body: JSON.stringify(payload),
	});
}

export async function deleteNotification(id: number): Promise<Response> {
	return await fetch(`${notificationBase}/notification/${id}`, {
		method: "DELETE",
		credentials: "include",
		headers: {
			"Content-Type": "application/json"
		}
	});
}
