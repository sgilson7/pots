"""Inserts objects into a JSON list by text, after the object whose "id" is
given, so the rest of the file keeps its formatting. Asserts it matched and
that the result still parses.

    from jsoninsert import insert_after
"""
import json
import re


def insert_after(path, list_key, after_id, objs, depth=2):
    s = open(path).read()
    data = json.loads(s)
    ids = [o.get("id") for o in data[list_key]]
    assert after_id in ids, after_id
    for o in objs:
        assert o["id"] not in ids, o["id"]
    pad = " " * depth
    m = re.search(r'\n' + pad + r'\{\n' + pad + r' "id": "' + re.escape(after_id) + r'",', s)
    assert m, after_id
    # the end of that object: the next line at the same depth closing a brace
    end = s.index("\n" + pad + "}", m.end()) + len(pad) + 2
    text = "".join(",\n" + "\n".join(pad + line for line in json.dumps(o, indent=1, ensure_ascii=False).split("\n")) for o in objs)
    s = s[:end] + text + s[end:]
    json.loads(s)
    open(path, "w").write(s)
