//#ifndef PRE_PROCESSING_BY_BUILD_RS
//#error "You're not supposed to compile this directly, build.rs pre-processes it before compiling it, and after it runs it, it post-processes its stdout to then generate the .rs file based on that output, which is then used to compile the crate. Look for them in OUT_DIR, eg. under ./target/ dir."
//#endif

#include <stdio.h>
//#include <string.h> // for strncmp
#include <curses.h>

#define PCONST(ty, NAME) printf("pub const " #NAME ": " #ty " = %lld;\n", (long long) (NAME))
#define PCONSTU(ty, NAME) printf("pub const " #NAME ": " #ty " = %llu;\n", (unsigned long long) (NAME))

//#define STRINGIFY_ITS_VALUE(x) #x
//
//#define IS_DEFINED(ID_NAME, CALL_WHEN_YES, CALL_WHEN_NO) \
//  do { \
//    size_t len1=sizeof(STRINGIFY_ITS_VALUE(ID_NAME)); \
//    size_t len2=sizeof(#ID_NAME); \
//    size_t maxlen= len1>len2?len1:len2; \
//    if (strncmp(STRINGIFY_ITS_VALUE(ID_NAME), #ID_NAME, maxlen) != 0) { \
//      CALL_WHEN_YES(ID_NAME); \
//    } else {\
//      CALL_WHEN_NO(ID_NAME); \
//    } \
//  } while(0)
//
//#define CAN_MISS(ID_NAME, type) \
//  IS_DEFINED(ID_NAME, PCONST(ID_NAME, type), FOO);
//
////      printf("defined "#ID_NAME"\n");
////      printf("NOT defined "#ID_NAME"\n");

//#define UNEXPECTED_MISS(NAME) fprintf(stderr,"Unexpected missing def: " #NAME "\n");
#define EXPECT_MISS(NAME) fprintf(stderr,"Expected missing def: " #NAME "\n");
//#define UNEXPECTED_MISS(NAME) printf("pub const " #NAME ": &str = \"You don't have " #NAME " in your ncurses.h header file which means you're probably using older version of ncurses\";\n");
#define ERROR_MSG_TYPE "Your_ncurses_installation_is_too_old_and_it_does_not_have_this_underlined_identifier_defined_in_its_header_file_therefore_the_ncurses_crate_did_not_include_it_because_it_tries_to_be_compatible_however_this_crate_in_which_you_see_the_error_needs_the_identifier__If_you_are_on_MacOS_then_use_the_brew_version_of_ncurses_and_set_PKG_CONFIG_PATH_to_the_pkgconfig_dir_from_within_it"
#define ERROR_MSG_TYPE_ALIAS "TypeAliasForErrorMsgType"
#define ERROR_MSG_TYPE_CONSTRUCTOR "ERROR_MSG_TYPE_CONSTRUCTOR"
//#define UNEXPECTED_MISS(NAME) printf("pub const " #NAME ": " ERROR_MSG_TYPE " = " ERROR_MSG_TYPE ";\n");
#define UNEXPECTED_MISS(NAME) printf("pub const " #NAME ": " ERROR_MSG_TYPE_ALIAS " = " ERROR_MSG_TYPE_CONSTRUCTOR ";\n");

//build.rs converts any rust_* (starting lines) like rust_PCONST(i32, KEY_EVENT); into:
//#ifdef KEY_EVENT
//  PCONST(i32, KEY_EVENT);
//#else
//  printf("Missing def: KEY_EVENT\n")
//#endif
//and canmiss_PCONST(i32, KEY_EVENT); into:
//#ifdef KEY_EVENT
//  PCONST(i32, KEY_EVENT);
//#else
//  printf("Missing def(expected): KEY_EVENT\n")
//#endif
//Then, at post-processing, after running the binary that this .c file generates,
//the stdout is processed by build.rs to know if it needs to gather some info to issue warnings,
//such as, in case of "Missing def" which aren't "expected", a warning will be issued
//with all missing defs, but then the expected missing defs will also be mentioned.
//Otherwise, if no unexpected missing defs, no warnings will be issued.
//Warnings are cargo:warning={} messages on stdout during build.rs run eg. at 'cargo build' time
//which cargo interprets and warns accordingly.

