class Object
  def vars
    {{ @type.instance_vars.map {|v| v.name.stringify + " : " + v.type.stringify} }}
  end
  
  def show_vars
    self.vars.each {|v| puts v}
  end
end

class Foo
  def initialize(@bar : Int32, @baz : String)
  end
end

foo = Foo.new 123, "abc"
foo.show_vars()
