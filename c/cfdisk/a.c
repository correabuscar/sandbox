/*
 * Copyright (C) 2010 Karel Zak <kzak@redhat.com>
 * Copyright (C) 2010 Davidlohr Bueso <dave@gnu.org>
 */

#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <ctype.h>
#include <errno.h>
#include <sys/stat.h>
#include <string.h>
#include <assert.h>

#define HAVE_LOCALE_H 1 //ye...
#include "nls.h"

//#include "/home/user/build/1packages/3rarelyused/util-linux/makepkg_pacman/util-linux/src/util-linux-2.33.2/lib/strutils.c"

//static int do_scale_by_power (uintmax_t *x, int base, int power)
//{
//	while (power--) {
//		if (UINTMAX_MAX / base < *x)
//			return -ERANGE;
//		*x *= base;
//	}
//	return 0;
//}
//
//int parse_size(const char *str, uintmax_t *res, int *power)
//{
//	char *p;
//	uintmax_t x, frac = 0;
//	int base = 1024, rc = 0, pwr = 0, frac_zeros = 0;
//
//	static const char *suf  = "KMGTPEZY";
//	static const char *suf2 = "kmgtpezy";
//	const char *sp;
//
//	*res = 0;
//
//	if (!str || !*str) {
//		rc = -EINVAL;
//		goto err;
//	}
//
//	/* Only positive numbers are acceptable
//	 *
//	 * Note that this check is not perfect, it would be better to
//	 * use lconv->negative_sign. But coreutils use the same solution,
//	 * so it's probably good enough...
//	 */
//	p = (char *) str;
//	while (isspace((unsigned char) *p))
//		p++;
//	if (*p == '-') {
//		rc = -EINVAL;
//		goto err;
//	}
//	p = NULL;
//
//	errno = 0;
//	x = strtoumax(str, &p, 0);
//
//	if (p == str ||
//	    (errno != 0 && (x == UINTMAX_MAX || x == 0))) {
//		rc = errno ? -errno : -EINVAL;
//		goto err;
//	}
//	if (!p || !*p)
//		goto done;			/* without suffix */
//
//	/*
//	 * Check size suffixes
//	 */
//check_suffix:
//	if (*(p + 1) == 'i' && (*(p + 2) == 'B' || *(p + 2) == 'b') && !*(p + 3))
//		base = 1024;			/* XiB, 2^N */
//	else if ((*(p + 1) == 'B' || *(p + 1) == 'b') && !*(p + 2))
//		base = 1000;			/* XB, 10^N */
//	else if (*(p + 1)) {
//		struct lconv const *l = localeconv();
//		char *dp = l ? l->decimal_point : NULL;
//		size_t dpsz = dp ? strlen(dp) : 0;
//    printf("dp=%p dp='%s' dpsz=%d\n", dp, dp, dpsz);
//
//		if (frac == 0 && *p && dp && strncmp(dp, p, dpsz) == 0) {
//      //printf("here\n");
//			char *fstr = p + dpsz;
//
//			for (p = fstr; *p && *p == '0'; p++)
//				frac_zeros++;
//			errno = 0, p = NULL;
//			frac = strtoumax(fstr, &p, 0);
//			if (p == fstr ||
//			    (errno != 0 && (frac == UINTMAX_MAX || frac == 0))) {
//				rc = errno ? -errno : -EINVAL;
//				//rc = errno ? -12 : -11;
//				goto err;
//			}
//			if (frac && (!p  || !*p)) {
//				rc = -EINVAL;
//				//rc = -11;
//				goto err;		/* without suffix, but with frac */
//			}
//			goto check_suffix;
//		}
//		rc = -EINVAL; //hit for 7.4G
//		//rc = -11;
//		goto err;			/* unexpected suffix */
//	}
//
//	sp = strchr(suf, *p);
//	if (sp)
//		pwr = (sp - suf) + 1;
//	else {
//		sp = strchr(suf2, *p);
//		if (sp)
//			pwr = (sp - suf2) + 1;
//		else {
//			rc = -EINVAL;
//			goto err;
//		}
//	}
//
//  printf("x=%ju base=%d frac=%d frac_zeros=%d\n",x,base, frac, frac_zeros);
//	rc = do_scale_by_power(&x, base, pwr);
//  printf("x=%ju base=%d frac=%d frac_zeros=%d\n",x,base, frac, frac_zeros);
////  if (rc < 0)
////    goto err;
//	if (power)
//		*power = pwr;
//	if (frac && pwr) {
//		int zeros_in_pwr = frac_zeros % 3;
//		int frac_pwr = pwr - (frac_zeros / 3) - 1;
//		uintmax_t y = frac * (zeros_in_pwr == 0 ? 100 :
//				      zeros_in_pwr == 1 ?  10 : 1);
//
//		if (frac_pwr < 0) {
//			rc = -EINVAL;
//			goto err;
//		}
//    printf("y=%ju base=%d frac=%d frac_pwr=%d frac_zeros=%d zeros_in_pwr=%d\n",y,base, frac, frac_pwr, frac_zeros, zeros_in_pwr);
//		do_scale_by_power(&y, base, frac_pwr);
//    printf("x=%ju y=%d\n",x,y);
//		if (frac >= 10) { //more than 1 fraction digits not supported eg. 7.4G ok, but 7.45G not good!
//			rc = -EINVAL;
//			goto err;
//		}
//		x += y;
//    printf("x=%ju y=%ju\n",x,y);
//	}
//done:
//	*res = x;
//  printf("res=%ju\n", *res);
//err:
//	if (rc < 0)
//		errno = -rc;
//	return rc;
//} 


