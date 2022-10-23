#!/bin/bash

#for bug: https://bugs.gentoo.org/759067

#XXX this test assumes 'portage' was already emerged once and a binpkg was thus created already for it! and make.conf's "FEATURES=getbinpkg" already!
#test takes this much time:
#real	1m12.988s
#user	1m9.876s
#sys	0m3.158s

#Note a copy of your existing make.conf and a couple of others will be done so that FEATURES=getbinpkg can be changed on disk to -getbinpkg! and then tested!
CFGROOT="/tmp/" #--config-root= arg to "emerge" command!

# nothing to change below:

in1=(
  -getbinpkg
  getbinpkg
  ""
)
in2=(
  --getbinpkg
  --getbinpkg=True
  --getbinpkg=y
  --getbinpkg=n
  ""
)
expectedout1=(
  binary
  binary
  binary
  ebuild
  ebuild
  binary
  binary
  binary
  ebuild
  binary
  binary
  binary
  binary
  ebuild
  binary
  )

expectedout2=(
  binary
  binary
  binary
  ebuild
  ebuild
  binary
  binary
  binary
  ebuild
  binary
  binary
  binary
  binary
  ebuild
  ebuild
  )


  set -e
  cd "/tmp" #keep this /tmp! it's a safety feature to avoid overwriting!
  test -n "$CFGROOT"
  set +e
  mkdir -p "${CFGROOT}etc/portage/"
  cp -a /etc/portage/binrepos.conf "${CFGROOT}etc/portage/binrepos.conf"
  ln -fLs -T -- /etc/portage/make.profile "${CFGROOT}etc/portage/make.profile"
  cp -a /etc/portage/make.conf "${CFGROOT}etc/portage/make.conf"
  cp -a /etc/portage/package.license "${CFGROOT}etc/portage/package.license"

  red="$(tput setab 1)"
  green="$(tput setaf 2)"
  reset="$(tput sgr0)"
  let 'errs=0'

dotest() {
  local out=("$@")
  #mock=()
  local pos
  let 'pos=0'
  for i1 in "${in1[@]}"; do
    for i2 in "${in2[@]}"; do
      #cmd=( env EMERGE_DEFAULT_OPTS= FEATURES="$i1" emerge "$i2" --pretend --ask=n --quiet -- portage \| cut -f1 -d' ' \| tr -d '[ \n' )
      #echo "${#cmd[@]}=!${cmd[@]}!"
      #o+=( "$("${cmd[@]}")" )
      local outnow
      outnow="$(env EMERGE_DEFAULT_OPTS= FEATURES="$i1" emerge $i2 --config-root="${CFGROOT}" --pretend --ask=n --quiet -- portage | cut -f1 -d' ' | tr -d '[ \n' )"
      #mock+=("$outnow")
      local expected
      expected="${out[pos]}"
      if test "${expected}" != "$outnow"; then
        echo "${red}fail${reset}: expected '$expected' got '$outnow' for 'FEATURES=$i1' and emerge arg:'$i2'"
        let 'errs++'
      else
        echo "${green}good${reset}: expected '$expected' got '$outnow' for 'FEATURES=$i1' and emerge arg:'$i2'"
      fi
      let 'pos++'
    done
  done
}

  sed '/^FEATURES=/ s/-getbinpkg/getbinpkg/' -i /tmp/etc/portage/make.conf
  dotest "${expectedout1[@]}"
  sed '/^FEATURES=/ s/getbinpkg/-getbinpkg/' -i /tmp/etc/portage/make.conf
  dotest "${expectedout2[@]}"

#echo "${mock[@]}"

if test "$errs" == "0"; then
  echo "All ok."
else
  echo "${red}${errs} errors!${reset}"
fi

#sample output:
# time ./test_getbinpkg.bash
#good: expected 'binary' got 'binary' for 'FEATURES=-getbinpkg' and emerge arg:'--getbinpkg'
#good: expected 'binary' got 'binary' for 'FEATURES=-getbinpkg' and emerge arg:'--getbinpkg=True'
#good: expected 'binary' got 'binary' for 'FEATURES=-getbinpkg' and emerge arg:'--getbinpkg=y'
#good: expected 'ebuild' got 'ebuild' for 'FEATURES=-getbinpkg' and emerge arg:'--getbinpkg=n'
#good: expected 'ebuild' got 'ebuild' for 'FEATURES=-getbinpkg' and emerge arg:''
#good: expected 'binary' got 'binary' for 'FEATURES=getbinpkg' and emerge arg:'--getbinpkg'
#good: expected 'binary' got 'binary' for 'FEATURES=getbinpkg' and emerge arg:'--getbinpkg=True'
#good: expected 'binary' got 'binary' for 'FEATURES=getbinpkg' and emerge arg:'--getbinpkg=y'
#good: expected 'ebuild' got 'ebuild' for 'FEATURES=getbinpkg' and emerge arg:'--getbinpkg=n'
#good: expected 'binary' got 'binary' for 'FEATURES=getbinpkg' and emerge arg:''
#good: expected 'binary' got 'binary' for 'FEATURES=' and emerge arg:'--getbinpkg'
#good: expected 'binary' got 'binary' for 'FEATURES=' and emerge arg:'--getbinpkg=True'
#good: expected 'binary' got 'binary' for 'FEATURES=' and emerge arg:'--getbinpkg=y'
#good: expected 'ebuild' got 'ebuild' for 'FEATURES=' and emerge arg:'--getbinpkg=n'
#good: expected 'binary' got 'binary' for 'FEATURES=' and emerge arg:''
#good: expected 'binary' got 'binary' for 'FEATURES=-getbinpkg' and emerge arg:'--getbinpkg'
#good: expected 'binary' got 'binary' for 'FEATURES=-getbinpkg' and emerge arg:'--getbinpkg=True'
#good: expected 'binary' got 'binary' for 'FEATURES=-getbinpkg' and emerge arg:'--getbinpkg=y'
#good: expected 'ebuild' got 'ebuild' for 'FEATURES=-getbinpkg' and emerge arg:'--getbinpkg=n'
#good: expected 'ebuild' got 'ebuild' for 'FEATURES=-getbinpkg' and emerge arg:''
#good: expected 'binary' got 'binary' for 'FEATURES=getbinpkg' and emerge arg:'--getbinpkg'
#good: expected 'binary' got 'binary' for 'FEATURES=getbinpkg' and emerge arg:'--getbinpkg=True'
#good: expected 'binary' got 'binary' for 'FEATURES=getbinpkg' and emerge arg:'--getbinpkg=y'
#good: expected 'ebuild' got 'ebuild' for 'FEATURES=getbinpkg' and emerge arg:'--getbinpkg=n'
#good: expected 'binary' got 'binary' for 'FEATURES=getbinpkg' and emerge arg:''
#good: expected 'binary' got 'binary' for 'FEATURES=' and emerge arg:'--getbinpkg'
#good: expected 'binary' got 'binary' for 'FEATURES=' and emerge arg:'--getbinpkg=True'
#good: expected 'binary' got 'binary' for 'FEATURES=' and emerge arg:'--getbinpkg=y'
#good: expected 'ebuild' got 'ebuild' for 'FEATURES=' and emerge arg:'--getbinpkg=n'
#good: expected 'ebuild' got 'ebuild' for 'FEATURES=' and emerge arg:''
#All ok.
#
#real	1m13.081s
#user	1m10.068s
#sys	0m3.091s

