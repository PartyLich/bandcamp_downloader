A port of [Otiel/BandcampDownloader](https://github.com/Otiel/BandcampDownloader) in Rust with
multiplatform support.

There are a quite a few options for this seemingly specific niche, but the one I prefer is Windows
only. So I decided to implement it for the platform I spend most of my time on (Linux) using a
language I'm building familiarity with.

The gui is currently implemented using the [iced](https://github.com/hecrj/iced) framework. In my
initial exploration I neglected to check for the existence of a *multiline* text input. For the sake
of expedience we currently have a single line input with the ability to add/remove multiple urls,
one at a time.

Still working toward feature parity.

**TODO** (unordered)
- task cancellation
- full user settings support
  - settings screen in the iced gui
- TUI. i wouldnt want to force anyone to leave the comfort of their terminal
- proxy support
- cover art. everything i've tested on already has cover art included by default, but it is a
  feature of the original application
- additional language support. english is here, spanish is coming, others tbd (and will likely
  require assistance)
