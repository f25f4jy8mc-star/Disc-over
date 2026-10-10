import { useEffect, useState } from "react";
import { LibraryHit, searchLibrary } from "../../lib/api";

const FORMAT_LABEL = { vinyl: "Vinyl", cd: "CD", cassette: "Cassette" } as const;

// One search bar, two sections: "In my library" (offline, on the phone) and
// "On Discogs" (live). The Discogs half arrives with Discogs sign-in.
export function SearchScreen() {
  const [query, setQuery] = useState("");
  const [hits, setHits] = useState<LibraryHit[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let current = true;
    const q = query.trim();
    if (!q) {
      setHits([]);
      return;
    }
    const timer = setTimeout(() => {
      searchLibrary(q)
        .then((h) => current && (setHits(h), setError(null)))
        .catch((e) => current && setError(String(e)));
    }, 150);
    return () => {
      current = false;
      clearTimeout(timer);
    };
  }, [query]);

  return (
    <main className="search">
      <input
        className="search-bar"
        type="search"
        placeholder="Artist, title, track, label, catalog number…"
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        autoFocus
      />
      {error && <p className="error">{error}</p>}

      <section>
        <h2>In my library</h2>
        {query.trim() && hits.length === 0 && <p className="empty">Nothing in your library matches.</p>}
        <ul className="results">
          {hits.map((h) => (
            <li key={`${h.kind}-${h.id}`}>
              <span className="title">
                {h.artist} – {h.title}
              </span>
              <span className="meta">
                {h.year ?? "—"} · {FORMAT_LABEL[h.format]}
                {h.kind === "wishlist" && <span className="badge">Wishlist</span>}
              </span>
            </li>
          ))}
        </ul>
      </section>

      <section>
        <h2>On Discogs</h2>
        <p className="empty">Sign in to Discogs to search the database.</p>
      </section>
    </main>
  );
}
