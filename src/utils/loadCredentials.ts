import Util from "../Util.js";

import Fs from "node:fs";
import FsAsync from "node:fs/promises";

import authorize from "./authorize.js";

export default async function loadCredentials() {
	console.log("Loading possibly already exisiting credentials...");

	// If a token has been generated already
	if (Fs.existsSync("credentials.json")) {
		const credentials: Credentials = JSON.parse(await FsAsync.readFile("credentials.json", { encoding: "utf8" }));

		console.log(" - An access token has already been generated.");
		Util.spotify.setRefreshToken(credentials.data.refresh_token);

		// If the token is still valid
		const expiresAt = new Date(credentials.created_at).valueOf() + credentials.data.expires_in * 1000;
		if (expiresAt + 60_000 > Date.now()) {
			Util.spotify.setAccessToken(credentials.data.access_token);
			console.log(" - It is still valid and saved for future API calls.");
		} else {
			// If it expired already
			console.log(" - It has expired, refreshing the access token...");
			const spotifyResponse = await Util.spotify.refreshAccessToken();

			Util.spotify.setAccessToken(spotifyResponse.body.access_token);
			console.log(" - Access token successfully refreshed and saved for future API calls.");
		}
	} else {
		// Otherwise generate new token
		console.log(" - No access token has been found, generating a new one...");
		const newCredentials = await authorize();
		// Save the access token
		await FsAsync.writeFile("credentials.json", JSON.stringify(newCredentials, null, 2));

		Util.spotify.setAccessToken(newCredentials.data.access_token);
		Util.spotify.setRefreshToken(newCredentials.data.refresh_token);
		console.log(" - Access token and refresh token successfully generated and saved for future API calls.");
	}
}
