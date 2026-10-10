// Typed wrappers over the Tauri commands in src-tauri/src/commands.rs.
// Shapes mirror discover-core's serde types.
import { invoke } from "@tauri-apps/api/core";

export type Grade = "M" | "NM" | "VG+" | "VG" | "G+" | "G" | "F" | "P";
export type Format = "vinyl" | "cd" | "cassette";

export interface LibraryHit {
  kind: "copy" | "wishlist";
  id: string;
  release_id: string;
  artist: string;
  title: string;
  year: number | null;
  format: Format;
}

export interface NewRelease {
  discogs_release_id: number | null;
  discogs_master_id: number | null;
  artist: string;
  title: string;
  year: number | null;
  label: string | null;
  catalog_no: string | null;
  format: Format;
  format_detail: string | null;
  country: string | null;
  barcode: string | null;
  tracks: [position: string, title: string, durationSeconds: number | null][];
  genres: string[];
  styles: string[];
}

export interface NewCopy {
  media_condition: Grade | null;
  sleeve_condition: Grade | null;
  price_paid: number | null;
  currency: string | null;
  purchased_on: string | null;
  location: string | null;
  notes: string | null;
}

export const searchLibrary = (query: string) =>
  invoke<LibraryHit[]>("search_library", { query });

export const addCopy = (release: NewRelease, copy: NewCopy) =>
  invoke<string>("add_copy", { release, copy });

export const addToWishlist = (release: NewRelease, notes: string | null) =>
  invoke<string>("add_to_wishlist", { release, notes });
