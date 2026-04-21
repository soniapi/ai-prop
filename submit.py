import urllib.request
import json
req = urllib.request.Request('http://127.0.0.1:8000/submit', data=json.dumps({"branch": "jules-2893936258022630140-adf3170c"}).encode('utf-8'), headers={'Content-Type': 'application/json'})
urllib.request.urlopen(req)
