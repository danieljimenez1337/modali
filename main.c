// Standard C libraries
#include <stdio.h>   // For g_print
#include <string.h>  // For strlen, strcmp
#include <stdlib.h>  // For setenv, realpath
#include <limits.h>  // For PATH_MAX
#include <time.h>    // For timing measurements
#include <sys/time.h> // For gettimeofday

// GLib family
#include <glib.h>    // For GString, g_get_user_config_dir(), etc.
#include <gio/gio.h> // For GApplication, g_get_application_executable_path(), etc.
#include <gio/gapplication.h> // Explicitly for GApplication specifics, if needed

// GTK family (GTK includes GDK)
#include <gtk/gtk.h> // Includes GDK, Pango, etc.

// Other libraries
#include <json-glib/json-glib.h> // For JSON parsing

// --- Timing Globals ---
static struct timeval app_start_time;
static gboolean first_input_received = FALSE;

// Helper function to get elapsed time in milliseconds
static double get_elapsed_ms(struct timeval start) {
    struct timeval now;
    gettimeofday(&now, NULL);
    return ((now.tv_sec - start.tv_sec) * 1000.0) + 
           ((now.tv_usec - start.tv_usec) / 1000.0);
}

// Helper function to print timing milestone
static void print_timing_milestone(const char* milestone) {
    double elapsed = get_elapsed_ms(app_start_time);
    g_print("[TIMING] %s: %.2f ms\n", milestone, elapsed);
}

// --- Key Binding Structures ---
typedef struct KeyAction KeyAction;
struct KeyAction {
    char key;                   // The key character for this action
    char* description;          // Description of this key/action
    KeyAction* sub_actions;     // Array of next possible actions (NULL terminated)
    char* command_to_run;       // If this key completes a command, this is the command
    void (*execute_func)(const char* command); // Function to execute the command
};

// Forward declarations
static void update_display_label(void);
static void process_key_press(const char* key_name);
static void load_key_bindings_from_json(const char *filename);
static void free_loaded_key_actions(KeyAction *actions_to_free);
static KeyAction* parse_json_array_to_key_actions(JsonArray *array);

// Function to execute the command
static void execute_command(const char* command_line) {
    if (!command_line) return;

    g_print("Attempting to execute: %s\n", command_line);

    gchar **argv = NULL;
    GError *error = NULL;

    // Split the command line into an argument vector
    if (!g_shell_parse_argv(command_line, NULL, &argv, &error)) {
        g_warning("Failed to parse command line: %s (Error: %s)", command_line, error->message);
        g_error_free(error);
        return;
    }

    // G_SPAWN_DO_NOT_REAP_CHILD is important for GTK applications to avoid issues with child process handling
    // if you don't explicitly manage the child process termination (e.g. via g_child_watch_add).
    if (!g_spawn_async(NULL, argv, NULL, G_SPAWN_SEARCH_PATH | G_SPAWN_DO_NOT_REAP_CHILD, NULL, NULL, NULL, &error)) {
        g_warning("Failed to execute command '%s': %s", argv[0], error->message);
        g_error_free(error);
    } else {
        g_print("Command '%s' launched successfully.\n", argv[0]);
    }
    
    g_strfreev(argv); // Free the argument vector
}

// --- Global State ---
GtkLabel *display_label;            // Label to show current sequence/options
GString *current_key_sequence;      // Stores the keys pressed so far
KeyAction *g_loaded_root_actions = NULL; // Root of the loaded key bindings
KeyAction *current_node_options = NULL; // Current level of key binding options being displayed/processed

// --- JSON Loading, Parsing, and Freeing Functions ---

