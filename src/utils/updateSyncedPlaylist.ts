import loadSavedTracks from "./loadSavedTracks.js";
import clearSyncedPlaylist from "./clearSyncedPlaylist.js";
import fillSyncedPlaylist from "./fillSyncedPlaylist.js";

export default async function updateSyncedPlaylist() {
	console.log("Updating synced playlist...");
	console.log(" - Loading saved tracks...");
	const savedTracks = await loadSavedTracks();
	console.log(" - Clearing the synced playlist...");
	await clearSyncedPlaylist();
	console.log(" - Adding the saved tracks back...");
	await fillSyncedPlaylist(savedTracks.map((savedTrack) => savedTrack.track.uri));
	console.log("Synced playlist successfully updated");
}
