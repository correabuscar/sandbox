//based on, src: https://www.opensourceforu.com/2011/08/lets-hook-a-library-function/
//#define NOSTACK_internal //define to not print stacktraces! comment out to get stacktraces for the calls to the hooked functions!
#define LOGFILE_internal "/tmp/gamma_setters.log" //comment out to use stderr !

#define _GNU_SOURCE
#include <stdio.h>
#include <stdint.h>
#include <dlfcn.h>                               /* header required for dlsym() */
#include <stdbool.h>


// backtrace code src: https://www.gnu.org/software/libc/manual/html_node/Backtraces.html
// backtrace code compile with: gcc -D_FORTIFY_SOURCE=2 backtrace.c -ggdb -O1 -rdynamic
#include <execinfo.h>
//#include <stdio.h>
#include <stdlib.h>

#include <X11/extensions/Xrandr.h> //for XRRSetCrtcGamma's args' types! TODO; include only the needed .h or even just copy/paste the type?(hard due to any ifdefs when defining types, I imagine) note that headers aren't needed at all, just the function signature(and the types it references) but can do this without including headers.

#include <unistd.h> //page size, close()
#include <fcntl.h> //open()

#include <X11/extensions/xf86vmode.h>

//TODO: there are more gamma setting functions to hook!

#include <xcb/randr.h> // xcb_randr_set_crtc_gamma_checked(), xcb_randr_crtc_t
#include <xcb/xcb.h> // xcb_void_cookie_t

#ifdef LOGFILE_internal
//use log file
#define LE_OUT_HANDLE_internal logf
#define FCLOSE_internal \
      if (NULL!=LE_OUT_HANDLE_internal) {\
        fclose(LE_OUT_HANDLE_internal); \
        LE_OUT_HANDLE_internal=NULL; \
      }

#define FOPENLOG_internal \
      FILE *LE_OUT_HANDLE_internal = fopen(LOGFILE_internal,"a"); \
      if (NULL==LE_OUT_HANDLE_internal){ \
        LE_OUT_HANDLE_internal=stderr; \
      }
#else
//if not defined, use standard error output
#define LE_OUT_HANDLE_internal stderr
#define FCLOSE_internal
#define FOPENLOG_internal
#endif
//extern void *__libc_dlsym (void *, const char *);
/* lcheck() is for memory leak check; its code is not shown
 here */
//void lcheck(void);
void __internalshie_print_trace(void); //renamed to prevent overloading any actual such function!
void __internalshie_show_current_process(const char *func_name); //renamed to prevent overloading any actual such function!

