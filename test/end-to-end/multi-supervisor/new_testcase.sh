#!/bin/bash

testcase=${1}
testcase_dir="testcases/${testcase}"

if [ -d "${testcase_dir}" ]; then
    echo "Test case '${testcase}' already exists!"
    exit 1
else
    mkdir "${testcase_dir}"
    cp defaults/* "${testcase_dir}"
    echo "A new (failing) testcase created in ${testcase_dir}"
fi
