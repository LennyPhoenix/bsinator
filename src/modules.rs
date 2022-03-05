use crate::config;
use std::path::PathBuf;
use yaml_rust::{Yaml, YamlLoader};

fn yaml_to_string(yaml: &Yaml) -> String {
    match yaml {
        Yaml::Real(_) => "real",
        Yaml::Integer(_) => "integer",
        Yaml::Boolean(_) => "boolean",
        Yaml::String(_) => "string",
        Yaml::Array(_) => "array",
        Yaml::Hash(_) => "hash",
        Yaml::Alias(_) => "alias",
        Yaml::BadValue => "badvalue",
        Yaml::Null => "null",
    }
    .to_string()
}

pub enum DepGroupReadError {
    MissingPackageList,
    TypeMismatch {
        key: String,
        expected: String,
        actual: String,
    },
}

pub struct DependencyGroup {
    pub packages: Vec<String>,
    pub prompt: Option<String>,
    pub requires: i32,
    pub asdeps: bool,
}

impl DependencyGroup {
    pub fn from_yaml(yaml: &Yaml) -> Result<Self, DepGroupReadError> {
        if yaml["packages"].is_badvalue() {
            return Err(DepGroupReadError::MissingPackageList);
        }

        let packages = match &yaml["packages"] {
            Yaml::Array(packages) => {
                let mut pkgs = Vec::new();
                for (index, package) in packages.iter().enumerate() {
                    match package {
                        Yaml::String(package) => pkgs.push(package.to_string()),
                        _ => {
                            return Err(DepGroupReadError::TypeMismatch {
                                key: format!("packages[{}]", index),
                                expected: "string".to_string(),
                                actual: yaml_to_string(package),
                            })
                        }
                    }
                }
                pkgs
            }
            other => {
                let expected = "array".to_string();
                let actual = yaml_to_string(other);
                return Err(DepGroupReadError::TypeMismatch {
                    key: "packages".to_string(),
                    expected,
                    actual,
                });
            }
        };

        let prompt = match &yaml["prompt"] {
            Yaml::String(prompt) => Some(prompt.to_string()),
            Yaml::BadValue => None,
            other => {
                let expected = "string".to_string();
                let actual = yaml_to_string(other);
                return Err(DepGroupReadError::TypeMismatch {
                    key: "prompt".to_string(),
                    expected,
                    actual,
                });
            }
        };

        let requires = match &yaml["requires"] {
            Yaml::Integer(requires) => *requires as i32,
            Yaml::BadValue => -1,
            other => {
                let expected = "integer".to_string();
                let actual = yaml_to_string(other);
                return Err(DepGroupReadError::TypeMismatch {
                    key: "requires".to_string(),
                    expected,
                    actual,
                });
            }
        };

        let asdeps = match &yaml["asdeps"] {
            Yaml::Boolean(asdeps) => *asdeps,
            Yaml::BadValue => false,
            other => {
                let expected = "boolean".to_string();
                let actual = yaml_to_string(other);
                return Err(DepGroupReadError::TypeMismatch {
                    key: "asdeps".to_string(),
                    expected,
                    actual,
                });
            }
        };

        Ok(Self {
            packages,
            prompt,
            requires,
            asdeps,
        })
    }
}

pub enum ModuleReadError {
    TypeMismatch {
        key: String,
        expected: String,
        actual: String,
    },
    MissingName,
    DepReadFailed(usize, DepGroupReadError),
}

pub struct Module {
    pub name: String,
    pub description: Option<String>,
    pub dependencies: Option<Vec<DependencyGroup>>,
    pub pre_hook: Option<String>,
    pub post_hook: Option<String>,
}

