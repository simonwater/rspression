use std::rc::Rc;

#[derive(Clone)]
pub struct Field {
    name: String,
    owner: Option<Rc<Field>>,
    src: Option<String>,
}

impl Field {
    pub fn from_name(name: String) -> Self {
        Field {
            name: name,
            owner: None,
            src: None,
        }
    }

    pub fn from_src(src: String) -> Rc<Field> {
        let mut cur: Option<Rc<Field>> = None;
        for name in src.split('.') {
            let field = match cur {
                Some(ref owner) => Field::with_owner(name.to_string(), owner.clone()),
                None => Field::from_name(name.to_string()),
            };
            cur = Some(Rc::new(field));
        }
        cur.unwrap()
    }

    pub fn with_owner(name: String, owner: Rc<Field>) -> Self {
        Field {
            name: name,
            owner: Some(owner),
            src: None,
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let src = "a.b.c.d";
        let field = Field::from_src(src.into());
        assert_eq!(src, field.to_string());

        let src = "table1";
        let field = Field::from_src(src.into());
        assert_eq!(src, field.to_string());

        let field = Field::with_owner("field1".into(), field.clone());
        assert_eq!("table1.field1", field.to_string());

        let field = Field::from_src("f1".into());
        assert_eq!("f1", field.to_string());
    }
}