//function xcb_randr_set_crtc_gamma_checked is used by x11-misc/redshift: /usr/bin/redshift -m randr
xcb_void_cookie_t xcb_randr_set_crtc_gamma_checked (xcb_connection_t *c, xcb_randr_crtc_t  crtc, uint16_t size, const uint16_t *red, const uint16_t *green, const uint16_t *blue) {
  static xcb_void_cookie_t (*my_xcb_randr_set_crtc_gamma_checked)(xcb_connection_t *c, xcb_randr_crtc_t  crtc, uint16_t          size, const uint16_t   *red, const uint16_t     *green, const uint16_t   *blue)=NULL;
  if (NULL==my_xcb_randr_set_crtc_gamma_checked) {
    my_xcb_randr_set_crtc_gamma_checked=dlsym(RTLD_NEXT, "xcb_randr_set_crtc_gamma_checked");
    if (NULL==my_xcb_randr_set_crtc_gamma_checked) {
      FOPENLOG_internal
#ifndef LOGFILE_internal
      fflush(stdout);
#endif
      fprintf(LE_OUT_HANDLE_internal,"!! EPIC fail for function '%s'.\n",__func__);
      fflush(LE_OUT_HANDLE_internal);
      FCLOSE_internal
      static xcb_void_cookie_t fixme; //FIXME:
      return fixme;
    }
  }
  __internalshie_show_current_process(__func__);
  __internalshie_print_trace();
  return my_xcb_randr_set_crtc_gamma_checked(c,crtc,size,red,green,blue);
}
//function drmModeCrtcSetGamma is used by x11-misc/redshift: /usr/bin/redshift -m drm
int drmModeCrtcSetGamma(int fd, uint32_t crtc_id, uint32_t size, uint16_t *red, uint16_t *green, uint16_t *blue){
  static int (*my_drmModeCrtcSetGamma)(int fd, uint32_t crtc_id, uint32_t size, uint16_t *red, uint16_t *green, uint16_t *blue)=NULL;
  if (NULL==my_drmModeCrtcSetGamma) {
    my_drmModeCrtcSetGamma=dlsym(RTLD_NEXT, "drmModeCrtcSetGamma");
    if (NULL==my_drmModeCrtcSetGamma) {
      FOPENLOG_internal
#ifndef LOGFILE_internal
      fflush(stdout);
#endif
      fprintf(LE_OUT_HANDLE_internal,"!! EPIC fail for function '%s'.\n",__func__);
      fflush(LE_OUT_HANDLE_internal);
      FCLOSE_internal
      return 0; //FIXME:
    }
  }
  __internalshie_show_current_process(__func__);
  __internalshie_print_trace();
  return my_drmModeCrtcSetGamma(fd,crtc_id,size,red,green,blue);
}
//function XF86VidModeSetGammaRamp from xf86vmode.h it's used by x11-misc/redshift: /usr/bin/redshift -m vidmode
Bool XF86VidModeSetGammaRamp(
      Display*                    dpy,
      int                         screen,
      int       size,
      unsigned short*             red_array,
      unsigned short*             green_array,
      unsigned short*             blue_array
  ) {
  static Bool (*my_XF86VidModeSetGammaRamp)(Display* dpy, int screen, int size, unsigned short*, unsigned short*,unsigned short*)=NULL;
  if (NULL==my_XF86VidModeSetGammaRamp) {
    my_XF86VidModeSetGammaRamp=dlsym(RTLD_NEXT, "XF86VidModeSetGammaRamp");
    if (NULL==my_XF86VidModeSetGammaRamp) {
      FOPENLOG_internal
#ifndef LOGFILE_internal
      fflush(stdout);
#endif
      fprintf(LE_OUT_HANDLE_internal,"!! EPIC fail for function '%s'.\n",__func__);
      fflush(LE_OUT_HANDLE_internal);
      FCLOSE_internal
      return false;
    }
  }
  __internalshie_show_current_process(__func__);
  __internalshie_print_trace();
  return my_XF86VidModeSetGammaRamp(dpy,screen,size,red_array,green_array,blue_array);
}

//typedef struct {
//    float red;            /* Red Gamma value */
//    float green;        /* Green Gamma value */
//    float blue;            /* Blue Gamma value */
//} XF86VidModeGamma;
//function used by x11-apps/xgamma: /usr/bin/xgamma
Bool XF86VidModeSetGamma(Display *display, int screen, XF86VidModeGamma *Gamma) {
  static Bool (*my_XF86VidModeSetGamma)(Display *display, int screen, XF86VidModeGamma *Gamma)=NULL;
  if (NULL==my_XF86VidModeSetGamma) {
    my_XF86VidModeSetGamma=dlsym(RTLD_NEXT, "XF86VidModeSetGamma");
    if (NULL==my_XF86VidModeSetGamma) {
      FOPENLOG_internal
#ifndef LOGFILE_internal
      fflush(stdout);
#endif
      fprintf(LE_OUT_HANDLE_internal,"!! EPIC fail for function '%s'.\n",__func__);
      fflush(LE_OUT_HANDLE_internal);
      FCLOSE_internal
      return false;
    }
  }
  __internalshie_show_current_process(__func__);
  __internalshie_print_trace();
  return my_XF86VidModeSetGamma(display,screen,Gamma);
}

//function used by secote, at least. https://github.com/temporaryrespite/secote
void XRRSetCrtcGamma (Display *dpy, RRCrtc crtc, XRRCrtcGamma *gamma) {
  static void (*my_XRRSetCrtcGamma)(Display *dpy, RRCrtc crtc, XRRCrtcGamma *gamma)=NULL;
  static bool already_in=false; //was only needed for when hooking malloc() to prevent recursion! for XRRSetCrtcGamma isn't needed! FIXME: remove
  if (!my_XRRSetCrtcGamma) {
    already_in=true;
    my_XRRSetCrtcGamma=dlsym(RTLD_NEXT, "XRRSetCrtcGamma");
    already_in=false;
    if (NULL==my_XRRSetCrtcGamma) {
      already_in=true;
      FOPENLOG_internal
#ifndef LOGFILE_internal
      fflush(stdout);
#endif
      fprintf(LE_OUT_HANDLE_internal,"!! EPIC fail for function '%s'.\n",__func__);
      fflush(LE_OUT_HANDLE_internal);
      FCLOSE_internal
      already_in=false;
      return;
    }
  }
  if (!already_in) {
    already_in=true;
    __internalshie_show_current_process(__func__);
    __internalshie_print_trace();
    already_in=false;
  }
  my_XRRSetCrtcGamma(dpy,crtc,gamma);
  return;
}

