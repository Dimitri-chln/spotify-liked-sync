interface Config {
	synced_playlist_id: string;
}

interface Credentials {
	created_at: string;
	data: {
		access_token: string;
		token_type: string;
		expires_in: number;
		refresh_token: string;
		scope: string;
	};
}
