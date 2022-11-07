#/bin/bash
contest_id=$1
problem_id=$2
language=$3
file=/home/nkpr/Desktop/rust/judge_tool/abc275_a.rs

TOKEN=`curl -b cookie https://atcoder.jp/login | grep -oP 'csrf_token" value=\"(.+=)\"' | sed -r 's/csrf_token\" value=\"(.+=)\"/\1/g' | sed -e 's/+/\+/g' | awk NR==1`
curl -i -b cookie https://atcoder.jp/contests/${contest_id}/submit -X POST -F "data.TaskScreenName=${problem_id}" -F "data.LanguageId=4050" -F "sourceCode=@${file}" -F "csrf_token=${TOKEN}"