//void* malloc(size_t size)
//{
//	static void* (*my_malloc)(size_t) = NULL;
//  static bool already_in=false;
//	if (!my_malloc) {
//    already_in=true;
//    my_malloc = dlsym(RTLD_NEXT, "malloc");  /* returns the object reference for malloc */
//    //my_malloc = __libc_dlsym(RTLD_NEXT, "malloc");  /* returns the object reference for malloc */
//    already_in=false;
//    if (NULL==my_malloc) {
//      already_in=true;
//      printf("!! EPIC fail...\n");
//      already_in=false;
//      return NULL;
//    }
//    //my_malloc = (void * (*)(size_t))dlsym(RTLD_NEXT, "malloc");  /* returns the object reference for malloc */
//  }
//  if (!already_in) {
//    already_in=true;
//    printf("!! inside shared object...\n");
//    fflush(stdout);
//    already_in=false;
//  }
//	void *p = my_malloc(size);               /* call malloc() using function pointer my_malloc */  
//  if (!already_in) {
//    already_in=true;
//    printf("!! malloc(%ld) = %p\n", size, p);
//    fflush(stdout);
//    __internalshie_print_trace();
//    //lcheck();                                /* calling do_your_stuff function */
//    printf("!! returning from shared object...\n");
//    fflush(stdout);
//    already_in=false;
//  }
//  //already_in=false; // stack trace recursion was only needed because this was uncommented!
//	return p;
//}
//void lcheck(void)
//{
//	printf("!! displaying memory leaks...\n");
//  fflush(stdout);
//	/* do required stuff here */
//}


//used sources for this function:
//https://stackoverflow.com/questions/24127416/parsing-command-line-arguments-from-proc-pid-cmdline/24128544#24128544
//https://stackoverflow.com/questions/1563168/example-of-realpath-function-in-c/1563237#1563237
void __internalshie_show_current_process(const char *func_name) {
  FOPENLOG_internal
  {
    char *rp=realpath("/proc/self/exe", NULL/*aka malloc() for me*/);
    if (NULL != rp) {
      fprintf(LE_OUT_HANDLE_internal,"!! The program '%s' was called as: ",rp);
      free(rp);
    } else {
      perror("!! realpath");//FIXME: this should use LE_OUT_HANDLE_internal ie. the log file or stderr, now using only stderr!
    }
  }
  const long PAGESIZE = sysconf(_SC_PAGESIZE); //ie. 4096, run at prompt: $ getconf PAGESIZE  (getconf is part of sys-libs/glibc, on Gentoo)
  const long BUFSIZE = PAGESIZE;
  unsigned char buffer[BUFSIZE];//FIXME: hmm, using 4k of stack?
  int fd = open("/proc/self/cmdline", O_RDONLY);
  int nbytesread = read(fd, buffer, BUFSIZE);
	unsigned char *end = buffer + nbytesread;
  //fprintf(LE_OUT_HANDLE_internal,"!! ");
	for (unsigned char *p = buffer; p < end; /**/)
	{
		fprintf(LE_OUT_HANDLE_internal,"'%s' ",p);
		while (*p++); // skip until start of next 0-terminated section
	}
  fprintf(LE_OUT_HANDLE_internal,"and is using function '%s'\n",func_name);//don't erase last space char and add new line after this text
  fflush(LE_OUT_HANDLE_internal);
  FCLOSE_internal
  close(fd);
}

/* Obtain a backtrace and print it to stdout. */
void
__internalshie_print_trace (void)
{
#if defined NOSTACK_internal
  return;
#endif
  FOPENLOG_internal
#define MAX_LINES_IN_STACKDUMP 100
  //static bool intrace=false;
  void *array[MAX_LINES_IN_STACKDUMP];
  size_t size;
  char **strings;
  size_t i;

//  if (!intrace) {
//    intrace=true;

    size = backtrace (array, MAX_LINES_IN_STACKDUMP);
    strings = backtrace_symbols (array, size);

    fflush(stdout);
    fprintf (LE_OUT_HANDLE_internal,"!! Obtained %zd stack frames.\n", size);
    fflush(LE_OUT_HANDLE_internal);

    for (i = 0; i < size; i++)
      fprintf (LE_OUT_HANDLE_internal,"!! %s\n", strings[i]);

    fflush(LE_OUT_HANDLE_internal);
    free (strings);
//    intrace=false;
//  } else {
//    printf ("!! stacktrace recursion prevented.\n");
//    fflush(stdout);
//  }
  FCLOSE_internal
}
