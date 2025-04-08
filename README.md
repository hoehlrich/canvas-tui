# canvas-tui

A simple terminal user interface wrapper around the Canvas LMS

### Setup

Create a '.env' file in the root directory of the project and populate with:
```
CANVAS_API_TOKEN = "your_api_key"
```
Currently data storage is only supported on *nix systems (stored in ~/.local/share/canvas-tui/).

### Background

This project originated as a BlasterHacks 2025 project [canvas.lte](https://github.com/hoehlrich/canvas-tui)
programmed in conjunction with [shauryasaxenas](https://github.com/shauryasaxenas). This fork is an attempt to
remedy the various issues with the original project in a smaller scope: just
the Canvas API and TUI (removed web frontend and groq integration). Ideally,
this projects backend connects back to the original project to provide a more
functional web frontend.

### TODO
- [X] Fix TUI async refresh
- [X] Fix data file path
- [X] Remove server feature
- [ ] Make able to mark as completed from interface (make them gray)
- [ ] Refactor to not use graphql_client (reqwest only) (hopefully speed up)
- [ ] Make async refresh only update changed data (to preserve completion status)
- [ ] custom DIR_DIR for data files
- [ ] Canvas assignment link scraping
- [ ] Fix abhorrent load times for grades
- [ ] Fetch external submission links
- [ ] If browser is opened with open::this(url) for the first time, allow the browser to remain open after the program exits
