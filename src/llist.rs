struct Element<'a> {
    data: i32,
    link: Option<&'a Box<Element<'a>>>,
}

impl Element<'_> {
    fn new(data: i32) -> Self {
        Self{data: data, link: None}
    }
}

struct LList<'a> {
    root: &'a mut Box<Element<'a>>,
}

impl LList<'_> {
    fn new() -> Self {
        Self{root: & Box::new(Element::new(0))}
    }
    
    fn prepend(&mut self, data: i32) {
        let mut elem = Element::new(data);
        elem.link = self.root.link;
        self.root.link = Some(&Box::new(elem));
    }
    /*
    fn append(data: i32) {
    }*/
    /*
    fn show(&self) {
        let mut elem = &self.root;
        loop {
            let maybe_elem = &elem.link;
             match maybe_elem {
                 None => { break; }
                 Some(some_elem) => {
                     elem = &some_elem;
                     println!("{}", elem.data)
                 }
             }
        }
    }*/
}

fn main() {
    let mut llist = LList::new();
    //llist.append(123);
    llist.prepend(456);
    llist.prepend(99999999);
    //llist.append(789);
    //llist.show();
}
