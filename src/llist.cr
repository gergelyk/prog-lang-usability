class Element
  property link : Element?
  getter data

  def initialize(@data : Int32)
  end
end

class LList
  def initialize
    @root = Element.new 0
  end

  def prepend(data)
    elem = Element.new data
    elem.link = @root.link
    @root.link = elem
  end

  private def get_last
    last = @root
    while !last.link.nil?
      last = last.link.not_nil! # how to avoid not_nil! ?
    end
    last
  end

  def append(data)
    last = get_last
    last.link = Element.new data
  end

  def show
    elem = @root
    loop do
      elem = elem.link
      break if elem.nil?
      puts elem.data
    end
  end
end

llist = LList.new
llist.append 123
llist.prepend 456
llist.append 789
llist.show
