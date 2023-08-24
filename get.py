#!/bin/python3

import sys

URL = sys.argv[1]

SEARCH = "https://leetcode.com/problems/"
TARGET_DIR = "./src/"

if not URL.startswith(SEARCH):
    print("Invalid URL")
    exit(1)

tmp = URL[len(SEARCH) :]
tmp = tmp[: tmp.index("/")]
name = tmp.replace("-", "_")

with open(TARGET_DIR + name + ".rs", "w") as f:
    f.write(
        f"""
// {SEARCH}{name.replace('_', '-')}/

struct Solution {{}}

impl Solution {
    {}
}

"""
    )
