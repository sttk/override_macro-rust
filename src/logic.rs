// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use crate::syn_dax;

use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

static TRAIT_VEC: RwLock<Vec<OverridableTrait>> = RwLock::new(Vec::new());
static TRAIT_MAP: LazyLock<RwLock<HashMap<String, FoundTrait>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

enum FoundTrait {
    Found(usize),
    Conflict,
}

struct OverridableTrait {
    #[allow(dead_code)]
    mod_path: Option<String>,

    #[allow(dead_code)]
    name: String,

    method_map: HashMap<String, OverridableMethod>,
}

struct OverridableMethod {
    name: String,
    sig: String,
    call: String,
    has_impl: bool,
}

pub fn collect_trait_info(dax: &syn_dax::OverridableDax) {
    let trait_name = dax.get_trait_name();
    let mod_path = dax.get_module_path();
    let trait_index = register_trait(dax, &trait_name, &mod_path);

    let mut trait_map = TRAIT_MAP.write().unwrap();

    for key in dax.list_trait_search_keys() {
        match trait_map.get_mut(&key) {
            Some(_) => trait_map.insert(key, FoundTrait::Conflict),
            None => trait_map.insert(key, FoundTrait::Found(trait_index)),
        };
    }
}

fn register_trait(
    dax: &syn_dax::OverridableDax,
    trait_name: &str,
    mod_path: &Option<String>,
) -> usize {
    let mut trait_vec = TRAIT_VEC.write().unwrap();
    let trait_index = trait_vec.len();

    let mut method_map = HashMap::<String, OverridableMethod>::new();

    dax.for_each_method_registration(|name, sig, call, has_impl, key| {
        let m = OverridableMethod {
            name,
            sig,
            call,
            has_impl,
        };
        method_map.insert(key, m);
    });

    let t = OverridableTrait {
        mod_path: mod_path.clone(),
        name: trait_name.to_string(),
        method_map,
    };
    trait_vec.push(t);

    trait_index
}

struct ArgumentTrait {
    path: String,
    index: usize,
}

pub fn override_trait_methods(dax: &mut syn_dax::OverrideWithDax) {
    let trait_map = TRAIT_MAP.read().unwrap();

    let impl_trait_index = dax
        .get_impl_trait(|keys| Some(find_trait(&trait_map, keys)))
        .unwrap();

    let trait_vec = TRAIT_VEC.read().unwrap();
    let impl_trait = &trait_vec[impl_trait_index];

    let arg_traits = dax.list_argument_traits(|keys, path| {
        let index = find_trait(&trait_map, keys);
        if index == impl_trait_index {
            panic!(
                "The same trait as the impl trait was found in the arguments: {}",
                path
            );
        }
        Some(ArgumentTrait { path, index })
    });

    let impl_method_keys = dax.list_impl_method_keys();

    let mut overriding_methods = Vec::<String>::new();

    for (key, method) in &impl_trait.method_map {
        if method.has_impl {
            continue;
        }

        if impl_method_keys.contains(&key) {
            continue;
        }

        let mut conflicting_traits = Vec::<String>::new();

        for arg_trait in &arg_traits {
            let t = &trait_vec[arg_trait.index];

            if let Some(m) = t.method_map.get(key) {
                if !m.has_impl {
                    continue;
                }

                let t_path = &arg_trait.path;

                conflicting_traits.push(t_path.clone());
                if conflicting_traits.len() > 1 {
                    continue;
                }

                overriding_methods.push(format!("{} {{ {}::{} }}", m.sig, t_path, m.call));
            }
        }

        if conflicting_traits.len() > 1 {
            panic!(
                "The method `{}` is implemented in multiple traits: {}",
                method.name,
                &conflicting_traits.join(", "),
            );
        }
    }

    dax.set_overriding_method_impls(overriding_methods);
}

fn find_trait(trait_map: &HashMap<String, FoundTrait>, search_keys: &[String]) -> usize {
    for key in search_keys {
        match trait_map.get(key) {
            Some(FoundTrait::Found(i)) => return *i,
            Some(FoundTrait::Conflict) => {
                panic!("There are multiple traits matching with: {}", key);
            }
            None => continue,
        }
    }

    panic!(
        "There is no traits with the same path or sub path: {}",
        search_keys[0]
    );
}
