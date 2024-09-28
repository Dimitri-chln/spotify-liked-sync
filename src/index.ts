import "dotenv/config";

import Util from "./Util.js";

async function main() {
	// Load user access and refresh tokens
	await Util.loadCredentials();
	// Update the synced playlist
	Util.updateSyncedPlaylist();
}

// Main function
main();
