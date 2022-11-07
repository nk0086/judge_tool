#!/bin/bash
user=$1
pass=$2

TOKEN=`curl -c cookie https://atcoder.jp/login | grep -oP 'csrf_token" value=\"(.+=)\"' | sed -r 's/csrf_token\" value=\"(.+=)\"/\1/g' | sed -e 's/+/\+/g' | awk NR==1`
curl -i -b cookie https://atcoder.jp/login -X POST -F "csrf_token=${TOKEN}" -F "username=${user}" -F "password=${pass}"
