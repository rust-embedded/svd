use svd_rs::RegisterCluster;

use super::{
    new_node, Config, Element, ElementMerge, Encode, EncodeChildren, EncodeError, XMLNode,
};

use crate::{
    config::{change_case, format_number, DerivableSorting, RcSorting, Sorting},
    svd::{Peripheral, PeripheralInfo},
};

impl Encode for Peripheral {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        match self {
            Self::Single(info) => info.encode_with_config(config),
            Self::Array(info, array_info) => {
                let mut base = Element::new("peripheral");
                base.merge(&array_info.encode_with_config(config)?);
                base.merge(&info.encode_with_config(config)?);
                Ok(base)
            }
        }
    }
}

impl Encode for PeripheralInfo {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        let mut elem = Element::new("peripheral");
        elem.children.push(new_node(
            "name",
            change_case(&self.name, config.peripheral_name),
        ));

        if let Some(v) = &self.display_name {
            elem.children.push(new_node("displayName", v.to_string()));
        }

        if let Some(v) = &self.version {
            elem.children.push(new_node("version", v.to_string()));
        }

        if let Some(v) = &self.description {
            elem.children.push(new_node("description", v.to_string()));
        }

        if let Some(v) = &self.alternate_peripheral {
            elem.children.push(new_node(
                "alternatePeripheral",
                change_case(v, config.peripheral_name),
            ));
        }

        if let Some(v) = &self.group_name {
            elem.children.push(new_node("groupName", v.to_string()));
        }

        if let Some(v) = &self.prepend_to_name {
            elem.children.push(new_node(
                "prependToName",
                change_case(v, config.peripheral_name),
            ));
        }

        if let Some(v) = &self.append_to_name {
            elem.children.push(new_node(
                "appendToName",
                change_case(v, config.peripheral_name),
            ));
        }

        if let Some(v) = &self.header_struct_name {
            elem.children.push(new_node(
                "headerStructName",
                change_case(v, config.peripheral_name),
            ));
        }

        elem.children.push(new_node(
            "baseAddress",
            format_number(self.base_address, config.peripheral_base_address),
        ));

        elem.children.extend(
            self.default_register_properties
                .encode_with_config(config)?,
        );

        if let Some(v) = &self.address_block {
            for ab in v {
                elem.children.push(ab.encode_node_with_config(config)?);
            }
        }

        let interrupts: Result<Vec<_>, _> = self
            .interrupt
            .iter()
            .map(|interrupt| interrupt.encode_node_with_config(config))
            .collect();

        elem.children.append(&mut interrupts?);

        if let Some(v) = &self.registers {
            let children: Result<Vec<_>, _> = match config.register_cluster_sorting {
                RcSorting::Unchanged(DerivableSorting::Unchanged(None)) => v
                    .iter()
                    .map(|e| e.encode_node_with_config(config))
                    .collect(),
                RcSorting::Unchanged(sorting) => sort_derived_register_cluster(v, sorting)
                    .into_iter()
                    .map(|e| e.encode_node_with_config(config))
                    .collect(),
                RcSorting::RegistersFirst(sorting) => rc_sort(v, sorting, true)
                    .map(|e| e.encode_node_with_config(config))
                    .collect(),
                RcSorting::ClustersFirst(sorting) => rc_sort(v, sorting, false)
                    .map(|e| e.encode_node_with_config(config))
                    .collect(),
            };

            elem.children.push({
                let mut e = Element::new("registers");
                e.children = children?;
                XMLNode::Element(e)
            });
        }

        if let Some(v) = &self.derived_from {
            elem.attributes.insert(
                String::from("derivedFrom"),
                change_case(v, config.peripheral_name),
            );
        }

        Ok(elem)
    }
}

fn sort_register_cluster(refs: &mut [&RegisterCluster], sorting: Option<Sorting>) {
    if let Some(sorting) = sorting {
        match sorting {
            Sorting::Offset => refs.sort_by_key(|r| r.address_offset()),
            Sorting::OffsetReversed => {
                refs.sort_by_key(|r| -(r.address_offset() as i32));
            }
            Sorting::Name => refs.sort_by_key(|r| r.name()),
        }
    }
}

fn sort_derived_register_cluster<'a>(
    rcs: impl IntoIterator<Item = &'a RegisterCluster>,
    sorting: DerivableSorting,
) -> Vec<&'a RegisterCluster> {
    match sorting {
        DerivableSorting::Unchanged(sorting) => {
            let mut refs = rcs.into_iter().collect::<Vec<_>>();
            sort_register_cluster(&mut refs, sorting);
            refs
        }
        DerivableSorting::DeriveLast(sorting) => {
            let mut common_refs = Vec::new();
            let mut derived_refs = Vec::new();
            for rc in rcs {
                if rc.derived_from().is_some() {
                    derived_refs.push(rc);
                } else {
                    common_refs.push(rc);
                }
            }
            sort_register_cluster(&mut common_refs, sorting);
            sort_register_cluster(&mut derived_refs, sorting);
            common_refs.extend(derived_refs);
            common_refs
        }
    }
}

fn rc_sort(
    v: &[RegisterCluster],
    sorting: DerivableSorting,
    register_first: bool,
) -> impl Iterator<Item = &RegisterCluster> {
    let reg_refs = v
        .iter()
        .filter(|rc| matches!(rc, RegisterCluster::Register(_)))
        .collect::<Vec<_>>();
    let reg_refs = sort_derived_register_cluster(reg_refs, sorting);

    let c_refs = v
        .iter()
        .filter(|rc| matches!(rc, RegisterCluster::Cluster(_)))
        .collect::<Vec<_>>();
    let c_refs = sort_derived_register_cluster(c_refs, sorting);
    if register_first {
        reg_refs.into_iter().chain(c_refs)
    } else {
        c_refs.into_iter().chain(reg_refs)
    }
}
