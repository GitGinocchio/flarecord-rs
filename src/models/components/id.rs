use std::any::TypeId;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub (crate) struct HierarchicalId {
    prefix: Option<String>,
    path: Vec<usize>
}

impl HierarchicalId {
    pub fn new() -> Self {
        Self {
            prefix: None,
            path: vec![0]
        }
    }

    pub fn set_prefix(&mut self, prefix: String) {
        self.prefix = Some(prefix);
    }

    #[allow(unused)]
    pub fn root(&mut self) -> &mut Self {
        self.path = vec![0];
        self
    }

    pub fn next_root(&mut self) -> &mut Self {
        self.path.truncate(1);
        if let Some(first) = self.path.first_mut() {
            *first += 1;
        } else {
            self.path = vec![1];
        }
        self
    }

    #[allow(unused)]
    pub fn parent(&mut self) -> &mut Self {
        if self.path.len() > 1 {
            self.path.pop();
        }
        self
    }

    pub fn child(&mut self) -> &mut Self {
        if let Some(last) = self.path.last() {
            self.path.push(*last+1)
        }
        self
    }

    pub fn sibling(&mut self) -> &mut Self {
        if let Some(last) = self.path.last_mut() {
            *last += 1;
        }
        self
    }

    pub fn as_string(&self) -> String {
        let path = self.path.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(":");

        if let Some(prefix) = &self.prefix {
            format!("{prefix}:{path}")
        } else {
            path
        }
    }

    pub fn as_usize(&self) -> usize {
        *self.path.last().unwrap_or(&0)
    }
}

pub (crate) trait IdAssignable {
    fn set_id(&mut self, id: &HierarchicalId);
    
    fn children(&mut self) -> Box<dyn Iterator<Item = &mut dyn IdAssignable> + '_> {
        Box::new(std::iter::empty())
    }
}

#[allow(unused)]
pub (crate) fn get_component_id<T: 'static>() -> String {
    let mut s = DefaultHasher::new();
    TypeId::of::<T>().hash(&mut s);
    let hash = s.finish();
    format!("{:016x}", hash)[..4].to_string()
}

pub (crate) fn get_component_id_from_type_id(type_id: TypeId) -> String {
    let mut s = DefaultHasher::new();
    type_id.hash(&mut s);
    let hash = s.finish();
    format!("{:016x}", hash)[..4].to_string()
}

pub (crate) fn assign_ids(component: &mut dyn IdAssignable, current_id: &mut HierarchicalId) {
    component.set_id(&current_id);

    for (i, child) in component.children().enumerate() {
        let mut child_id = current_id.clone();
        
        child_id.child();
        
        for _ in 0..i {
            child_id.sibling();
        }
        
        assign_ids(child, &mut child_id);
    }
}