#/bin/bash
contest_id=$1
problem_id=$2
language=$3

TOKEN=`curl -b cookie https://atcoder.jp/login | grep -oP 'csrf_token" value=\"(.+=)\"' | sed -r 's/csrf_token\" value=\"(.+=)\"/\1/g' | sed -e 's/+/\+/g' | awk NR==1`
curl -v -i -b cookie https://atcoder.jp/contests/${contest_id}/submit -X POST -F "data.TaskScreenName=${problem_id}" -F "data.LanguageId=4050" -F "sourceCode=@abc275_a.rs" -F "csrf_token=${TOKEN}"
