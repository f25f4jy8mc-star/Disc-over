import { describe, expect, it } from "vitest";
import { linkOutSearches } from "./linkOut";

describe("linkOutSearches", () => {
  it("builds one search link per service, encoded", () => {
    const links = linkOutSearches("Simon & Garfunkel", "  Bookends ");
    expect(links.map((l) => l.service)).toEqual(["apple_music", "youtube_music", "bandcamp"]);
    expect(links[0].url).toBe("https://music.apple.com/search?term=Simon%20%26%20Garfunkel%20Bookends");
  });
});
