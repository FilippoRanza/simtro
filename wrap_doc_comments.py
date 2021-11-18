#! /usr/bin/python

""" 
Wrap doc-comment lines in
Rust source code.
"""



import re
import pathlib
import os
import tempfile
import shutil


doc_comment = re.compile(r"(\s*)\/\/([\/|!])(.+)")

def wrap_comment(match):
    head = match.group(1)
  
    doc_type = match.group(2)
    words = match.group(3).split()
    count = 0
    output = ""
    output +=  head + "//" + doc_type + " "
    for word in words:
        if count + len(word) > 80:
            output += "\n"
            output += head + "//" + doc_type + " "
            count = len(word)
        else:
            count += len(word)

        output += word + " "
    output += "\n"
    return output

def process_line(line):
    if match := doc_comment.match(line):
        return wrap_comment(match)
    else:
        return line

def modify_inplace(file_name):
    with tempfile.TemporaryDirectory() as temp_dir:
        output_file_path = os.path.join(temp_dir, "file")
        with open(output_file_path, "w") as output_file:
            with open(file_name) as input_file:
                for line in input_file:
                    line = process_line(line)
                    print(line, end="", file=output_file)

        shutil.move(output_file_path, file_name)
    

os.chdir("src/")
for file in pathlib.Path().rglob("*.rs"):
    modify_inplace(file)
