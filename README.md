### playonbsd-browser

Terminal user interface to browse the PlayOnBSD database.


#### Roadmap

- [x] Properly display game list
- [x] Properly display game details
- [x] Use output of steamctl to indicate owned games in the list (no clue about how to do the same for Gog)
- [x] Filters on owned games
- [ ] Write a man page
- [ ] Provide instructions to install Steam games (no clue about how to do for Gog)
- [ ] Choose a license
- [ ] Integrate data from IGDB (probably using pobsd.chocolatines.org)
- [ ] Maybe provide a way to launch some games from the TUI (e.g., using IndieRunner)


#### Manpage
```
POBSD-BROWSER(1)            General Commands Manual           POBSD-BROWSER(1)

NAME
     pobsd-browser - terminal user interface for browsing the PlayOnBSD
     database

SYNOPSIS
     pobsd-browser [-f file] [-s file] [-u url] [-o]

DESCRIPTION
     Provides a terminal user interface for browsing the PlayOnBSD database
     provided by the playonbsd/OpenBSD-Games-Database GitHub repository.

     Its arguments are as follows:

     -f file
             Load the game database from the provided file (see FILES section
             for more details on the database format).  Cannot be used
             together with -o or -u argument

     -s file
             Use the Steam ids contained in the provided file to identify the
             games owned by the user (the id list can be obtained using
             steamctl; see FILES section for more details on the file format)

     -u url  Load the game database from the provided url (see FILES section
             for more details on the database format).  Cannot be used
             together with -f or -o argument

     -o      Load the game database from the game database provided by the
             official PlayOnBSD GitHub repository.  Cannot be used together
             with -f or -u argument

     At least one the the -f , -u or -o argument must be provided.

COMMANDS
     pobsd-browser is a mode-based interface supporting two modes: a list mode
     and a search mode.  The commands differ according to the mode.  In the
     following descriptions, ^X means control-X.

   list mode
     In this mode, it is possible to navigate in the game list.  When a game
     is selected, the corresponding details are displayed in the right panel.

     G       go to the last game of the list

     g       go to the first game of the list

     j | down arrow
             scroll down in the list

     k | up arrow
             scroll up in the list

     o       toggle between a first mode where all games are displayed and a
             second mode where only owned games are displayed (show nothing if
             no Steam ids are provided)

     q | ^C | ^c
             exit application

     ESC     clear the search

     TAB | /
             switch to search mode

   search mode
     In this mode, the list of games is updated according to the search.  The
     search is performed on the name, engine, runtime, tag and genre fields.

     TAB | ESC | RETURN
             exit from search mode back to list mode

FILES
     pobsd-browser uses a game database and, optionally, a file containing a
     list of Steam ids.

   game database
     The format of the game database is described in the official PlayOnBSD
     GitHub repository.  Please refer to it for up-to-date formatting.

   list of Steam ids
     The file providing the Steam ids should contain space separated values,
     and the first column should contain the Steam ids.  The correct
     formatting can be obtain with steamctl and used with pobsd-browser using
     the following command:

           $ steamctl apps list > steam_ids.txt
           $ pobsd-browser -o -s steam_ids.txt

AUTHORS
     pobsd-browser was written by Hukadan, me@hukadan.org.

OpenBSD 7.6                   September 11, 2024                   OpenBSD 7.6
```
