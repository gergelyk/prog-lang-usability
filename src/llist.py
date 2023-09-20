class Element:
    def __init__(self, data):
        self.data = data
        self.link = None

class LList:    
    def __init__(self):
        self.root = Element(None)

    def _get_last(self):
        last = self.root
        while last.link is not None:
            last = last.link
        return last

    def prepend(self, data):
        elem = Element(data)
        elem.link = self.root.link
        self.root.link = elem

    def append(self, data):
        last = self._get_last()
        last.link = Element(data)
        
    def show(self):
        elem = self.root
        while True:
            elem = elem.link
            if elem is None:
                break
            print(elem.data)
            
llist = LList()
llist.append(123)
llist.prepend(456)
llist.append(789)
llist.show()
