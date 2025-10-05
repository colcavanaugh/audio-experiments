# GitHub Project Board Setup Guide

This guide walks through setting up the GitHub Project board for the audio-experiments repository.

## Step 1: Create the Project

1. Go to https://github.com/colcavanaugh/audio-experiments
2. Click on the **Projects** tab
3. Click **New project**
4. Choose **Table** view
5. Name it: **Audio DSP Experiments**
6. Click **Create project**

## Step 2: Configure Custom Fields

Once the project is created, add these custom fields:

### Field 1: Project (Single Select)
- Click **+ New field** → **Single select**
- Name: `Project`
- Add options:
  - `naughty-and-tender`
  - `shared-utilities`
  - `infrastructure`
  - *(Add more as new projects are created)*

### Field 2: Category (Single Select)
- Click **+ New field** → **Single select**
- Name: `Category`
- Add options:
  - `Synthesis`
  - `Effects`
  - `Analysis`
  - `GUI/UX`
  - `Infrastructure`
  - `Documentation`
  - `Testing`

### Field 3: Learning Focus (Text)
- Click **+ New field** → **Text**
- Name: `Learning Focus`
- Description: Brief DSP concept being explored

### Field 4: Priority (Single Select)
- Click **+ New field** → **Single select**
- Name: `Priority`
- Add options:
  - `Critical` (🔴)
  - `High` (🟠)
  - `Medium` (🟡)
  - `Low` (🟢)

**Note**: Status is already included by default (Backlog, Todo, In Progress, Done)

## Step 3: Create Custom Views

GitHub Projects supports multiple views. Create these:

### View 1: Current Sprint
1. Click **+ New view** → **Table**
2. Name: `Current Sprint`
3. Filter: Status is `Todo` or `In Progress`
4. Group by: `Status`
5. Sort by: `Priority` (descending)

### View 2: By Project
1. Click **+ New view** → **Board**
2. Name: `By Project`
3. Filter: Status is not `Done`
4. Group by: `Project`
5. Sort by: `Priority` (descending)

### View 3: Learning Roadmap
1. Click **+ New view** → **Table**
2. Name: `Learning Roadmap`
3. No filters (show all)
4. Group by: `Category`
5. Sort by: `Project`, then `Priority`

### View 4: Completed Work
1. Click **+ New view** → **Table**
2. Name: `Completed`
3. Filter: Status is `Done`
4. Group by: `Project`
5. Sort by: Date closed (newest first)

## Step 4: Configure Automation (Optional)

In Project Settings:
1. **Auto-add items**: Turn on "Auto-add to project when issues/PRs are created"
2. **Auto-archive**: Turn on "Auto-archive items when closed"
3. **Default status**: Set to `Backlog` for new items

## Step 5: Update README Link

Once the project is set up, get its URL (should be something like):
```
https://github.com/users/colcavanaugh/projects/X
```

Update the link in [README.md](../README.md) under the "Project Management" section.

---

## Quick Reference: Using the Project Board

### Adding a New Issue
```bash
# After authenticating gh CLI:
gh issue create --project "Audio DSP Experiments"
```

Or use the web interface:
1. Go to Issues tab
2. Click "New issue"
3. Fill out template
4. Issue will auto-add to project

### Moving Issues
- Drag and drop between columns in Board view
- Or update Status field in issue sidebar

### Bulk Updates
- Select multiple issues (checkbox)
- Use bulk actions menu to update fields

---

*Follow this guide to get the project board fully configured!*
