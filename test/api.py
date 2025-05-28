import requests
import json

r = requests.get(url='http://127.0.0.1:8888/api/v1/recipe/1643132210')
print(f"STATUS {r.status_code}")
assert r.status_code == 200
j = json.dumps(r.json(), indent=2)
print(j)

r = requests.get(url='http://127.0.0.1:8888/api/v1/recipe/with-tags', json=["soy free", "winter", "kosher"])
assert r.status_code == 200
print(f"STATUS {r.status_code}")
j2 = json.dumps(r.json(), indent=2)
print(j2)

r = requests.get(url='http://127.0.0.1:8888/api/v1/recipe/random')
assert r.status_code == 200
print(f"STATUS {r.status_code}")
j3 = json.dumps(r.json(), indent=2)
print(j3)
