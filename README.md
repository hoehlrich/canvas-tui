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

NOTE: course-ids are defined in ~/.local/share/canvas-tui/data.json

### Keybidings

```
j -> move assignment selection down
k -> move assignment selection up
d -> toggle competion status of selected assignment
o -> open assignment url in browser
q -> quit
r -> force refresh
n -> enter new assignment mode
    a-z -> fill out text field
    tab -> go to next field
    esc/C-c > exit new assignment mode
    j/k -> when due date selected decrement/increment date
x -> delete custom assignment
J -> move link selection down
K -> move link selection up
O -> open link in browser
```

### Background

This project originated as a BlasterHacks 2025 project [canvas.lte](https://github.com/shauryasaxenas/Canvas.LTE_demo)
programmed in conjunction with [shauryasaxenas](https://github.com/shauryasaxenas). This fork is an attempt to
remedy the various issues with the original project in a smaller scope: just
the Canvas API and TUI (removed web frontend and groq integration). Ideally,
this project's backend connects back to the original project to provide a more
functional web frontend.

### TODO
- [X] Fix TUI async refresh
- [X] Fix data file path
- [X] Remove server feature
- [X] Make able to mark as completed from interface (make them gray)
- [X] Make items that have been submitted (as defined by the API) gray (instead of disappearing)
- [X] Auto-refresh on startup
- [X] Make async refresh only update changed data (to preserve completion status)
- [X] Make # of upcoming assignments respect completed assignments
- [X] Prevent scroll up down wraparound
- [X] Fix issue: can only refresh when run using 'cargo r'
- [X] Refactor to not use graphql_client (didn't do because serialization would be a pain in the ass)
- [X] Errors pop up in box instead of randomly printing
- [X] Remove whitespace
- [X] Make highlight color yellow when editing
- [X] Be able to delete assignments (only the custom ones) by pressing 'x'
- [X] Be able to type capital letters and shift tab in new assignment mode
- [X] Be able to create custom assignemtns by pressing 'n'
- [x] Don't overwrite user set completion status
- [X] Indicate which field is selected when creating writing custom assignments
- [x] Be able to change the day with j and k when editing the DueDate field (don't allow it to go before today)
- [X] Add a lock symbol to locked assignments
- [X] Update logic for locked assignments that become unlocked
- [X] Grade fetching doesn't work (figured it out: use REST)
- [X] **Migrate to the maintained ratatui**
- [X] Canvas assignment link scraping
- [X] Reinstate links pane
- [X] Be able to open links in the link pane
- [X] Write data to file after every modification to state
- [X] Marking assignments as complete moves them to the bottom of their day in the order
- [X] Make index stay on the same assignment if refresh occurs and changes indeces
- [ ] FEATURE: notes section of the "Assignment Summary" pane. Can be edited by pressing 'i'
- [ ] Rework assignment query to use REST instead of graphql
- [ ] Config file (fields: course_ids, data_dir (optional), config_dir (optional))
- [ ] Download pdf files and open in zathura, courses should have a download directory for attachments specified by config file
- [ ] When opening pdf file that's already downloaded check to see if its been modified and update it if so
- [ ] ISSUE: read write precedence with multiple instances working on the same data
- [ ] Custom DIR for data files
- [ ] Fetch external submission links (try REST query after its been migrated)
- [ ] If browser is opened with open::this(url) for the first time, allow the browser to remain open after the program exits
