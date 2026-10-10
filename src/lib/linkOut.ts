// Playback rule 3 (spec: "Playback rules"): when neither Spotify nor a
// Discogs video is available, offer search links. No API is used.

export type LinkOutService = "apple_music" | "youtube_music" | "bandcamp";

export interface LinkOut {
  service: LinkOutService;
  label: string;
  url: string;
}

export function linkOutSearches(artist: string, title: string): LinkOut[] {
  const q = encodeURIComponent(`${artist} ${title}`.replace(/\s+/g, " ").trim());
  return [
    { service: "apple_music", label: "Apple Music", url: `https://music.apple.com/search?term=${q}` },
    { service: "youtube_music", label: "YouTube Music", url: `https://music.youtube.com/search?q=${q}` },
    { service: "bandcamp", label: "Bandcamp", url: `https://bandcamp.com/search?q=${q}` },
  ];
}
