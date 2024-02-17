import Util from "../Util.js";

import express from "express";
import open from "open";

import generateRandomString from "./generateRandomString.js";

export default function authorize(): Promise<Credentials> {
	return new Promise(async (resolve, reject) => {
		const app = express();

		// Start local web server
		const server = app.listen(8080);

		app.get("/", async (req, res) => {
			res.send("Hello");
		});

		// When the user authorized the app
		app.get("/callback", async (req, res) => {
			if (req.query.state !== state) return res.send("Error");
			res.send("<body onload='javascript:close();'></body>");

			// Get an access token
			const spotifyResponse = await Util.spotify.authorizationCodeGrant(req.query.code as string);

			const credentials: Credentials = {
				created_at: new Date().toISOString(),
				data: spotifyResponse.body,
			};

			// Close the web server and return the access token
			server.close();
			resolve(credentials);
		});

		// Open authorization window in browser
		const state = generateRandomString(32);
		const url = Util.spotify.createAuthorizeURL(
			["user-library-read", "playlist-modify-public", "playlist-read-private", "playlist-modify-private"],
			state,
		);

		console.log(`- Login to Spotify to ${url}`);
		open(url).catch(console.error);
	});
}
