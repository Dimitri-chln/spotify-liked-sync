import Util from "../Util.js";

import Fs from "fs";
import authorize from "./authorize.js";
import refreshAccessToken from "./refreshAccessToken.js";

/**
 *
 * @returns The date at which the access token expires
 */
export default async function loadCredentials() {
	console.log("Loading possibly already exisiting credentials...");
	const credentials: Credentials = JSON.parse(Fs.readFileSync("./dist/credentials.json").toString());

	// The time before the current access token expires in ms
	let expiresIn: number;

	// If a token has been generated already
	if (credentials.created_at) {
		console.log(" - An access token has already been generated.");
		Util.spotify.setRefreshToken(credentials.data.refresh_token);

		// If the token is still valid
		if (new Date(credentials.created_at).valueOf() + credentials.data.expires_in * 1000 + 60_000 > Date.now()) {
			Util.spotify.setAccessToken(credentials.data.access_token);
			console.log(" - It is still valid and saved for future API calls.");
			expiresIn = credentials.data.expires_in * 1000 - (Date.now() - new Date(credentials.created_at).valueOf());
		} else {
			// If it expired already
			console.log(" - It has expired, refreshing the access token...");
			const spotifyResponse = await Util.spotify.refreshAccessToken();

			Util.spotify.setAccessToken(spotifyResponse.body.access_token);
			console.log(" - Access token successfully refreshed and saved for future API calls.");
			expiresIn = spotifyResponse.body.expires_in * 1000;
		}
	} else {
		// Otherwise generate new token
		console.log(" - No access token has been found, generating a new one...");
		const newCredentials = await authorize();
		// Save the access token
		Fs.writeFileSync("./dist/credentials.json", JSON.stringify(credentials, null, 2));

		Util.spotify.setAccessToken(newCredentials.data.access_token);
		Util.spotify.setRefreshToken(newCredentials.data.refresh_token);
		console.log(" - Access token and refresh token successfully generated and saved for future API calls.");
		expiresIn = newCredentials.data.expires_in * 1000;
	}

	// Refresh token 1 min before the current one expires
	setTimeout(refreshAccessToken, expiresIn - 60_000);
}