static KeyAction* parse_json_array_to_key_actions(JsonArray *array) {
    if (!array) return NULL;

    guint length = json_array_get_length(array);
    if (length == 0) return NULL;

    // Allocate for KeyActions plus one for the NULL terminator
    KeyAction *actions = g_new0(KeyAction, length + 1);

    for (guint i = 0; i < length; ++i) {
        JsonNode *node = json_array_get_element(array, i);
        if (!JSON_NODE_HOLDS_OBJECT(node)) {
            g_warning("JSON array element at index %u is not an object.", i);
            continue;
        }
        JsonObject *obj = json_node_get_object(node);

        // Key (required, single char)
        if (json_object_has_member(obj, "key")) {
            const char *key_str = json_object_get_string_member(obj, "key");
            if (key_str && strlen(key_str) == 1) {
                actions[i].key = key_str[0];
            } else {
                g_warning("Invalid or missing 'key' for action at index %u. Must be a single character.", i);
                actions[i].key = 0; // Mark as invalid for now or skip
                continue; 
            }
        } else {
             g_warning("Missing 'key' for action at index %u.", i);
             continue;
        }

        // Description (required)
        if (json_object_has_member(obj, "description")) {
            actions[i].description = g_strdup(json_object_get_string_member(obj, "description"));
        } else {
            g_warning("Missing 'description' for key '%c'.", actions[i].key);
            actions[i].description = g_strdup("(no description)");
        }

        // Command (optional)
        if (json_object_has_member(obj, "command")) {
            actions[i].command_to_run = g_strdup(json_object_get_string_member(obj, "command"));
            actions[i].execute_func = execute_command; // Assign the execution function
        } else {
            actions[i].command_to_run = NULL;
            actions[i].execute_func = NULL;
        }

        // Sub-actions (optional, recursive call)
        if (json_object_has_member(obj, "sub_actions")) {
            JsonNode *sub_actions_node = json_object_get_member(obj, "sub_actions");
            if (JSON_NODE_HOLDS_ARRAY(sub_actions_node)) {
                actions[i].sub_actions = parse_json_array_to_key_actions(json_node_get_array(sub_actions_node));
            } else {
                g_warning("'sub_actions' for key '%c' is not an array.", actions[i].key);
                actions[i].sub_actions = NULL;
            }
        } else {
            actions[i].sub_actions = NULL;
        }
    }
    // The last element is already zeroed out by g_new0, serving as a terminator
    return actions;
}

static void load_key_bindings_from_json(const char *filename) {
    print_timing_milestone("JSON loading started");
    
    JsonParser *parser = json_parser_new();
    GError *error = NULL;

    if (!json_parser_load_from_file(parser, filename, &error)) {
        g_warning("Failed to load or parse '%s': %s", filename, error->message);
        g_error_free(error);
        g_object_unref(parser);
        // Fallback: could load some default hardcoded bindings or exit
        g_loaded_root_actions = NULL; // Ensure it's NULL if loading fails
        return;
    }

    JsonNode *root_node = json_parser_get_root(parser);
    if (!root_node || !JSON_NODE_HOLDS_ARRAY(root_node)) {
        g_warning("'%s' does not contain a root JSON array.", filename);
        g_object_unref(parser);
        g_loaded_root_actions = NULL;
        return;
    }

    JsonArray *root_array = json_node_get_array(root_node);
    g_loaded_root_actions = parse_json_array_to_key_actions(root_array);

    g_object_unref(parser);
    print_timing_milestone("JSON loading completed");
}

static void free_loaded_key_actions(KeyAction *actions_to_free) {
    if (!actions_to_free) return;

    for (int i = 0; actions_to_free[i].key != 0; ++i) {
        g_free(actions_to_free[i].description);
        g_free(actions_to_free[i].command_to_run);
        if (actions_to_free[i].sub_actions) {
            free_loaded_key_actions(actions_to_free[i].sub_actions);
        }
    }
    g_free(actions_to_free);
}

// --- Functions ---

