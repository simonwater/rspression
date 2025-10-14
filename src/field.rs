use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Field {
    name: String,
    owner: Option<Rc<Field>>,
    src: Option<String>,
}

impl Field {
    pub fn new(name: &str) -> Self {
        Field {
            name: name.to_string(),
            owner: None,
            src: None,
        }
    }

    pub fn with_owner(name: &str, owner: Rc<Field>) -> Self {
        Field {
            name: name.to_string(),
            owner: Some(owner),
            src: None,
        }
    }

    pub fn with_str(src: &str) -> Rc<Field> {
        let mut cur: Option<Rc<Field>> = None;
        for name in src.split('.') {
            let field = match cur {
                Some(ref owner) => Field::with_owner(name, owner.clone()),
                None => Field::new(name),
            };
            cur = Some(Rc::new(field));
        }
        cur.unwrap()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_owner(&self) -> Option<Rc<Field>> {
        self.owner.clone()
    }

    fn search(field: Option<Rc<Field>>, path: &mut Vec<String>) {
        if let Some(f) = field {
            Field::search(f.owner.clone(), path);
            path.push(f.name.clone());
        }
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self.src {
            Some(s) => s,
            None => {
                let mut path = Vec::new();
                Field::search(Some(Rc::new(self.clone())), &mut path);
                &path.join(".")
            }
        };
        write!(f, "{}", s)
    }
}

#[test]
fn test() {
    let src = "a.b.c.d";
    let field = Field::with_str(src);
    assert_eq!(src, field.to_string());

    let src = "table1";
    let field = Field::with_str(src);
    assert_eq!(src, field.to_string());

    let field = Field::with_owner("field1", field.clone());
    assert_eq!("table1.field1", field.to_string());

    let field = Field::with_str("f1");
    assert_eq!("f1", field.to_string());
}
