# canvas-tui

A simple terminal user interface wrapper around the Canvas LMS

### Setup

Env variable setup:
- Create a '.env' file in the root directory of the project
- Create a Canvas API key and export it as `CANVAS_API_KEY` in the `.env` file

### Background

This project originated as a BlasterHacks 2025 project [canvas.lte](https://github.com/hoehlrich/canvas-tui)
programmed in conjunction with [shauryasaxenas](https://github.com/shauryasaxenas). This fork is an attempt to
remedy the various issues with the original project in a smaller scope: just
the Canvas API and TUI (removed web frontend and groq integration). Ideally,
this projects backend connects back to the original project to provide a more
functional web frontend.

### TODO
- [X] Fix TUI async refresh
- [ ] Fix data file path
- [ ] Refactor to not use graphql_client (reqwest only) (hopefully speed up)
- [ ] Fix abhorrent load times for grades
- [ ] Canvas assignment link scraping
- [ ] Actually determine whether assignment is completed
- [ ] Fetch external submission links
- [ ] Make able to mark as completed from interface
