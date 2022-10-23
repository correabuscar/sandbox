// this file is named entry.c
// To repro. the issue:
// first select some text from the terminal so that it's in the PRIMARY selection
// this means now you can paste it anywhere by pressing MMB (middle mouse button) or pressing shift+insert
// Now compile and run this entry.c like this:
// $ rm a.out ; gcc -Werror entry.c `pkgconf glib-2.0 --cflags --libs` `pkgconf gtk+-3.0 --cflags --libs` && ./a.out
// Now try to MMB aka paste what's in PRIMARY selection
// if you get "blah" then congrats you got the issue.
// this is seemingly normal gtk3 behaviour tho.
//
// alternatively: don't use this file, repro. in another way:
// using 'mousepad' command, run it, write some random text, select some of it
// press Ctrl+F now it's inside the Find all selected
// select more/other text (because selection got lost), now press Ctrl+F again
// and the text in Find is selected again, and your previously selected text is lost
// the selected text(s) are also in PRIMARY selection (aka that MMB clipboard)) during all these steps. If you see it selected, it's in PRIMARY!
//
//
// src: 'gtk3-demo' command, Entry->Entry Buffer
// src: https://developer.gnome.org/gtk3/stable/ch01s04.html#id-1.2.3.12.6

#include <glib/gi18n.h>
#include <gtk/gtk.h>

GtkWidget *
do_entry_buffer (GtkWidget *do_widget)
{
  static GtkWidget *window = NULL;
  GtkWidget *vbox;
  GtkWidget *label;
  GtkWidget *entry;
  GtkEntryBuffer *buffer;

  if (!window)
    {
      window = gtk_window_new (GTK_WINDOW_TOPLEVEL);
      gtk_window_set_screen (GTK_WINDOW (window),
                             gtk_widget_get_screen (do_widget));
      gtk_window_set_title (GTK_WINDOW (window), "Entry Buffer");
      gtk_window_set_resizable (GTK_WINDOW (window), FALSE);
      g_signal_connect (window, "destroy",
                        G_CALLBACK (gtk_widget_destroyed), &window);

      vbox = gtk_box_new (GTK_ORIENTATION_VERTICAL, 5);
      gtk_container_add (GTK_CONTAINER (window), vbox);
      gtk_container_set_border_width (GTK_CONTAINER (vbox), 5);


      label = gtk_label_new (NULL);
      gtk_label_set_markup (GTK_LABEL (label),
                            "Entries share a buffer. Typing in one is reflected in the other.");
      gtk_box_pack_start (GTK_BOX (vbox), label, FALSE, FALSE, 0);

      /* Create a buffer */
      buffer = gtk_entry_buffer_new (NULL, 0);

      /* Create our first entry */
      entry = gtk_entry_new_with_buffer (buffer);
      gtk_box_pack_start (GTK_BOX (vbox), entry, FALSE, FALSE, 0);
      gtk_entry_set_text (GTK_ENTRY (entry), "blah"); // this is the text that will be auto-selected and copied into PRIMARY selection thus overwriting it

      gtk_widget_grab_focus( entry); //overwrites PRIMARY selection

      /* Create the second entry */
      entry = gtk_entry_new_with_buffer (buffer);
      gtk_entry_set_visibility (GTK_ENTRY (entry), FALSE);
      gtk_box_pack_start (GTK_BOX (vbox), entry, FALSE, FALSE, 0);

      g_object_unref (buffer);
      //gtk_widget_grab_focus( entry); //overwrites PRIMARY selection
      //gtk_entry_grab_focus_without_selecting( GTK_ENTRY(entry)); // doesn't overwrite or select
      //gtk_editable_select_region (GTK_EDITABLE (entry), 0, -1); // overwrites when selecting
    }

  if (!gtk_widget_get_visible (window))
    gtk_widget_show_all (window);
  else
    gtk_widget_destroy (window);

  return window;
}

//int main() {
//  GtkWidgetClass *widget_class = GTK_WIDGET_CLASS (class);
//  do_entry_buffer(widget_class);
////  GtkApplication *app;
////
////  app = GTK_APPLICATION (g_object_new (demo_application_get_type (),
////        "application-id", "org.gtk.Demo2",
////        "flags", G_APPLICATION_HANDLES_OPEN,
////        NULL));
////
////  return g_application_run (G_APPLICATION (app), 0, NULL);
//}

static void
activate (GtkApplication* app,
          gpointer        user_data)
{
  GtkWidget *window;

  window = gtk_application_window_new (app);
  gtk_window_set_title (GTK_WINDOW (window), "Window");
  gtk_window_set_default_size (GTK_WINDOW (window), 200, 200);
  gtk_widget_show_all (window);
//  GtkWidget *header_bar = gtk_header_bar_new ();
  //do_entry_buffer(header_bar);
  do_entry_buffer(window);
}

int
main (int    argc,
      char **argv)
{
  GtkApplication *app;
  int status;

  app = gtk_application_new ("org.gtk.example", G_APPLICATION_FLAGS_NONE);
  g_signal_connect (app, "activate", G_CALLBACK (activate), NULL);
  status = g_application_run (G_APPLICATION (app), argc, argv);
  g_object_unref (app);

  return status;
}