impl Module {
    pub fn from_yaml(yaml: &Yaml) -> Result<Self, ModuleReadError> {
        if yaml["name"].is_badvalue() {
            return Err(ModuleReadError::MissingName);
        }

        let name = match &yaml["name"] {
            Yaml::String(s) => s.to_string(),
            other => {
                let expected = "string".to_string();
                let actual = yaml_to_string(other);
                return Err(ModuleReadError::TypeMismatch {
                    key: "name".to_string(),
                    expected,
                    actual,
                });
            }
        };

        let description = match &yaml["description"] {
            Yaml::String(s) => Some(s.to_string()),
            Yaml::BadValue => None,
            other => {
                let expected = "string".to_string();
                let actual = yaml_to_string(other);
                return Err(ModuleReadError::TypeMismatch {
                    key: "description".to_string(),
                    expected,
                    actual,
                });
            }
        };

        let dependencies = match &yaml["dependencies"] {
            Yaml::Array(a) => {
                let mut deps = Vec::new();
                for (index, dep) in a.iter().enumerate() {
                    let dep_group = match DependencyGroup::from_yaml(dep) {
                        Ok(dep_group) => dep_group,
                        Err(e) => return Err(ModuleReadError::DepReadFailed(index, e)),
                    };
                    deps.push(dep_group);
                }
                Some(deps)
            }
            Yaml::BadValue => None,
            other => {
                let expected = "array".to_string();
                let actual = yaml_to_string(other);
                return Err(ModuleReadError::TypeMismatch {
                    key: "dependencies".to_string(),
                    expected,
                    actual,
                });
            }
        };

        let pre_hook = match &yaml["pre_hook"] {
            Yaml::String(s) => Some(s.to_string()),
            Yaml::BadValue => None,
            other => {
                let expected = "string".to_string();
                let actual = yaml_to_string(other);
                return Err(ModuleReadError::TypeMismatch {
                    key: "pre-hook".to_string(),
                    expected,
                    actual,
                });
            }
        };

        let post_hook = match &yaml["post_hook"] {
            Yaml::String(s) => Some(s.to_string()),
            Yaml::BadValue => None,
            other => {
                let expected = "string".to_string();
                let actual = yaml_to_string(other);
                return Err(ModuleReadError::TypeMismatch {
                    key: "post-hook".to_string(),
                    expected,
                    actual,
                });
            }
        };

        Ok(Self {
            name,
            description,
            dependencies,
            pre_hook,
            post_hook,
        })
    }
}

pub fn parse_modules(config_path: PathBuf) -> std::io::Result<Vec<Module>> {
    let module_paths = config::traverse_config_directory(config_path)?;
    let mut modules = Vec::new();

    for module_path in module_paths {
        let module_yaml = config::read_config(&module_path)?;
        let docs = match YamlLoader::load_from_str(&module_yaml) {
            Ok(docs) => docs,
            Err(err) => {
                let marker = err.marker();
                println!(
                    "Error parsing module {} at line {} char {}",
                    module_path.display(),
                    marker.line(),
                    marker.index()
                );
                continue;
            }
        };
        for (index, doc) in docs.iter().enumerate() {
            let module = match Module::from_yaml(doc) {
                Ok(module) => module,
                Err(err) => {
                    println!(
                        "Error parsing module {} index {}: {}",
                        module_path.display(),
                        index,
                        match err {
                            // TODO: Cleanup TypeMismatch duplication
                            ModuleReadError::TypeMismatch {
                                key,
                                expected,
                                actual,
                            } => format!(
                                "type mismatch for key '{}', expected '{}' but got '{}'",
                                key, expected, actual
                            ),
                            ModuleReadError::MissingName => "missing name".to_string(),
                            ModuleReadError::DepReadFailed(index, err) => format!(
                                "dependency group at index '{}' failed to parse: {}",
                                index,
                                match err {
                                    DepGroupReadError::MissingPackageList => {
                                        "missing package list".to_string()
                                    }
                                    DepGroupReadError::TypeMismatch {
                                        key,
                                        expected,
                                        actual,
                                    } => format!(
                                        "type mismatch for key '{}', expected '{}' but got '{}'",
                                        key, expected, actual
                                    ),
                                },
                            ),
                        }
                    );
                    continue;
                }
            };
            modules.push(module);
        }
    }

    Ok(modules)
}
