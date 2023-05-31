import Util from "../Util.js";

export default async function loadSavedTracks() {
	const savedTracks: SpotifyApi.SavedTrackObject[] = [];

	let spotifyResult = await Util.spotify.getMySavedTracks();
	savedTracks.push(...spotifyResult.body.items);

	while (spotifyResult.body.next) {
		let [, offset, limit] = spotifyResult.body.next.match(/offset=(\d+)&limit=(\d+)/).map((a) => parseInt(a));
		spotifyResult = await Util.spotify.getMySavedTracks({ offset, limit });
		savedTracks.push(...spotifyResult.body.items);
	}

	return savedTracks;
}
