#!/bin/python3

import os
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

new_filename = TARGET_DIR + name + ".rs"

if os.path.exists(new_filename):
    print("File already exists")
    exit(1)

with open(new_filename, "w") as f:
    f.write(
        f"""
// {SEARCH}{name.replace('_', '-')}/

struct Solution {{}}

impl Solution {
    {}
}

"""
    )
