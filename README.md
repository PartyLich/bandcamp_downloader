A port of [Otiel/BandcampDownloader](https://github.com/Otiel/BandcampDownloader) in Rust with
multiplatform support.

There are a quite a few options for this seemingly specific niche, but the one I prefer is Windows
only. So I decided to implement it for the platform I spend most of my time on (Linux) using a
language I'm building familiarity with.

The gui is currently implemented using the [iced](https://github.com/hecrj/iced) framework. In my
initial exploration I neglected to check for the existence of a *multiline* text input. For the sake
of expedience we currently have a single line input with the ability to add/remove multiple urls,
one at a time.

Still working toward feature parity. Missing/planned features are being added to issues. If you opt
to use this implementation and there's a missing feature that is important to you, please open an
issue!

Docs (for the code, rather than user-centric manuals) can be built with standard cargo commands.
They are not currently hosted online. Eventually I'll add that to the build pipeline.

### Features

Download individual albums/tracks from a bandcamp url (eg https://artist.bandcamp.com/album/album-name).
Download entire artist discography from a bandcamp url.

File/path name formats allow the following placeholders. The file written to disk will have the
appropriate substitution made:
- `{album}`
- `{year}`: album release year
- `{month}`: album release month
- `{day}`: album release day
- `{album}`: album title
- `{artist}`: album/track artist
- For individual tracks only (ie not cover art, playlists, album directories)
  - `{title}`: track title
  - `{tracknum}`: track number

Configurable id3 tagging
Playlist creation