static void update_display_label() {
    GString *text_to_display = g_string_new("");

    if (current_node_options) {
        int num_options = 0;
        for (int i = 0; current_node_options[i].key != 0; ++i) {
            num_options++;
        }

        if (num_options > 0) {
            const int NUM_COLUMNS = 3;
            int items_per_full_column = (num_options + NUM_COLUMNS - 1) / NUM_COLUMNS; // Ceiling division
            
            const int MAX_DESC_CHARS = 22; // Max characters for the description part (incl. ellipsis if needed)
            const char *ELLIPSIS_STR = "...";
            const int ELLIPSIS_STR_LEN = strlen(ELLIPSIS_STR);
            const int ITEM_TEXT_CONTENT_WIDTH = 5 + MAX_DESC_CHARS; // 5 for "  k: " part
            const char *COLUMN_SEPARATOR = "  "; // Two spaces between columns

            for (int i = 0; i < items_per_full_column; ++i) { // Iterate down the rows
                GString *line_str = g_string_new("");
                for (int j = 0; j < NUM_COLUMNS; ++j) { // Iterate across the columns
                    int option_idx = j * items_per_full_column + i;
                    if (option_idx < num_options) {
                        char desc_to_display[MAX_DESC_CHARS + 1];
                        const char *original_desc = current_node_options[option_idx].description;

                        if (strlen(original_desc) > MAX_DESC_CHARS) {
                            // Ensure space for ellipsis by truncating original_desc appropriately
                            g_snprintf(desc_to_display, sizeof(desc_to_display), "%.*s%s", 
                                       MAX_DESC_CHARS - ELLIPSIS_STR_LEN, 
                                       original_desc, 
                                       ELLIPSIS_STR);
                        } else {
                            g_snprintf(desc_to_display, sizeof(desc_to_display), "%s", original_desc);
                        }
                        
                        char current_item_full_text[ITEM_TEXT_CONTENT_WIDTH + 10]; // Buffer for "  k: desc_to_display"
                        g_snprintf(current_item_full_text, sizeof(current_item_full_text), "  %c: %s",
                                   current_node_options[option_idx].key,
                                   desc_to_display);

                        // Append the item, padded to its content width
                        g_string_append_printf(line_str, "%-*s", ITEM_TEXT_CONTENT_WIDTH, current_item_full_text);
                        
                        // Add column separator if not the last column in this row AND if there's a next item to print
                        if (j < NUM_COLUMNS - 1) {
                            if (((j + 1) * items_per_full_column + i) < num_options) { // Check if next column cell in this row has an item
                                g_string_append(line_str, COLUMN_SEPARATOR);
                            }
                        }
                    } else {
                        // If no item for this cell, append spaces to maintain alignment
                        g_string_append_printf(line_str, "%*s", ITEM_TEXT_CONTENT_WIDTH, "");
                    }
                }
                g_string_append_printf(text_to_display, "%s\n", line_str->str);
                g_string_free(line_str, TRUE);
            }
        } else { // num_options is 0, but current_node_options might be a valid (empty) level
            if (current_key_sequence->len > 0) {
                g_string_append(text_to_display, "  (No further actions defined for this sequence)\n");
            } else {
                g_string_append(text_to_display, "  (No key bindings loaded or root is empty)\n");
            }
        }
    } else { // current_node_options is NULL (e.g. invalid sequence or load error)
        if (current_key_sequence->len > 0) {
             g_string_append(text_to_display, "  (Invalid key sequence - no matching options)\n");
        } else { // At root, and g_loaded_root_actions was NULL
            g_string_append(text_to_display, "  (No key bindings loaded or error during load)\n");
        }
    }
    gtk_label_set_text(display_label, text_to_display->str);
    g_string_free(text_to_display, TRUE);
}

static void reset_key_sequence() {
    g_string_truncate(current_key_sequence, 0);
    current_node_options = g_loaded_root_actions;
    update_display_label();
}

static void process_key_press(const char* key_name_char_array) {
    // Measure time to first input
    if (!first_input_received) {
        first_input_received = TRUE;
        print_timing_milestone("FIRST INPUT RECEIVED");
        g_print("[TIMING] === TIME TO FIRST INPUT: %.2f ms ===\n", get_elapsed_ms(app_start_time));
    }

    // Handle Backspace to go up a level
    if (g_strcmp0(key_name_char_array, "Backspace") == 0) {
        if (current_key_sequence->len > 0) {
            g_string_truncate(current_key_sequence, current_key_sequence->len - 1);
            
            // Reset to root and re-evaluate the sequence
            current_node_options = g_loaded_root_actions;
            if (current_key_sequence->len > 0) {
                KeyAction *temp_options = g_loaded_root_actions;
                for (gsize i = 0; i < current_key_sequence->len; ++i) {
                    char key_in_seq = current_key_sequence->str[i];
                    gboolean found_in_seq = FALSE;
                    if (temp_options) {
                        for (int j = 0; temp_options[j].key != 0; ++j) {
                            if (temp_options[j].key == key_in_seq) {
                                temp_options = temp_options[j].sub_actions;
                                found_in_seq = TRUE;
                                break;
                            }
                        }
                    }
                    if (!found_in_seq) { // Should not happen if sequence was valid
                        temp_options = NULL; // Path broken
                        break;
                    }
                }
                current_node_options = temp_options;
            } else {
                current_node_options = g_loaded_root_actions; // Back to root if sequence is empty
            }
        } else {
             // Already at root, or sequence empty, do nothing for Backspace
        }
        update_display_label();
        return;
    }
    if (strlen(key_name_char_array) == 1) { // Process single character keys
        char pressed_key = key_name_char_array[0];

        KeyAction *selected_action = NULL;
        if (current_node_options) {
            for (int i = 0; current_node_options[i].key != 0; ++i) {
                if (current_node_options[i].key == pressed_key) {
                    selected_action = &current_node_options[i];
                    break;
                }
            }
        }

        if (selected_action) {
            g_string_append_c(current_key_sequence, pressed_key);

            if (selected_action->command_to_run) { // Command found
                if (selected_action->execute_func) {
                    selected_action->execute_func(selected_action->command_to_run);
                }
                g_print("Command: %s\nDescription: %s\n", selected_action->command_to_run, selected_action->description);
                // reset_key_sequence(); // App will quit, so reset is not strictly needed here
                g_application_quit(g_application_get_default()); // Quit the application 
            } else if (selected_action->sub_actions) { // More sub-actions available
                current_node_options = selected_action->sub_actions;
                update_display_label();
            } else { // End of a branch, but no command (should be configured better)
                 g_print("End of sequence '%s', but no command defined.\n", current_key_sequence->str);
                 reset_key_sequence();
            }
        } else {
            // Invalid key in the current context
            g_print("Invalid key '%c' in sequence '%s'. Resetting.\n", pressed_key, current_key_sequence->str);
            reset_key_sequence();
        }
    } else if (g_strcmp0(key_name_char_array, "Escape") == 0) {
        g_print("Escape pressed, resetting sequence.\n");
        reset_key_sequence();
    }
    // Other non-char keys are ignored for now
}

