use gtk::prelude::*;
use gtk::{Application, gio};

/// Adds all application actions to the GTK application.
pub fn add_actions(app: &Application) {
    // File menu actions
    let new_action = gio::SimpleAction::new("new", None);
    new_action.connect_activate(|_, _| println!("New action activated"));
    app.add_action(&new_action);

    let open_action = gio::SimpleAction::new("open", None);
    open_action.connect_activate(|_, _| println!("Open action activated"));
    app.add_action(&open_action);

    let save_action = gio::SimpleAction::new("save", None);
    save_action.connect_activate(|_, _| println!("Save action activated"));
    app.add_action(&save_action);

    let save_as_action = gio::SimpleAction::new("save-as", None);
    save_as_action.connect_activate(|_, _| println!("Save As action activated"));
    app.add_action(&save_as_action);

    let options_action = gio::SimpleAction::new("options", None);
    options_action.connect_activate(|_, _| println!("Options action activated"));
    app.add_action(&options_action);

    let restart_action = gio::SimpleAction::new("restart", None);
    restart_action.connect_activate(|_, _| println!("Restart action activated"));
    app.add_action(&restart_action);

    let restart_admin_action = gio::SimpleAction::new("restart-admin", None);
    restart_admin_action.connect_activate(|_, _| println!("Restart as Admin action activated"));
    app.add_action(&restart_admin_action);

    let quit_action = gio::SimpleAction::new("quit", None);
    let app_weak = app.downgrade();
    quit_action.connect_activate(move |_, _| {
        println!("Quit action activated");
        if let Some(app) = app_weak.upgrade() {
            app.quit();
        }
    });
    app.add_action(&quit_action);

    // Edit menu actions
    let undo_action = gio::SimpleAction::new("undo", None);
    undo_action.connect_activate(|_, _| println!("Undo action activated"));
    app.add_action(&undo_action);

    let redo_action = gio::SimpleAction::new("redo", None);
    redo_action.connect_activate(|_, _| println!("Redo action activated"));
    app.add_action(&redo_action);

    let cut_action = gio::SimpleAction::new("cut", None);
    cut_action.connect_activate(|_, _| println!("Cut action activated"));
    app.add_action(&cut_action);

    let copy_action = gio::SimpleAction::new("copy", None);
    copy_action.connect_activate(|_, _| println!("Copy action activated"));
    app.add_action(&copy_action);

    let python_action = gio::SimpleAction::new("python", None);
    python_action.connect_activate(|_, _| println!("Python action activated"));
    app.add_action(&python_action);

    let paste_action = gio::SimpleAction::new("paste", None);
    paste_action.connect_activate(|_, _| println!("Paste action activated"));
    app.add_action(&paste_action);

    let delete_action = gio::SimpleAction::new("delete", None);
    delete_action.connect_activate(|_, _| println!("Delete action activated"));
    app.add_action(&delete_action);

    let find_action = gio::SimpleAction::new("find", None);
    find_action.connect_activate(|_, _| println!("Find action activated"));
    app.add_action(&find_action);

    let find_next_action = gio::SimpleAction::new("find-next", None);
    find_next_action.connect_activate(|_, _| println!("Find Next action activated"));
    app.add_action(&find_next_action);

    // View menu actions
    let show_toolbar_action = gio::SimpleAction::new_stateful("show-toolbar", None, true.to_variant());
    show_toolbar_action.connect_activate(|action, _| {
        let state = action.state().unwrap();
        let new_state = !state.get::<bool>().unwrap();
        action.set_state(new_state.to_variant());
        println!("Show toolbar toggled: {}", new_state);
    });
    app.add_action(&show_toolbar_action);

    let expand_action = gio::SimpleAction::new("expand", None);
    expand_action.connect_activate(|_, _| println!("Expand action activated"));
    app.add_action(&expand_action);

    let collapse_action = gio::SimpleAction::new("collapse", None);
    collapse_action.connect_activate(|_, _| println!("Collapse action activated"));
    app.add_action(&collapse_action);

    let expand_children_action = gio::SimpleAction::new("expand-children", None);
    expand_children_action.connect_activate(|_, _| println!("Expand Children action activated"));
    app.add_action(&expand_children_action);

    let collapse_children_action = gio::SimpleAction::new("collapse-children", None);
    collapse_children_action.connect_activate(|_, _| println!("Collapse Children action activated"));
    app.add_action(&collapse_children_action);

    let expand_all_action = gio::SimpleAction::new("expand-all", None);
    expand_all_action.connect_activate(|_, _| println!("Expand All action activated"));
    app.add_action(&expand_all_action);

    let collapse_all_action = gio::SimpleAction::new("collapse-all", None);
    collapse_all_action.connect_activate(|_, _| println!("Collapse All action activated"));
    app.add_action(&collapse_all_action);

    // Configuration menu actions
    let add_plugin_action = gio::SimpleAction::new("add-plugin", None);
    add_plugin_action.connect_activate(|_, _| println!("Add Plugin action activated"));
    app.add_action(&add_plugin_action);

    let add_folder_action = gio::SimpleAction::new("add-folder", None);
    add_folder_action.connect_activate(|_, _| println!("Add Folder action activated"));
    app.add_action(&add_folder_action);

    let add_macro_action = gio::SimpleAction::new("add-macro", None);
    add_macro_action.connect_activate(|_, _| println!("Add Macro action activated"));
    app.add_action(&add_macro_action);

    let add_event_action = gio::SimpleAction::new("add-event", None);
    add_event_action.connect_activate(|_, _| println!("Add Event action activated"));
    app.add_action(&add_event_action);

    let add_action_action = gio::SimpleAction::new("add-action", None);
    add_action_action.connect_activate(|_, _| println!("Add Action action activated"));
    app.add_action(&add_action_action);

    let configure_action = gio::SimpleAction::new("configure", None);
    configure_action.connect_activate(|_, _| println!("Configure action activated"));
    app.add_action(&configure_action);

    let rename_action = gio::SimpleAction::new("rename", None);
    rename_action.connect_activate(|_, _| println!("Rename action activated"));
    app.add_action(&rename_action);

    let execute_action = gio::SimpleAction::new("execute", None);
    execute_action.connect_activate(|_, _| println!("Execute action activated"));
    app.add_action(&execute_action);

    // Help menu actions
    let help_contents_action = gio::SimpleAction::new("help-contents", None);
    help_contents_action.connect_activate(|_, _| println!("Help Contents action activated"));
    app.add_action(&help_contents_action);

    let web_homepage_action = gio::SimpleAction::new("web-homepage", None);
    web_homepage_action.connect_activate(|_, _| println!("Web Homepage action activated"));
    app.add_action(&web_homepage_action);

    let web_forum_action = gio::SimpleAction::new("web-forum", None);
    web_forum_action.connect_activate(|_, _| println!("Web Forum action activated"));
    app.add_action(&web_forum_action);

    let web_wiki_action = gio::SimpleAction::new("web-wiki", None);
    web_wiki_action.connect_activate(|_, _| println!("Web Wiki action activated"));
    app.add_action(&web_wiki_action);

    let check_update_action = gio::SimpleAction::new("check-update", None);
    check_update_action.connect_activate(|_, _| println!("Check for Updates action activated"));
    app.add_action(&check_update_action);

    let python_shell_action = gio::SimpleAction::new("python-shell", None);
    python_shell_action.connect_activate(|_, _| println!("Python Shell action activated"));
    app.add_action(&python_shell_action);

    let about_action = gio::SimpleAction::new("about", None);
    about_action.connect_activate(|_, _| println!("About action activated"));
    app.add_action(&about_action);
} 