static int do_scale_by_power (uintmax_t *x, int base, int power)
{
	while (power--) {
		if (UINTMAX_MAX / base < *x)
			return -ERANGE;
		*x *= base;
	}
	return 0;
}

/*
 * strtosize() - convert string to size (uintmax_t).
 *
 * Supported suffixes:
 *
 * XiB or X for 2^N
 *     where X = {K,M,G,T,P,E,Z,Y}
 *        or X = {k,m,g,t,p,e}  (undocumented for backward compatibility only)
 * for example:
 *		10KiB	= 10240
 *		10K	= 10240
 *
 * XB for 10^N
 *     where X = {K,M,G,T,P,E,Z,Y}
 * for example:
 *		10KB	= 10000
 *
 * The optional 'power' variable returns number associated with used suffix
 * {K,M,G,T,P,E,Z,Y}  = {1,2,3,4,5,6,7,8}.
 *
 * The function also supports decimal point, for example:
 *              0.5MB   = 500000
 *              0.5MiB  = 512000
 *
 * Note that the function does not accept numbers with '-' (negative sign)
 * prefix.
 */
int parse_size(const char *str, uintmax_t *res, int *power)
{
	const char *p;
	char *end;
	uintmax_t x, frac = 0;
	int base = 1024, rc = 0, pwr = 0, frac_zeros = 0;

	static const char *suf  = "KMGTPEZY";
	static const char *suf2 = "kmgtpezy";
	const char *sp;

	*res = 0;

	if (!str || !*str) {
		rc = -EINVAL;
		goto err;
	}

	/* Only positive numbers are acceptable
	 *
	 * Note that this check is not perfect, it would be better to
	 * use lconv->negative_sign. But coreutils use the same solution,
	 * so it's probably good enough...
	 */
	p = str;
	while (isspace((unsigned char) *p))
		p++;
	if (*p == '-') {
		rc = -EINVAL;
		goto err;
	}

	errno = 0, end = NULL;
	x = strtoumax(str, &end, 0);

	if (end == str ||
	    (errno != 0 && (x == UINTMAX_MAX || x == 0))) {
		rc = errno ? -errno : -EINVAL;
		goto err;
	}
	if (!end || !*end)
		goto done;			/* without suffix */
	p = end;

	/*
	 * Check size suffixes
	 */
check_suffix:
	if (*(p + 1) == 'i' && (*(p + 2) == 'B' || *(p + 2) == 'b') && !*(p + 3))
		base = 1024;			/* XiB, 2^N */
	else if ((*(p + 1) == 'B' || *(p + 1) == 'b') && !*(p + 2))
		base = 1000;			/* XB, 10^N */
	else if (*(p + 1)) {
		struct lconv const *l = localeconv();
		const char *dp = l ? l->decimal_point : NULL;
		size_t dpsz = dp ? strlen(dp) : 0;

		if (frac == 0 && *p && dp && strncmp(dp, p, dpsz) == 0) {
			const char *fstr = p + dpsz;

			for (p = fstr; *p == '0'; p++)
				frac_zeros++;
			errno = 0, end = NULL;
			frac = strtoumax(fstr, &end, 0);
			if (end == fstr ||
			    (errno != 0 && (frac == UINTMAX_MAX || frac == 0))) {
				rc = errno ? -errno : -EINVAL;
				goto err;
			}
			if (frac && (!end  || !*end)) {
				rc = -EINVAL;
				goto err;		/* without suffix, but with frac */
			}
			p = end;
			goto check_suffix;
		}
		rc = -EINVAL;
		goto err;			/* unexpected suffix */
	}

	sp = strchr(suf, *p);
	if (sp)
		pwr = (sp - suf) + 1;
	else {
		sp = strchr(suf2, *p);
		if (sp)
			pwr = (sp - suf2) + 1;
		else {
			rc = -EINVAL;
			goto err;
		}
	}

	rc = do_scale_by_power(&x, base, pwr);
	if (power)
		*power = pwr;
	if (frac && pwr) {
		int zeros_in_pwr = frac_zeros % 3;
		int frac_pwr = pwr - (frac_zeros / 3) - 1;
		uintmax_t y = frac * (zeros_in_pwr == 0 ? 100 :
				      zeros_in_pwr == 1 ?  10 : 1);

		if (frac_pwr < 0) {
			rc = -EINVAL;
			goto err;
		}
		do_scale_by_power(&y, base, frac_pwr);
    if (frac >= 10) {
			rc = -EINVAL;
      goto err;
    }
		x += y;
	}
done:
	*res = x;
err:
	if (rc < 0)
		errno = -rc;
	return rc;
}

int main(int argc, char *argv[]) {
  int pwr=0;
  uintmax_t user = UINTMAX_MAX;
  ssize_t rc=0;
  //rc=parse_size("1.1111111KB", &user, &pwr); // 111112100
  rc=parse_size("7.16G", &user, &pwr); // 9193914368
  //rc=parse_size("7.1G", &user, &pwr); // 7621050368
  //rc=parse_size("1.023K", &user, &pwr); // 1214
  //rc=parse_size("1.0123K", &user, &pwr); // 1854
  //rc=parse_size("1.0234K", &user, &pwr); // 2584
  //rc=parse_size("1.234K", &user, &pwr); // 24424
  //rc=parse_size("1.23K", &user, &pwr); // 3324
  printf("rc=%zd user=%ju power=%d -EINVAL=%d UINTMAX_MAX=%u\n", rc, user, pwr,-EINVAL, UINTMAX_MAX);
}
