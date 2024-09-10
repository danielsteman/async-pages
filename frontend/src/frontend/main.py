import httpx

BASE_URL = "http://localhost:8080"
ITEMS_ENDPOINT = "list_things"

params = {"page": 1, "per_page": 50}
response = httpx.get(f"{BASE_URL}/{ITEMS_ENDPOINT}", params=params)
print(response)
