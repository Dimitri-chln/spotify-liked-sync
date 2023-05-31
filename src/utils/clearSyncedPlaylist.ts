import Util from "../Util.js";

export default async function clearSyncedPlaylist() {
	const spotifyResult = await Util.spotify.getPlaylist(Util.config.synced_playlist_id);
	if (spotifyResult.body.tracks.total === 0) return;

	const positions = Array.from(Array(spotifyResult.body.tracks.total).keys());

	await Util.spotify.removeTracksFromPlaylistByPosition(
		Util.config.synced_playlist_id,
		positions,
		spotifyResult.body.snapshot_id,
	);
}
