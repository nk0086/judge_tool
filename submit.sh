#/bin/bash
contest_id=$1
problem_id=$2
language=$3

TOKEN=`curl -sS -b cookie https://atcoder.jp/login | grep -oP 'csrf_token" value=\"(.+=)\"' | sed -r 's/csrf_token\" value=\"(.+=)\"/\1/g' | sed -e 's/+/\+/g' | awk NR==1`
curl -sS -b cookie https://atcoder.jp/contests/${contest_id}/submit -X POST -F "TaskScreenName=${problem_id}" -F "LanguageId=4050" -F "sourceCode=</home/nkpr/Desktop/rust/judge_tool/${problem_id}.${language}" -F "csrf_token=${TOKEN}"

