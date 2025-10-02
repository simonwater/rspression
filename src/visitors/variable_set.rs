use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct VariableSet {
    assigns: HashSet<String>,
    depends: HashSet<String>,
}

impl VariableSet {
    pub fn get_assigns(&self) -> &HashSet<String> {
        &self.assigns
    }

    pub fn set_assigns(&mut self, assigns: HashSet<String>) {
        self.assigns = assigns;
    }

    pub fn get_depends(&self) -> &HashSet<String> {
        &self.depends
    }

    pub fn set_depends(&mut self, depends: HashSet<String>) {
        self.depends = depends;
    }

    pub fn add_assign(&mut self, name: String) {
        self.assigns.insert(name);
    }

    pub fn add_depend(&mut self, name: String) {
        self.depends.insert(name);
    }

    pub fn comebine(&mut self, other_opt: Option<VariableSet>) {
        if let Some(VariableSet { assigns, depends }) = other_opt {
            self.assigns.extend(assigns);
            self.depends.extend(depends);
        }
    }

    pub fn from_depends(names: &[&String]) -> VariableSet {
        let mut result = VariableSet::default();
        for &name in names {
            result.add_depend(name.to_string());
        }
        result
    }

    pub fn from_assigns(names: &[&String]) -> VariableSet {
        let mut result = VariableSet::default();
        for &name in names {
            result.add_assign(name.to_string());
        }
        result
    }
}

impl fmt::Display for VariableSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut assign_arr: Vec<&str> = self.assigns.iter().map(|s| s.as_str()).collect();
        assign_arr.sort();
        let mut result = assign_arr.join(",");
        if !result.is_empty() {
            result.push_str(" = ");
        }
        let mut depend_arr: Vec<&str> = self.depends.iter().map(|s| s.as_str()).collect();
        depend_arr.sort();
        let depend_str = depend_arr.join(",");
        result.push_str(depend_str.as_str());
        write!(f, "{}", result)
    }
}
