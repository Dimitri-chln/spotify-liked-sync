import Util from "../Util.js";

export default async function fillSyncedPlaylist(tracks: string[]) {
	while (tracks.length > 0) {
		const tracksToAdd = tracks.splice(0, 100);
		await Util.spotify.addTracksToPlaylist(Util.config.synced_playlist_id, tracksToAdd);
	}
}
