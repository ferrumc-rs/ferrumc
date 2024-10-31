import json

path = input("Enter the path to the file: ")
jsondata = ""
with open(path, "r") as f:
    jsondata = json.load(f)
with open(path, "w") as f:
    json.dump(jsondata, f, separators=(',', ':'))