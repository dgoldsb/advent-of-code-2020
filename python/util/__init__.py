import os
import sys


with open("session_cookie") as f:
    session = f.readline().strip()

os.environ["AOC_SESSION"] = session
sys.setrecursionlimit(1000)
