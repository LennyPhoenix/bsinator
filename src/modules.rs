pub struct DependencyGroup {
    pub packages: Vec<String>,
    pub prompt: Option<String>,
    pub requires: i32,
    pub asdeps: bool,
}

impl DependencyGroup {
    pub fn new_empty() -> Self {
        Self {
            packages: Vec::new(),
            prompt: None,
            requires: -1,
            asdeps: false,
        }
    }
}

pub struct Module {
    pub name: String,
    pub description: Option<String>,
    pub dependencies: Option<Vec<DependencyGroup>>,
    pub pre_hook: Option<String>,
    pub post_hook: Option<String>,
}

impl Module {
    pub fn new_empty() -> Self {
        Self {
            name: String::new(),
            description: None,
            dependencies: None,
            pre_hook: None,
            post_hook: None,
        }
    }
}