static gboolean on_key_pressed_event(
    GtkEventControllerKey *controller,
    guint keyval,
    guint keycode,
    GdkModifierType state,
    gpointer user_data
) {
    // We are interested in simple character keys for bindings, and Escape
    // Convert keyval to a character, handling only basic a-z, A-Z, 0-9 for now
    char key_char_str[2] = {0}; 
    gunichar unicode_char = gdk_keyval_to_unicode(keyval);

    if ((unicode_char >= 'a' && unicode_char <= 'z') ||
        (unicode_char >= 'A' && unicode_char <= 'Z') ||
        (unicode_char >= '0' && unicode_char <= '9')) {
        key_char_str[0] = g_unichar_tolower(unicode_char); // Convert to lowercase for bindings
        process_key_press(key_char_str);
    } else if (keyval == GDK_KEY_Escape) {
        g_application_quit(g_application_get_default()); // Escape now closes the app
    } else if (keyval == GDK_KEY_BackSpace) {
        process_key_press("Backspace");
    }
    
    return TRUE; // Event handled, stop propagation
}

// Callback to measure when window is actually ready for input
static void on_window_mapped(GtkWidget *window, gpointer user_data) {
    print_timing_milestone("Window mapped (visible and ready)");
}

static void activate(GtkApplication *app, gpointer user_data) {
    print_timing_milestone("GTK activate callback started");
    
    GtkWidget *window = gtk_application_window_new(app);
    gtk_window_set_title(GTK_WINDOW(window), "Modali Launcher");
    gtk_window_set_default_size(GTK_WINDOW(window), 1200, 350); // Wider window
    gtk_widget_set_opacity(window, 0.85); // Set window opacity
    // For a launcher, you might want it to be non-resizable, always on top, and undecorated.
    // These require more specific window manager hints or GTK4 equivalents.
    gtk_window_set_resizable(GTK_WINDOW(window), FALSE);
    gtk_window_set_decorated(GTK_WINDOW(window), FALSE); // For a minimal look

    print_timing_milestone("Window created and configured");

    GtkWidget *main_box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0); // Spacing set to 0, CSS handles padding
    gtk_widget_add_css_class(main_box, "modali-main-box");
    gtk_window_set_child(GTK_WINDOW(window), main_box);

    display_label = GTK_LABEL(gtk_label_new(""));
    gtk_label_set_xalign(display_label, 0.0); // Align text block to the left
    gtk_label_set_yalign(display_label, 0.0); // Align text block to the top
    gtk_label_set_wrap(display_label, TRUE);
    gtk_widget_set_vexpand(GTK_WIDGET(display_label), TRUE); // Make display_label expand vertically
    gtk_widget_set_hexpand(GTK_WIDGET(display_label), TRUE); // Make display_label expand horizontally
    gtk_widget_add_css_class(GTK_WIDGET(display_label), "modali-display-label");

    gtk_box_append(GTK_BOX(main_box), GTK_WIDGET(display_label));

    GtkWidget *info_label = gtk_label_new("Esc: Close | Backspace: Up");
    gtk_widget_add_css_class(info_label, "modali-info-label");
    gtk_label_set_xalign(GTK_LABEL(info_label), 0.5); // Center the text block
    gtk_label_set_justify(GTK_LABEL(info_label), GTK_JUSTIFY_CENTER); // Center the text content
    gtk_box_append(GTK_BOX(main_box), info_label);

    print_timing_milestone("Widget hierarchy created");

    // Initialize global state
    current_key_sequence = g_string_new("");

    // Construct path for bindings.json in XDG config directory
    const char *user_config_dir = g_get_user_config_dir();
    char *bindings_path = g_build_filename(user_config_dir, "modali", "bindings.json", NULL);
    load_key_bindings_from_json(bindings_path);
    g_free(bindings_path);
    current_node_options = g_loaded_root_actions; // Start with loaded root options

    // Load CSS
    print_timing_milestone("CSS loading started");
    GtkCssProvider *provider = gtk_css_provider_new();
    char *style_path_final = NULL;
    char resolved_exe_path[PATH_MAX];

    if (realpath("/proc/self/exe", resolved_exe_path) != NULL) {
        char *exe_dir = g_path_get_dirname(resolved_exe_path);
        if (exe_dir) {
            // Path relative to exe: $exe_dir/../share/modali/style.css
            style_path_final = g_build_filename(exe_dir, "..", "share", "modali", "style.css", NULL);
            g_free(exe_dir);
        } else {
            g_warning("Could not get directory name from resolved executable path: %s. Falling back to local style.css", resolved_exe_path);
            style_path_final = g_strdup("style.css");
        }
    } else {
        // Fallback if realpath("/proc/self/exe", ...) failed
        g_warning("realpath(\"/proc/self/exe\") failed. Falling back to local style.css");
        style_path_final = g_strdup("style.css");
    }

    if (style_path_final) {
        gtk_css_provider_load_from_path(provider, style_path_final);
        g_print("Attempted to load CSS from: %s\n", style_path_final);
        g_free(style_path_final);
    } else {
        g_warning("Could not determine path for style.css");
    }
    gtk_style_context_add_provider_for_display(
        gdk_display_get_default(),
        GTK_STYLE_PROVIDER(provider),
        GTK_STYLE_PROVIDER_PRIORITY_USER
    );
    g_object_unref(provider); // Unref provider after adding it
    print_timing_milestone("CSS loading completed");

    update_display_label(); // Initial display update
    print_timing_milestone("Initial display updated");

    // Setup key event controller
    GtkEventController *key_controller = gtk_event_controller_key_new();
    g_signal_connect(key_controller, "key-pressed", G_CALLBACK(on_key_pressed_event), NULL);
    gtk_widget_add_controller(window, key_controller); // Add controller to window

    // Connect to window mapped signal to know when it's actually visible
    g_signal_connect(window, "map", G_CALLBACK(on_window_mapped), NULL);

    gtk_widget_add_css_class(window, "modali-launcher"); // Apply top-level window CSS class
    gtk_widget_set_visible(window, TRUE);
    print_timing_milestone("Window set visible");
    
    gtk_window_present(GTK_WINDOW(window)); // Ensure window gets focus
    print_timing_milestone("Window presented (focus requested)");
}

int main(int argc, char **argv) {
    // Record start time immediately
    gettimeofday(&app_start_time, NULL);
    g_print("[TIMING] === APPLICATION STARTUP TIMING ===\n");
    print_timing_milestone("Application main() started");

    // Force Cairo renderer for GSK to ensure fast startup
    setenv("GSK_RENDERER", "cairo", 1); // 1 means overwrite if already set
    print_timing_milestone("Environment configured");

    GtkApplication *app = gtk_application_new("org.example.modali.launcher", G_APPLICATION_DEFAULT_FLAGS);
    print_timing_milestone("GtkApplication created");
    
    g_signal_connect(app, "activate", G_CALLBACK(activate), NULL);
    print_timing_milestone("Activate signal connected");
    
    g_print("[TIMING] Starting g_application_run...\n");
    int status = g_application_run(G_APPLICATION(app), argc, argv);
    
    // Cleanup global GString
    if (current_key_sequence) {
        g_string_free(current_key_sequence, TRUE);
    }
    if (g_loaded_root_actions) {
        free_loaded_key_actions(g_loaded_root_actions);
    }
    g_object_unref(app);
    
    g_print("[TIMING] Application exited with status: %d\n", status);
    return status;
}
