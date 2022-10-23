//src: https://www.gnu.org/software/libc/manual/html_node/Syslog-Example.html
#include <syslog.h>
#include <unistd.h> // for getuid()
#include <pwd.h>

inline const char *getUserName(const uid_t uid)  //src: https://stackoverflow.com/a/8953445
{
  //uid_t uid = geteuid();
  struct passwd *pw = getpwuid(uid);
  if (pw)
  {
    return pw->pw_name; //return ptr to string; Do not free() this!
  }

  return NULL; //when printed shows "(null)" (as a string) so we could return this!
}

int main() {
	syslog(LOG_INFO, "A tree falls in a forest before openlog() was called!");//shown only on syslog ie. dmesg
	//setlogmask(LOG_UPTO (LOG_INFO));

	openlog("exampleprog", LOG_CONS | LOG_PID | LOG_NDELAY | LOG_PERROR, 
      LOG_LOCAL1);

  uid_t guid=getuid();
  uid_t geuid=geteuid();
  extern const char *__progname;
  extern const char *__progname_full;
  const pid_t cp_pid=getpid();
	syslog(LOG_NOTICE, "Program(%s[%d](full:%s) started by User %s(%d(e:%s(%d))) \x1b[41mred string\x1b(B\x1b[mnormal string", //from `tput setab 1` and `tput sgr0`
      __progname,
      cp_pid,
      __progname_full,
      getUserName(guid),
      guid,
      getUserName(geuid),
      geuid
      );
	syslog(LOG_INFO, "A tree falls in a forest");

	closelog();
	syslog(LOG_INFO, "A tree falls in a forest after closelog() was called!");
}
