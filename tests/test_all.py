import subprocess

cvto = "../target/debug/cvto"

def run_cmd(cmd, input_data=None): 
    result = subprocess.run(cmd, capture_output=True, input=input_data)
    assert result.returncode == 0
    return result.stdout.decode()

def test_json_to_json():
    result = run_cmd([cvto, "-i", "resources/a.json", "--out-format", "json"])
    assert result == """{
  "x": "a",
  "a": "x",
  "c": {
    "d": "e"
  },
  "h": [
    "l",
    "p"
  ]
}"""

def test_json_to_yaml():
    result = run_cmd([cvto, "-i", "resources/a.json", "--out-format", "yaml"])
    assert result == """x: a
a: x
c:
  d: e
h:
- l
- p
"""