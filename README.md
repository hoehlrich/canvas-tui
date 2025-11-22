# canvas-tui

A simple terminal user interface wrapper around the Canvas LMS

## Project Setup and Configuration

This guide will walk you through the necessary steps to configure the `canvas-tui` application.

### 1. Create the Configuration Directory

First, create the main configuration folder in your home directory:

```bash
mkdir -p ~/.config/canvas-tui/
```

### 2. Set Your Canvas API Token (Secure File)

You need to provide your Canvas API token to allow the application to access your courses. This is stored in a secure environment file.

* Create the environment file:
    ```bash
    touch ~/.config/canvas-tui/.env
    ```
* Populate the file with your API key, replacing `"your_api_key"` with your actual token:
    ```
    # ~/.config/canvas-tui/.env
    CANVAS_API_TOKEN = "your_api_key"
    ```

> ### Where to Find Your Canvas API Token
> 1.  Log into your Canvas account in a web browser.
> 2.  Navigate to **Account** (in the global navigation menu).
> 3.  Click **Settings**.
> 4.  Scroll down to the **Approved Integrations** section.
> 5.  Click the **"New Access Token"** button.
> 6.  Give your token a **Purpose** (e.g., "canvas-tui") and click **"Generate Token"**.
> 7.  **IMPORTANT:** Copy the generated token immediately—it will **only be displayed once!**

### 3. Configure Course IDs (Config File)

Next, specify which courses you want the application to display by listing their unique Canvas IDs in the configuration file.

* Create the configuration file:
    ```bash
    touch ~/.config/canvas-tui/config.toml
    ```
* Populate the file with a list of your desired course IDs:
    ```toml
    # ~/.config/canvas-tui/config.toml
    course_ids = [12345, 54321, 67890]
    ```

> **How to Find a Course ID**
> A course's ID is the 5-to-6-digit number found at the very end of the course's URL in your web browser when viewing the course page.
>
> **Example URL:** `https://elearning.mines.edu/courses/**75156**`

### Nickname Your Courses

For a much cleaner interface, it is **highly recommended** that you use **Canvas Course Nicknames**. The `canvas-tui` project respects these nicknames.

* **Original Name (Messy):** `"PARTIAL DIFFERENTIAL EQUATIONS-Fall 2025-MATH455A"`
* **Recommended Nickname (Clean):** `"MATH 455"`

#### How to set a Course Nickname:
1.  Navigate to your **Canvas Dashboard**.
2.  Ensure you are in **Card View**.
3.  Click the three-dot menu on the desired course card.
4.  Select **"Settings"** (or **"Card Properties"**) and modify the **Nickname** field.

---

## Keybindings and Navigation

This section outlines the keyboard shortcuts for navigating the TUI and interacting with your Canvas data.

### Core Navigation and Actions

| Key | Action | Description |
| :--- | :--- | :--- |
| **j** | Move Down | Select the **next** assignment in the list. |
| **k** | Move Up | Select the **previous** assignment in the list. |
| **d** | Toggle Status | Toggles the completion status (done/not done) of the selected assignment. |
| **o** | Open in Browser | Opens the selected assignment's URL in your default web browser. |
| **r** | Force Refresh | Forces the application to refresh and fetch the latest data from Canvas. |
| **q** | Quit | Exits the application. |

### Links Panel Navigation

| Key | Action | Description |
| :--- | :--- | :--- |
| **J** | Move Link Down | Select the **next** link in the links panel. |
| **K** | Move Link Up | Select the **previous** link in the links panel. |
| **O** | Open Link | Opens the selected link (e.g., Course Home) in your default web browser. |

### Custom Assignment Management

| Key | Action | Description |
| :--- | :--- | :--- |
| **x** | Delete | Permanently deletes the selected **custom** (non-Canvas) assignment. |
| **n** | Enter New Mode | Enters the **New Assignment** creation mode. |

> **New Assignment Mode Bindings (`n`):**
> * **a-z (Typing):** Fills out the currently selected text field.
> * **Tab:** Moves to the next input field.
> * **j / k:** When the due date field is selected, decrements/increments the date.
> * **Esc / Ctrl+c:** Exits the new assignment creation mode

--

## Background

This project originated as a BlasterHacks 2025 project [canvas.lte](https://github.com/shauryasaxenas/Canvas.LTE_demo)
programmed in conjunction with [shauryasaxenas](https://github.com/shauryasaxenas). This fork is an attempt to
remedy the various issues with the original project in a smaller scope: just
the Canvas API and TUI (removed web frontend and groq integration). Ideally,
this project's backend connects back to the original project to provide a more
functional web frontend.

--

## TODO
- [ ] FEATURE: notes section of the "Assignment Summary" pane. Can be edited by pressing 'i'
- [ ] Rework assignment query to use REST instead of graphql
- [ ] Download pdf files and open in zathura, courses should have a download directory for attachments specified by config file
- [ ] When opening pdf file that's already downloaded check to see if its been modified and update it if so
- [ ] ISSUE: read write precedence with multiple instances working on the same data
- [ ] Fetch external submission links (try REST query after its been migrated)
- [ ] If browser is opened with open::this(url) for the first time, allow the browser to remain open after the program exits
