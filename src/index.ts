import Util from "./Util.js";

async function main() {
	// Load user access and refresh tokens
	await Util.loadCredentials();

	// Update the synced playlist every hour
	Util.updateSyncedPlaylist();
	setInterval(Util.updateSyncedPlaylist, 3_600_000);
}

// Main function
main();
