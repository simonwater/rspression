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

    pub fn add_assign(&mut self, name: &str) {
        self.assigns.insert(name.to_string());
    }

    pub fn add_depend(&mut self, name: &str) {
        self.depends.insert(name.to_string());
    }

    pub fn comebine(&mut self, other: Option<VariableSet>) {
        if let Some(other) = other {
            self.assigns.extend(other.get_assigns().iter().cloned());
            self.depends.extend(other.get_depends().iter().cloned());
        }
    }

    pub fn from_depends(names: &[&str]) -> VariableSet {
        let mut result = VariableSet::default();
        for &name in names {
            result.add_depend(name);
        }
        result
    }

    pub fn from_assigns(names: &[&str]) -> VariableSet {
        let mut result = VariableSet::default();
        for &name in names {
            result.add_assign(name);
        }
        result
    }
}

impl fmt::Display for VariableSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut assign_arr: Vec<&String> = self.assigns.iter().collect();
        assign_arr.sort();
        let mut result = assign_arr
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join(",");
        if !result.is_empty() {
            result.push_str(" = ");
        }
        let mut depend_arr: Vec<&String> = self.depends.iter().collect();
        depend_arr.sort();
        let depend_strs: Vec<&str> = depend_arr.iter().map(|s| s.as_str()).collect();
        result.push_str(&depend_strs.join(","));
        write!(f, "{}", result)
    }
}
