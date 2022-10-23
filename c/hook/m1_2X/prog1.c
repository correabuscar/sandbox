//src: 2011 https://www.opensourceforu.com/2011/08/lets-hook-a-library-function/
#include<stdio.h>
#include<malloc.h>
#include<stdlib.h>
#include <X11/Xlib.h>
#include <X11/Xproto.h>
#include <X11/Xatom.h>
#include <X11/extensions/Xrandr.h>

#include <assert.h>     /* assert */

int main(void)
{
	//int *p;
	/*p=(int *)malloc(10);
	if(!p)
	{
		printf("Got allocation error...\n");
		exit(1);
	}*/
    // The X version:
  Display *dpy = XOpenDisplay(NULL);
  if (NULL == dpy) {
    fprintf(stderr, "X is not running? or cannot open a connection to it. Is DISPLAY env var set?\n");
    return 1;
  }
  int screen = DefaultScreen(dpy);
  Window root = RootWindow(dpy, screen);

  XRRScreenResources *res = XRRGetScreenResourcesCurrent(dpy, root); // available in RandR 1.3 or higher
  int num_crtcs = res->ncrtc;
  //gamma values for 3000 Kelvin hardcoded:
  double gamma_r=1.0;
  double gamma_g=0.6949030005552019;
  double gamma_b=0.4310480202110507;
  for (int c = 0; c < num_crtcs; c++) {
    assert(res->ncrtc == num_crtcs); //doneTODO: find out if this can change by the below XRRGetCrtcInfo call! it can't, only resources->configTimestamp is accessed in XRRGetCrtcInfo as per src/XrrCrtc.c
    int crtcxid = res->crtcs[c];
    // //XRRCrtcInfo *crtc_info =  //no point in saving this?!
    //   XRRGetCrtcInfo(dpy, res, crtcxid); //wait, is this useless? seems so
    //   XRRFreeCrtcInfo(crtc_info); //should free it eventually

    int size = XRRGetCrtcGammaSize(dpy, crtcxid);
    XRRCrtcGamma *crtc_gamma = XRRAllocGamma(size);

    for (int i = 0; i < size; i++) {
      double g = 65535.0 * i / size;
      crtc_gamma->red[i]   = g * gamma_r;
      crtc_gamma->green[i] = g * gamma_g;
      crtc_gamma->blue[i]  = g * gamma_b;
    }
    printf("calling from main...\n"); // this calls malloc internally!
    XRRSetCrtcGamma(dpy, crtcxid, crtc_gamma);
    printf("returning to main...\n");
    XFree(crtc_gamma);
  }

	//free(p);                           /* freeing memory from heap */
	//printf("freeing memory...\n");
	return 0;
}
