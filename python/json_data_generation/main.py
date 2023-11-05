import json
import random, string

# https://qiita.com/Scstechr/items/c3b2eb291f7c5b81902a
def randomname(n):
   return ''.join(random.choices(string.ascii_letters + string.digits, k=n))

number_of_fields = 10
length_of_data = 10
data = {}
dir = './data.json'

for i in range(1, number_of_fields+1):
    data['field'+str(i)] = randomname(length_of_data)

# https://qiita.com/Mijinko/items/d37d069e5a7485a72b11
with open(dir, mode="wt", encoding="utf-8") as f:
	json.dump(data, f, ensure_ascii=False, indent=2)
