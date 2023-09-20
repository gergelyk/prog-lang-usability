require "json"

class Person
  include JSON::Serializable
  property name : String
  property height : Float64

  def initialize(@name, @height)
  end
end

person_a = Person.new("Greg", 1.80)
json_text_a = person_a.to_json
puts json_text_a

json_text_b = %({"name":"Tom", "height": 1.85})
person_b = Person.from_json(json_text_b)
p! person_b
