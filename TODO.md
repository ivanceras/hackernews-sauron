# TODO
- [X] Rename `server page` to `index` to differentiage from actual page in client
        - Renamed client page to conten and retaint server to index
- [X] Push the history when clicks are intercepted by the client
        - Use history.push_state when navigating
        - Use `window.add_listener('popstate'` to listen to back button and rout to appropriate content.
- [x] Make the top stories link intercepted with mouse clicks
- [X] Make comment item become the page content.
- [ ] Deploy to heroku
- [ ] Make a pure client-side version for github.io hosting
    - Generate an `index.html` file based on `page` module with no app.
- [ ] Refactor code for Http fetch_stories.
    - Make a utility to convert futures into `Cmd`.
- [ ] Make a static database file format, where the app still works on
    static hosting sites (ie: no database, such as github pages, netlify)
