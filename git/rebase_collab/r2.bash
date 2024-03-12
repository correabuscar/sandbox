#!/bin/bash

set -xev

pushd /tmp

rm -rf gitserver
mkdir -p gitserver/somerepo
pushd gitserver/somerepo
  git init --bare -b main
  repourl="/tmp/gitserver/somerepo"
popd

rm -rf user1
mkdir user1
pushd user1
  git clone -- "$repourl"
  pushd somerepo
    git config --local --add user.name "user1"
    git config --local --add user.email "user1@example.com"
    echo "user1mod1 on main branch" > user1file.txt
    git add user1file.txt
    git commit -a -m "user1 commit1 on main"

    git checkout -b feature_branch
    git push --all
  popd
popd

rm -rf user2
mkdir user2
pushd user2
  git clone -- "$repourl"
  pushd somerepo
  sleep 1
    git config --local --add user.name "user2"
    git config --local --add user.email "user2@example.com"

    git checkout feature_branch
    echo "user2 mod1 on feature branch" > user2file_on_feature.txt
    git add user2file_on_feature.txt
    git commit -a -m "user2 commit1 on feature branch"
    git push
  popd

popd

rm -rf user3
mkdir user3
pushd user3
  git clone -- "$repourl"
  pushd somerepo
  sleep 1
    git config --local --add user.name "user3"
    git config --local --add user.email "user3@example.com"

    echo "user3mod1 on main branch" > user3file.txt
    git add user3file.txt
    git commit -a -m "user3 commit1 on main"
    git push

    git checkout feature_branch
    git pull
    echo "user3mod2 on feature branch" > user3file_on_feature.txt
    git add user3file_on_feature.txt
    git commit -a -m "user3 commit1 on feature branch"
    git rebase main
    #git push #fail
    #git pull #fail
    git rebase
    git push #works

  popd
popd

pushd user2
  sleep 1
  pushd somerepo
    git pull
    echo "user2 mod2 on feature branch" >> user2file_on_feature.txt
    #git add user2file_on_feature.txt
    git commit -a -m "user2 commit2 on feature branch"
    git push
    sed -e 's/user2 mod2 on feature branch/user2 mod2 on feature brancH/' -i user2file_on_feature.txt
    git commit -a -m "user2 commit3 on feature branch"

  popd
popd

pushd user3
  sleep 1
  pushd somerepo
    git pull
    echo "user3 mod3 on feature branch" >> user3file_on_feature.txt
#    echo "user3 mod3 on feature branch" >> user2file_on_feature.txt
#    sed -e 's/user2 mod2 on feature branc/uSer2 mod2 on feature branc/' -i user2file_on_feature.txt
    #git add user2file_on_feature.txt
    git commit -a -m "user3 commit2 on feature branch"
    git checkout main
    git pull
    git checkout feature_branch
    git rebase main
    git rebase
    git push
  popd
popd

pushd user2
  sleep 1
  pushd somerepo
    #git pull #fail
    git fetch
    git rebase
    git push
  popd
popd

popd
