# canvas-tui

A simple terminal user interface wrapper around the Canvas LMS

### Setup

Create a '.env' file in the root directory of the project and populate with:
```
CANVAS_API_TOKEN = "your_api_key"
```

This project will respect Canvas' course nicknames and it is HIGHLY recommended
to nickname your courses in canvas to something short (ex: "CALCULUS FOR
SCIENTISTS AND ENGINEERS III" -> "MATH 213"). This can be done by going to the
canvas dashboard in "Card View" and modifying the course nickname in the card
properties.

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
- [X] Make able to mark as completed from interface (make them gray)
- [ ] Make items that have been submitted (as defined by the API) gray (instead of disappearing)
- [ ] Auto-refresh on startup
- [ ] Refactor to not use graphql_client (reqwest only) (hopefully speed up)
- [X] Make async refresh only update changed data (to preserve completion status)
- [ ] Canvas assignment link scraping
- [ ] Course ID config
- [ ] Query Course IDs if no config
- [ ] Filter assignments by course (new pane idea??)
- [ ] custom DIR_DIR for data files
- [ ] Fix abhorrent load times for grades
- [ ] Fetch external submission links
- [ ] If browser is opened with open::this(url) for the first time, allow the browser to remain open after the program exits
