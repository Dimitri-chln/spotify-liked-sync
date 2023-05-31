import Dotenv from "dotenv";
Dotenv.config();

import Fs from "fs";
import SpotifyWebApi from "spotify-web-api-node";
import loadCredentials from "./utils/loadCredentials.js";
import updateSyncedPlaylist from "./utils/updateSyncedPlaylist.js";

export default class Util {
	static readonly spotify = new SpotifyWebApi({
		clientId: process.env.SPOTIFY_CLIENT_ID,
		clientSecret: process.env.SPOTIFY_CLIENT_SECRET,
		redirectUri: process.env.SPOTIFY_REDIRECT_URI,
	});

	static readonly config: Config = JSON.parse(Fs.readFileSync("./dist/config.json").toString());

	static loadCredentials = loadCredentials;
	static updateSyncedPlaylist = updateSyncedPlaylist;
}
