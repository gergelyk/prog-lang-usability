import json
from dataclasses import dataclass, asdict

@dataclass
class Person:
  name: str
  height: float
 
person_a = Person("Greg", 1.80)
json_text_a = json.dumps(asdict(person_a))
print(json_text_a)

json_text_b = '{"name":"Tom", "height": 1.85}'
person_b = Person(**json.loads(json_text_b))
print(person_b)
