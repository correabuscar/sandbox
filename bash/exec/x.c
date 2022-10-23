//src https://stackoverflow.com/a/3027353

#include <unistd.h>
int main(void)
{
    char *args[3] = { "rip van winkle", "30", 0 };
    //execv("/usr/bin/sleep", args);//it's 'sleep' (as reported by 'ps'), sleep is an ELF
    //execv("./bash.script", args);//yep it's './bash.script' aka still broken! bash.script is a shebang script! (as reported by 'ps' AND $0 within the bash script itself!)
    execv("./2.out", args);//well this works, shows "rip van winkle"; 2.out is an ELF; however 'ps' still reports it as '2.out' (no "./" preceeding it though)
    return 1;
}
