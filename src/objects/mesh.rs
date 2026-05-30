use super::core::{Property, PropertyValue, RiveObject, property_keys, type_keys};

pub struct MeshVertex {
    pub name: String,
    pub parent_id: u64,
    pub x: f32,
    pub y: f32,
    pub u: f32,
    pub v: f32,
}

impl MeshVertex {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            x: 0.0,
            y: 0.0,
            u: 0.0,
            v: 0.0,
        }
    }
}

impl RiveObject for MeshVertex {
    fn type_key(&self) -> u16 {
        type_keys::MESH_VERTEX
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.x != 0.0 {
            props.push(Property {
                key: property_keys::VERTEX_X,
                value: PropertyValue::Float(self.x),
            });
        }
        if self.y != 0.0 {
            props.push(Property {
                key: property_keys::VERTEX_Y,
                value: PropertyValue::Float(self.y),
            });
        }
        if self.u != 0.0 {
            props.push(Property {
                key: property_keys::MESH_VERTEX_U,
                value: PropertyValue::Float(self.u),
            });
        }
        if self.v != 0.0 {
            props.push(Property {
                key: property_keys::MESH_VERTEX_V,
                value: PropertyValue::Float(self.v),
            });
        }
        props
    }
}

pub struct Mesh {
    pub name: String,
    pub parent_id: u64,
}

impl Mesh {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self { name, parent_id }
    }
}

impl RiveObject for Mesh {
    fn type_key(&self) -> u16 {
        type_keys::MESH
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ]
    }
}

pub struct ContourMeshVertex {
    pub name: String,
    pub parent_id: u64,
    pub x: f32,
    pub y: f32,
    pub u: f32,
    pub v: f32,
}

impl ContourMeshVertex {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            x: 0.0,
            y: 0.0,
            u: 0.0,
            v: 0.0,
        }
    }
}

impl RiveObject for ContourMeshVertex {
    fn type_key(&self) -> u16 {
        type_keys::CONTOUR_MESH_VERTEX
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.x != 0.0 {
            props.push(Property {
                key: property_keys::VERTEX_X,
                value: PropertyValue::Float(self.x),
            });
        }
        if self.y != 0.0 {
            props.push(Property {
                key: property_keys::VERTEX_Y,
                value: PropertyValue::Float(self.y),
            });
        }
        if self.u != 0.0 {
            props.push(Property {
                key: property_keys::MESH_VERTEX_U,
                value: PropertyValue::Float(self.u),
            });
        }
        if self.v != 0.0 {
            props.push(Property {
                key: property_keys::MESH_VERTEX_V,
                value: PropertyValue::Float(self.v),
            });
        }
        props
    }
}

pub struct ForcedEdge {
    pub name: String,
    pub parent_id: u64,
    pub from_id: u64,
    pub to_id: u64,
}

impl ForcedEdge {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            from_id: 0,
            to_id: 0,
        }
    }
}

impl RiveObject for ForcedEdge {
    fn type_key(&self) -> u16 {
        type_keys::FORCED_EDGE
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.from_id != 0 {
            props.push(Property {
                key: property_keys::FORCED_EDGE_FROM_ID,
                value: PropertyValue::UInt(self.from_id),
            });
        }
        if self.to_id != 0 {
            props.push(Property {
                key: property_keys::FORCED_EDGE_TO_ID,
                value: PropertyValue::UInt(self.to_id),
            });
        }
        props
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_vertex_type_key() {
        let v = MeshVertex::new("v".to_string(), 1);
        assert_eq!(v.type_key(), 108);
    }

    #[test]
    fn test_mesh_vertex_default_omission() {
        let v = MeshVertex::new("v".to_string(), 1);
        let props = v.properties();
        assert_eq!(props.len(), 2);
    }

    #[test]
    fn test_mesh_vertex_with_values() {
        let mut v = MeshVertex::new("v".to_string(), 1);
        v.x = 10.0;
        v.y = 20.0;
        v.u = 0.5;
        v.v = 0.5;
        let props = v.properties();
        assert_eq!(props.len(), 6);
    }

    #[test]
    fn test_mesh_type_key() {
        let m = Mesh::new("m".to_string(), 1);
        assert_eq!(m.type_key(), 109);
        assert_eq!(m.properties().len(), 2);
    }

    #[test]
    fn test_contour_mesh_vertex_type_key() {
        let v = ContourMeshVertex::new("cv".to_string(), 1);
        assert_eq!(v.type_key(), 111);
    }

    #[test]
    fn test_contour_mesh_vertex_with_values() {
        let mut v = ContourMeshVertex::new("cv".to_string(), 1);
        v.x = 100.0;
        v.u = 1.0;
        let props = v.properties();
        assert_eq!(props.len(), 4);
    }

    #[test]
    fn test_forced_edge_type_key() {
        let e = ForcedEdge::new("e".to_string(), 1);
        assert_eq!(e.type_key(), 112);
    }

    #[test]
    fn test_forced_edge_default_omission() {
        let e = ForcedEdge::new("e".to_string(), 1);
        let props = e.properties();
        assert_eq!(props.len(), 2);
    }

    #[test]
    fn test_forced_edge_with_values() {
        let mut e = ForcedEdge::new("e".to_string(), 1);
        e.from_id = 3;
        e.to_id = 5;
        let props = e.properties();
        assert_eq!(props.len(), 4);
        assert_eq!(props[2].key, property_keys::FORCED_EDGE_FROM_ID);
        assert_eq!(props[3].key, property_keys::FORCED_EDGE_TO_ID);
    }
}