int main() {
	/* some values aren't set until after this is run */
	printf("//");
	//fflush(stdout);fflush(stderr);*((int *)0) = 42; //segfault(on purpose for testing purposes) before terminal gets messed up needing a `reset` shell command to restore!
	initscr();
	endwin();
	printf("\n");

  printf("/// For MacOS: `brew install ncurses` then `export PKG_CONFIG_PATH=\"/usr/local/opt/ncurses/lib/pkgconfig\"`\n/// that will give you a compatible/newer ncurses installation.\n");
  //using these avoids duplication of the long-named type
  printf("#[derive(Debug)] // derive to avoid a warning\n\
pub struct "ERROR_MSG_TYPE";\n\n\
type "ERROR_MSG_TYPE_ALIAS" = "ERROR_MSG_TYPE";\n\n\
const "ERROR_MSG_TYPE_CONSTRUCTOR": "ERROR_MSG_TYPE_ALIAS" = "ERROR_MSG_TYPE";\n\n\
\n");

	/* Success/Failure. */
	PCONST(i32, ERR);
	PCONST(i32, OK);
	PCONST(c_bool, TRUE);
	PCONST(c_bool, FALSE);

	/* Attributes. */
#ifdef NCURSES_ATTR_SHIFT
	PCONST(u32, NCURSES_ATTR_SHIFT);
#else
#error "foo"
#endif

	/* Colors */
	PCONST(i16, COLOR_BLACK);
	PCONST(i16, COLOR_RED);
	PCONST(i16, COLOR_GREEN);
	PCONST(i16, COLOR_YELLOW);
	PCONST(i16, COLOR_BLUE);
	PCONST(i16, COLOR_MAGENTA);
	PCONST(i16, COLOR_CYAN);
	PCONST(i16, COLOR_WHITE);

	/* Values for the _flags member */
#ifdef _SUBWIN
	PCONST(i32, _SUBWIN);
#else
#error "foo"
#endif

#ifdef _ENDLINE
	PCONST(i32, _ENDLINE);
#else
#error "foo"
#endif

#ifdef _FULLWIN
	PCONST(i32, _FULLWIN);
#else
#error "foo"
#endif

#ifdef _SCROLLWIN
	PCONST(i32, _SCROLLWIN);
#else
#error "foo"
#endif

#ifdef _ISPAD
	PCONST(i32, _ISPAD);
#else
#error "foo"
#endif

#ifdef _HASMOVED
	PCONST(i32, _HASMOVED);
#else
#error "foo"
#endif

#ifdef _WRAPPED
	PCONST(i32, _WRAPPED);
#else
#error "foo"
#endif

	/*
	 * This value is used in the firstchar and lastchar fields to mark
	 * unchanged lines
	 */
#ifdef _NOCHANGE
	PCONST(i32, _NOCHANGE);
#else
#error "foo"
#endif

	/*
	 * This value is used in the oldindex field to mark lines created by insertions
	 * and scrolls.
	 */
#ifdef _NEWINDEX
	PCONST(i32, _NEWINDEX);
#else
#error "foo"
#endif

	/* Keys */
	PCONST(i32, KEY_CODE_YES);
	PCONST(i32, KEY_MIN);
	PCONST(i32, KEY_BREAK);
	PCONST(i32, KEY_SRESET);
	PCONST(i32, KEY_RESET);
	PCONST(i32, KEY_DOWN);
	PCONST(i32, KEY_UP);
	PCONST(i32, KEY_LEFT);
	PCONST(i32, KEY_RIGHT);
	PCONST(i32, KEY_HOME);
	PCONST(i32, KEY_BACKSPACE);
	PCONST(i32, KEY_F0);
	PCONST(i32, KEY_DL);
	PCONST(i32, KEY_IL);
	PCONST(i32, KEY_DC);
	PCONST(i32, KEY_IC);
	PCONST(i32, KEY_EIC);
	PCONST(i32, KEY_CLEAR);
	PCONST(i32, KEY_EOS);
	PCONST(i32, KEY_EOL);
	PCONST(i32, KEY_SF);
	PCONST(i32, KEY_SR);
	PCONST(i32, KEY_NPAGE);
	PCONST(i32, KEY_PPAGE);
	PCONST(i32, KEY_STAB);
	PCONST(i32, KEY_CTAB);
	PCONST(i32, KEY_CATAB);
	PCONST(i32, KEY_ENTER);
	PCONST(i32, KEY_PRINT);
	PCONST(i32, KEY_LL);
#ifdef KEY_A1
	PCONST(i32, KEY_A1);
#else
#error "foo"
#endif
#ifdef KEY_A3
	PCONST(i32, KEY_A3);
#else
#error "foo"
#endif
#ifdef KEY_B2
	PCONST(i32, KEY_B2);
#else
#error "foo"
#endif
#ifdef KEY_C1
	PCONST(i32, KEY_C1);
#else
#error "foo"
#endif
#ifdef KEY_C3
	PCONST(i32, KEY_C3);
#else
#error "foo"
#endif
	PCONST(i32, KEY_BTAB);
	PCONST(i32, KEY_BEG);
	PCONST(i32, KEY_CANCEL);
	PCONST(i32, KEY_CLOSE);
	PCONST(i32, KEY_COMMAND);
	PCONST(i32, KEY_COPY);
	PCONST(i32, KEY_CREATE);
	PCONST(i32, KEY_END);
	PCONST(i32, KEY_EXIT);
	PCONST(i32, KEY_FIND);
	PCONST(i32, KEY_HELP);
	PCONST(i32, KEY_MARK);
	PCONST(i32, KEY_MESSAGE);
	PCONST(i32, KEY_MOVE);
	PCONST(i32, KEY_NEXT);
	PCONST(i32, KEY_OPEN);
	PCONST(i32, KEY_OPTIONS);
	PCONST(i32, KEY_PREVIOUS);
	PCONST(i32, KEY_REDO);
	PCONST(i32, KEY_REFERENCE);
	PCONST(i32, KEY_REFRESH);
	PCONST(i32, KEY_REPLACE);
	PCONST(i32, KEY_RESTART);
	PCONST(i32, KEY_RESUME);
	PCONST(i32, KEY_SAVE);
	PCONST(i32, KEY_SBEG);
	PCONST(i32, KEY_SCANCEL);
	PCONST(i32, KEY_SCOMMAND);
	PCONST(i32, KEY_SCOPY);
	PCONST(i32, KEY_SCREATE);
	PCONST(i32, KEY_SDC);
	PCONST(i32, KEY_SDL);
	PCONST(i32, KEY_SELECT);
	PCONST(i32, KEY_SEND);
	PCONST(i32, KEY_SEOL);
	PCONST(i32, KEY_SEXIT);
	PCONST(i32, KEY_SFIND);
	PCONST(i32, KEY_SHELP);
	PCONST(i32, KEY_SHOME);
	PCONST(i32, KEY_SIC);
	PCONST(i32, KEY_SLEFT);
	PCONST(i32, KEY_SMESSAGE);
	PCONST(i32, KEY_SMOVE);
	PCONST(i32, KEY_SNEXT);
	PCONST(i32, KEY_SOPTIONS);
	PCONST(i32, KEY_SPREVIOUS);
	PCONST(i32, KEY_SPRINT);
	PCONST(i32, KEY_SREDO);
	PCONST(i32, KEY_SREPLACE);
	PCONST(i32, KEY_SRIGHT);
	PCONST(i32, KEY_SRSUME);
	PCONST(i32, KEY_SSAVE);
	PCONST(i32, KEY_SSUSPEND);
	PCONST(i32, KEY_SUNDO);
	PCONST(i32, KEY_SUSPEND);
	PCONST(i32, KEY_UNDO);
	PCONST(i32, KEY_MOUSE);
	PCONST(i32, KEY_RESIZE);

//  CAN_MISS(KEY_EVENT, i32);
#ifdef KEY_EVENT
	PCONST(i32, KEY_EVENT);
#else
  EXPECT_MISS(KEY_EVENT);
#endif
	PCONST(i32, KEY_MAX);

#ifdef NCURSES_MOUSE_VERSION
	PCONST(i32, NCURSES_MOUSE_VERSION);
#else
#error "foo"
#endif

#ifdef MASK_SHIFT
	PCONST(i32, MASK_SHIFT);
#else
  EXPECT_MISS(MASK_SHIFT);
#endif

#ifdef MODIFIER_SHIFT
	PCONST(i32, MODIFIER_SHIFT);
#else
  EXPECT_MISS(MODIFIER_SHIFT);
#endif

	/* Mouse Support */
#ifdef NCURSES_BUTTON_RELEASED
	PCONST(i32, NCURSES_BUTTON_RELEASED);
#else
#error "foo"
#endif

#ifdef NCURSES_BUTTON_PRESSED
	PCONST(i32, NCURSES_BUTTON_PRESSED);
#else
#error "foo"
#endif

#ifdef NCURSES_BUTTON_CLICKED
	PCONST(i32, NCURSES_BUTTON_CLICKED);
#else
#error "foo"
#endif

#ifdef NCURSES_DOUBLE_CLICKED
	PCONST(i32, NCURSES_DOUBLE_CLICKED);
#else
#error "foo"
#endif

#ifdef NCURSES_TRIPLE_CLICKED
	PCONST(i32, NCURSES_TRIPLE_CLICKED);
#else
#error "foo"
#endif

#ifdef NCURSES_RESERVED_EVENT
	PCONST(i32, NCURSES_RESERVED_EVENT);
#else
  EXPECT_MISS(NCURSES_RESERVED_EVENT);
#endif

	/* event masks */
	PCONST(i32, BUTTON1_RELEASED);
	PCONST(i32, BUTTON1_PRESSED);
	PCONST(i32, BUTTON1_CLICKED);
	PCONST(i32, BUTTON1_DOUBLE_CLICKED);
	PCONST(i32, BUTTON1_TRIPLE_CLICKED);

	PCONST(i32, BUTTON2_RELEASED);
	PCONST(i32, BUTTON2_PRESSED);
	PCONST(i32, BUTTON2_CLICKED);
	PCONST(i32, BUTTON2_DOUBLE_CLICKED);
	PCONST(i32, BUTTON2_TRIPLE_CLICKED);

	PCONST(i32, BUTTON3_RELEASED);
	PCONST(i32, BUTTON3_PRESSED);
	PCONST(i32, BUTTON3_CLICKED);
	PCONST(i32, BUTTON3_DOUBLE_CLICKED);
	PCONST(i32, BUTTON3_TRIPLE_CLICKED);

	PCONST(i32, BUTTON4_RELEASED);
	PCONST(i32, BUTTON4_PRESSED);
	PCONST(i32, BUTTON4_CLICKED);
	PCONST(i32, BUTTON4_DOUBLE_CLICKED);
	PCONST(i32, BUTTON4_TRIPLE_CLICKED);

#ifdef BUTTON5_RELEASED
	PCONST(i32, BUTTON5_RELEASED);
#else
#error "foo"
#endif

#ifdef BUTTON5_PRESSED
	PCONST(i32, BUTTON5_PRESSED);
#else
#error "foo"
#endif

#ifdef BUTTON5_CLICKED
	PCONST(i32, BUTTON5_CLICKED);
#else
#error "foo"
#endif

#ifdef BUTTON5_DOUBLE_CLICKED
	PCONST(i32, BUTTON5_DOUBLE_CLICKED);
#else
#error "foo"
#endif

#ifdef BUTTON5_TRIPLE_CLICKED
	PCONST(i32, BUTTON5_TRIPLE_CLICKED);
#else
#error "foo"
#endif

	PCONST(i32, BUTTON_CTRL);
	PCONST(i32, BUTTON_SHIFT);
	PCONST(i32, BUTTON_ALT);
	PCONST(i32, REPORT_MOUSE_POSITION);

	PCONST(i32, ALL_MOUSE_EVENTS);

	/* Attributes */
	PCONSTU(crate::ll::chtype, A_NORMAL);
	PCONSTU(crate::ll::chtype, A_STANDOUT);
	PCONSTU(crate::ll::chtype, A_UNDERLINE);
#ifdef A_ITALIC
//	PCONSTU(crate::ll::chtype, A_ITALIC);
//#else
  UNEXPECTED_MISS(A_ITALIC);
#endif
	PCONSTU(crate::ll::chtype, A_REVERSE);
	PCONSTU(crate::ll::chtype, A_BLINK);
	PCONSTU(crate::ll::chtype, A_DIM);
	PCONSTU(crate::ll::chtype, A_BOLD);

#ifdef A_BLANK
	PCONSTU(crate::ll::chtype, A_BLANK);
#else
  EXPECT_MISS(A_BLANK);
#endif

	PCONSTU(crate::ll::chtype, A_INVIS);
	PCONSTU(crate::ll::chtype, A_PROTECT);
	PCONSTU(crate::ll::chtype, A_ALTCHARSET);
	PCONSTU(crate::ll::chtype, A_ATTRIBUTES);
	PCONSTU(crate::ll::chtype, A_CHARTEXT);
	PCONSTU(crate::ll::chtype, A_COLOR);

	//do last, flush just to be sure!
	fflush(stdout);fflush(stderr);
}
