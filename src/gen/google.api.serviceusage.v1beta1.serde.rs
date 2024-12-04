// @generated
impl serde::Serialize for AdminQuotaPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.policy_value != 0 {
            len += 1;
        }
        if !self.dimensions.is_empty() {
            len += 1;
        }
        if !self.metric.is_empty() {
            len += 1;
        }
        if !self.unit.is_empty() {
            len += 1;
        }
        if !self.container.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.AdminQuotaPolicy", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.policy_value != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("policyValue", ToString::to_string(&self.policy_value).as_str())?;
        }
        if !self.dimensions.is_empty() {
            struct_ser.serialize_field("dimensions", &self.dimensions)?;
        }
        if !self.metric.is_empty() {
            struct_ser.serialize_field("metric", &self.metric)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
        }
        if !self.container.is_empty() {
            struct_ser.serialize_field("container", &self.container)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AdminQuotaPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "policy_value",
            "policyValue",
            "dimensions",
            "metric",
            "unit",
            "container",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            PolicyValue,
            Dimensions,
            Metric,
            Unit,
            Container,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "policyValue" | "policy_value" => Ok(GeneratedField::PolicyValue),
                            "dimensions" => Ok(GeneratedField::Dimensions),
                            "metric" => Ok(GeneratedField::Metric),
                            "unit" => Ok(GeneratedField::Unit),
                            "container" => Ok(GeneratedField::Container),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AdminQuotaPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.AdminQuotaPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AdminQuotaPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut policy_value__ = None;
                let mut dimensions__ = None;
                let mut metric__ = None;
                let mut unit__ = None;
                let mut container__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PolicyValue => {
                            if policy_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyValue"));
                            }
                            policy_value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Dimensions => {
                            if dimensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dimensions"));
                            }
                            dimensions__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Metric => {
                            if metric__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metric"));
                            }
                            metric__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Container => {
                            if container__.is_some() {
                                return Err(serde::de::Error::duplicate_field("container"));
                            }
                            container__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AdminQuotaPolicy {
                    name: name__.unwrap_or_default(),
                    policy_value: policy_value__.unwrap_or_default(),
                    dimensions: dimensions__.unwrap_or_default(),
                    metric: metric__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                    container: container__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.AdminQuotaPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchCreateAdminOverridesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.overrides.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.BatchCreateAdminOverridesResponse", len)?;
        if !self.overrides.is_empty() {
            struct_ser.serialize_field("overrides", &self.overrides)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchCreateAdminOverridesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "overrides",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Overrides,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "overrides" => Ok(GeneratedField::Overrides),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchCreateAdminOverridesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.BatchCreateAdminOverridesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchCreateAdminOverridesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut overrides__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Overrides => {
                            if overrides__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrides"));
                            }
                            overrides__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchCreateAdminOverridesResponse {
                    overrides: overrides__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.BatchCreateAdminOverridesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchCreateConsumerOverridesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.overrides.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.BatchCreateConsumerOverridesResponse", len)?;
        if !self.overrides.is_empty() {
            struct_ser.serialize_field("overrides", &self.overrides)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchCreateConsumerOverridesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "overrides",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Overrides,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "overrides" => Ok(GeneratedField::Overrides),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchCreateConsumerOverridesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.BatchCreateConsumerOverridesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchCreateConsumerOverridesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut overrides__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Overrides => {
                            if overrides__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrides"));
                            }
                            overrides__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchCreateConsumerOverridesResponse {
                    overrides: overrides__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.BatchCreateConsumerOverridesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchEnableServicesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if !self.service_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.BatchEnableServicesRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if !self.service_ids.is_empty() {
            struct_ser.serialize_field("serviceIds", &self.service_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchEnableServicesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "service_ids",
            "serviceIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            ServiceIds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "parent" => Ok(GeneratedField::Parent),
                            "serviceIds" | "service_ids" => Ok(GeneratedField::ServiceIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchEnableServicesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.BatchEnableServicesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchEnableServicesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut service_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServiceIds => {
                            if service_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceIds"));
                            }
                            service_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchEnableServicesRequest {
                    parent: parent__.unwrap_or_default(),
                    service_ids: service_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.BatchEnableServicesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConsumerQuotaLimit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.metric.is_empty() {
            len += 1;
        }
        if !self.unit.is_empty() {
            len += 1;
        }
        if self.is_precise {
            len += 1;
        }
        if self.allows_admin_overrides {
            len += 1;
        }
        if !self.quota_buckets.is_empty() {
            len += 1;
        }
        if !self.supported_locations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ConsumerQuotaLimit", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.metric.is_empty() {
            struct_ser.serialize_field("metric", &self.metric)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
        }
        if self.is_precise {
            struct_ser.serialize_field("isPrecise", &self.is_precise)?;
        }
        if self.allows_admin_overrides {
            struct_ser.serialize_field("allowsAdminOverrides", &self.allows_admin_overrides)?;
        }
        if !self.quota_buckets.is_empty() {
            struct_ser.serialize_field("quotaBuckets", &self.quota_buckets)?;
        }
        if !self.supported_locations.is_empty() {
            struct_ser.serialize_field("supportedLocations", &self.supported_locations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConsumerQuotaLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "metric",
            "unit",
            "is_precise",
            "isPrecise",
            "allows_admin_overrides",
            "allowsAdminOverrides",
            "quota_buckets",
            "quotaBuckets",
            "supported_locations",
            "supportedLocations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Metric,
            Unit,
            IsPrecise,
            AllowsAdminOverrides,
            QuotaBuckets,
            SupportedLocations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "metric" => Ok(GeneratedField::Metric),
                            "unit" => Ok(GeneratedField::Unit),
                            "isPrecise" | "is_precise" => Ok(GeneratedField::IsPrecise),
                            "allowsAdminOverrides" | "allows_admin_overrides" => Ok(GeneratedField::AllowsAdminOverrides),
                            "quotaBuckets" | "quota_buckets" => Ok(GeneratedField::QuotaBuckets),
                            "supportedLocations" | "supported_locations" => Ok(GeneratedField::SupportedLocations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConsumerQuotaLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ConsumerQuotaLimit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConsumerQuotaLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut metric__ = None;
                let mut unit__ = None;
                let mut is_precise__ = None;
                let mut allows_admin_overrides__ = None;
                let mut quota_buckets__ = None;
                let mut supported_locations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metric => {
                            if metric__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metric"));
                            }
                            metric__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsPrecise => {
                            if is_precise__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPrecise"));
                            }
                            is_precise__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowsAdminOverrides => {
                            if allows_admin_overrides__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowsAdminOverrides"));
                            }
                            allows_admin_overrides__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuotaBuckets => {
                            if quota_buckets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotaBuckets"));
                            }
                            quota_buckets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SupportedLocations => {
                            if supported_locations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("supportedLocations"));
                            }
                            supported_locations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ConsumerQuotaLimit {
                    name: name__.unwrap_or_default(),
                    metric: metric__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                    is_precise: is_precise__.unwrap_or_default(),
                    allows_admin_overrides: allows_admin_overrides__.unwrap_or_default(),
                    quota_buckets: quota_buckets__.unwrap_or_default(),
                    supported_locations: supported_locations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ConsumerQuotaLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConsumerQuotaMetric {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.metric.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.consumer_quota_limits.is_empty() {
            len += 1;
        }
        if !self.descendant_consumer_quota_limits.is_empty() {
            len += 1;
        }
        if !self.unit.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ConsumerQuotaMetric", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.metric.is_empty() {
            struct_ser.serialize_field("metric", &self.metric)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.consumer_quota_limits.is_empty() {
            struct_ser.serialize_field("consumerQuotaLimits", &self.consumer_quota_limits)?;
        }
        if !self.descendant_consumer_quota_limits.is_empty() {
            struct_ser.serialize_field("descendantConsumerQuotaLimits", &self.descendant_consumer_quota_limits)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConsumerQuotaMetric {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "metric",
            "display_name",
            "displayName",
            "consumer_quota_limits",
            "consumerQuotaLimits",
            "descendant_consumer_quota_limits",
            "descendantConsumerQuotaLimits",
            "unit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Metric,
            DisplayName,
            ConsumerQuotaLimits,
            DescendantConsumerQuotaLimits,
            Unit,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "metric" => Ok(GeneratedField::Metric),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "consumerQuotaLimits" | "consumer_quota_limits" => Ok(GeneratedField::ConsumerQuotaLimits),
                            "descendantConsumerQuotaLimits" | "descendant_consumer_quota_limits" => Ok(GeneratedField::DescendantConsumerQuotaLimits),
                            "unit" => Ok(GeneratedField::Unit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConsumerQuotaMetric;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ConsumerQuotaMetric")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConsumerQuotaMetric, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut metric__ = None;
                let mut display_name__ = None;
                let mut consumer_quota_limits__ = None;
                let mut descendant_consumer_quota_limits__ = None;
                let mut unit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metric => {
                            if metric__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metric"));
                            }
                            metric__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConsumerQuotaLimits => {
                            if consumer_quota_limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consumerQuotaLimits"));
                            }
                            consumer_quota_limits__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DescendantConsumerQuotaLimits => {
                            if descendant_consumer_quota_limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descendantConsumerQuotaLimits"));
                            }
                            descendant_consumer_quota_limits__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ConsumerQuotaMetric {
                    name: name__.unwrap_or_default(),
                    metric: metric__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    consumer_quota_limits: consumer_quota_limits__.unwrap_or_default(),
                    descendant_consumer_quota_limits: descendant_consumer_quota_limits__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ConsumerQuotaMetric", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAdminOverrideRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if self.r#override.is_some() {
            len += 1;
        }
        if self.force {
            len += 1;
        }
        if !self.force_only.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.CreateAdminOverrideRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if let Some(v) = self.r#override.as_ref() {
            struct_ser.serialize_field("override", v)?;
        }
        if self.force {
            struct_ser.serialize_field("force", &self.force)?;
        }
        if !self.force_only.is_empty() {
            let v = self.force_only.iter().cloned().map(|v| {
                QuotaSafetyCheck::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("forceOnly", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAdminOverrideRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "override",
            "force",
            "force_only",
            "forceOnly",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            Override,
            Force,
            ForceOnly,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "parent" => Ok(GeneratedField::Parent),
                            "override" => Ok(GeneratedField::Override),
                            "force" => Ok(GeneratedField::Force),
                            "forceOnly" | "force_only" => Ok(GeneratedField::ForceOnly),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateAdminOverrideRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.CreateAdminOverrideRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAdminOverrideRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut r#override__ = None;
                let mut force__ = None;
                let mut force_only__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Override => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("override"));
                            }
                            r#override__ = map_.next_value()?;
                        }
                        GeneratedField::Force => {
                            if force__.is_some() {
                                return Err(serde::de::Error::duplicate_field("force"));
                            }
                            force__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ForceOnly => {
                            if force_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forceOnly"));
                            }
                            force_only__ = Some(map_.next_value::<Vec<QuotaSafetyCheck>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(CreateAdminOverrideRequest {
                    parent: parent__.unwrap_or_default(),
                    r#override: r#override__,
                    force: force__.unwrap_or_default(),
                    force_only: force_only__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.CreateAdminOverrideRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAdminQuotaPolicyMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.CreateAdminQuotaPolicyMetadata", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAdminQuotaPolicyMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateAdminQuotaPolicyMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.CreateAdminQuotaPolicyMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAdminQuotaPolicyMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(CreateAdminQuotaPolicyMetadata {
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.CreateAdminQuotaPolicyMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateConsumerOverrideRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if self.r#override.is_some() {
            len += 1;
        }
        if self.force {
            len += 1;
        }
        if !self.force_only.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.CreateConsumerOverrideRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if let Some(v) = self.r#override.as_ref() {
            struct_ser.serialize_field("override", v)?;
        }
        if self.force {
            struct_ser.serialize_field("force", &self.force)?;
        }
        if !self.force_only.is_empty() {
            let v = self.force_only.iter().cloned().map(|v| {
                QuotaSafetyCheck::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("forceOnly", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateConsumerOverrideRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "override",
            "force",
            "force_only",
            "forceOnly",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            Override,
            Force,
            ForceOnly,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "parent" => Ok(GeneratedField::Parent),
                            "override" => Ok(GeneratedField::Override),
                            "force" => Ok(GeneratedField::Force),
                            "forceOnly" | "force_only" => Ok(GeneratedField::ForceOnly),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateConsumerOverrideRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.CreateConsumerOverrideRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateConsumerOverrideRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut r#override__ = None;
                let mut force__ = None;
                let mut force_only__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Override => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("override"));
                            }
                            r#override__ = map_.next_value()?;
                        }
                        GeneratedField::Force => {
                            if force__.is_some() {
                                return Err(serde::de::Error::duplicate_field("force"));
                            }
                            force__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ForceOnly => {
                            if force_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forceOnly"));
                            }
                            force_only__ = Some(map_.next_value::<Vec<QuotaSafetyCheck>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(CreateConsumerOverrideRequest {
                    parent: parent__.unwrap_or_default(),
                    r#override: r#override__,
                    force: force__.unwrap_or_default(),
                    force_only: force_only__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.CreateConsumerOverrideRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteAdminOverrideRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.force {
            len += 1;
        }
        if !self.force_only.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.DeleteAdminOverrideRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.force {
            struct_ser.serialize_field("force", &self.force)?;
        }
        if !self.force_only.is_empty() {
            let v = self.force_only.iter().cloned().map(|v| {
                QuotaSafetyCheck::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("forceOnly", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteAdminOverrideRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "force",
            "force_only",
            "forceOnly",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Force,
            ForceOnly,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "force" => Ok(GeneratedField::Force),
                            "forceOnly" | "force_only" => Ok(GeneratedField::ForceOnly),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteAdminOverrideRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.DeleteAdminOverrideRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteAdminOverrideRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut force__ = None;
                let mut force_only__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Force => {
                            if force__.is_some() {
                                return Err(serde::de::Error::duplicate_field("force"));
                            }
                            force__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ForceOnly => {
                            if force_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forceOnly"));
                            }
                            force_only__ = Some(map_.next_value::<Vec<QuotaSafetyCheck>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(DeleteAdminOverrideRequest {
                    name: name__.unwrap_or_default(),
                    force: force__.unwrap_or_default(),
                    force_only: force_only__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.DeleteAdminOverrideRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteAdminQuotaPolicyMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.DeleteAdminQuotaPolicyMetadata", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteAdminQuotaPolicyMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteAdminQuotaPolicyMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.DeleteAdminQuotaPolicyMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteAdminQuotaPolicyMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteAdminQuotaPolicyMetadata {
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.DeleteAdminQuotaPolicyMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteConsumerOverrideRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.force {
            len += 1;
        }
        if !self.force_only.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.DeleteConsumerOverrideRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.force {
            struct_ser.serialize_field("force", &self.force)?;
        }
        if !self.force_only.is_empty() {
            let v = self.force_only.iter().cloned().map(|v| {
                QuotaSafetyCheck::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("forceOnly", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteConsumerOverrideRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "force",
            "force_only",
            "forceOnly",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Force,
            ForceOnly,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "force" => Ok(GeneratedField::Force),
                            "forceOnly" | "force_only" => Ok(GeneratedField::ForceOnly),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteConsumerOverrideRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.DeleteConsumerOverrideRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteConsumerOverrideRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut force__ = None;
                let mut force_only__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Force => {
                            if force__.is_some() {
                                return Err(serde::de::Error::duplicate_field("force"));
                            }
                            force__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ForceOnly => {
                            if force_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forceOnly"));
                            }
                            force_only__ = Some(map_.next_value::<Vec<QuotaSafetyCheck>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(DeleteConsumerOverrideRequest {
                    name: name__.unwrap_or_default(),
                    force: force__.unwrap_or_default(),
                    force_only: force_only__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.DeleteConsumerOverrideRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DisableServiceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.DisableServiceRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DisableServiceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DisableServiceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.DisableServiceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DisableServiceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DisableServiceRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.DisableServiceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnableServiceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.EnableServiceRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnableServiceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnableServiceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.EnableServiceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EnableServiceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EnableServiceRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.EnableServiceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenerateServiceIdentityRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.GenerateServiceIdentityRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenerateServiceIdentityRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "parent" => Ok(GeneratedField::Parent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenerateServiceIdentityRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.GenerateServiceIdentityRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenerateServiceIdentityRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenerateServiceIdentityRequest {
                    parent: parent__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.GenerateServiceIdentityRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetConsumerQuotaLimitRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.view != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.GetConsumerQuotaLimitRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.view != 0 {
            let v = QuotaView::try_from(self.view)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.view)))?;
            struct_ser.serialize_field("view", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetConsumerQuotaLimitRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "view",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            View,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "view" => Ok(GeneratedField::View),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetConsumerQuotaLimitRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.GetConsumerQuotaLimitRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetConsumerQuotaLimitRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut view__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::View => {
                            if view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("view"));
                            }
                            view__ = Some(map_.next_value::<QuotaView>()? as i32);
                        }
                    }
                }
                Ok(GetConsumerQuotaLimitRequest {
                    name: name__.unwrap_or_default(),
                    view: view__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.GetConsumerQuotaLimitRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetConsumerQuotaMetricRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.view != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.GetConsumerQuotaMetricRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.view != 0 {
            let v = QuotaView::try_from(self.view)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.view)))?;
            struct_ser.serialize_field("view", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetConsumerQuotaMetricRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "view",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            View,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "view" => Ok(GeneratedField::View),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetConsumerQuotaMetricRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.GetConsumerQuotaMetricRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetConsumerQuotaMetricRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut view__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::View => {
                            if view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("view"));
                            }
                            view__ = Some(map_.next_value::<QuotaView>()? as i32);
                        }
                    }
                }
                Ok(GetConsumerQuotaMetricRequest {
                    name: name__.unwrap_or_default(),
                    view: view__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.GetConsumerQuotaMetricRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetServiceIdentityMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.GetServiceIdentityMetadata", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetServiceIdentityMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetServiceIdentityMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.GetServiceIdentityMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetServiceIdentityMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetServiceIdentityMetadata {
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.GetServiceIdentityMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetServiceIdentityResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.identity.is_some() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.GetServiceIdentityResponse", len)?;
        if let Some(v) = self.identity.as_ref() {
            struct_ser.serialize_field("identity", v)?;
        }
        if self.state != 0 {
            let v = get_service_identity_response::IdentityState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetServiceIdentityResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "identity",
            "state",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Identity,
            State,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "identity" => Ok(GeneratedField::Identity),
                            "state" => Ok(GeneratedField::State),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetServiceIdentityResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.GetServiceIdentityResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetServiceIdentityResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut identity__ = None;
                let mut state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Identity => {
                            if identity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identity"));
                            }
                            identity__ = map_.next_value()?;
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<get_service_identity_response::IdentityState>()? as i32);
                        }
                    }
                }
                Ok(GetServiceIdentityResponse {
                    identity: identity__,
                    state: state__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.GetServiceIdentityResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_service_identity_response::IdentityState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "IDENTITY_STATE_UNSPECIFIED",
            Self::Active => "ACTIVE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for get_service_identity_response::IdentityState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "IDENTITY_STATE_UNSPECIFIED",
            "ACTIVE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_service_identity_response::IdentityState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "IDENTITY_STATE_UNSPECIFIED" => Ok(get_service_identity_response::IdentityState::Unspecified),
                    "ACTIVE" => Ok(get_service_identity_response::IdentityState::Active),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GetServiceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.GetServiceRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetServiceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetServiceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.GetServiceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetServiceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetServiceRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.GetServiceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportAdminOverridesMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ImportAdminOverridesMetadata", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportAdminOverridesMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportAdminOverridesMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ImportAdminOverridesMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImportAdminOverridesMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ImportAdminOverridesMetadata {
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ImportAdminOverridesMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportAdminOverridesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if self.force {
            len += 1;
        }
        if !self.force_only.is_empty() {
            len += 1;
        }
        if self.source.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ImportAdminOverridesRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if self.force {
            struct_ser.serialize_field("force", &self.force)?;
        }
        if !self.force_only.is_empty() {
            let v = self.force_only.iter().cloned().map(|v| {
                QuotaSafetyCheck::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("forceOnly", &v)?;
        }
        if let Some(v) = self.source.as_ref() {
            match v {
                import_admin_overrides_request::Source::InlineSource(v) => {
                    struct_ser.serialize_field("inlineSource", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportAdminOverridesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "force",
            "force_only",
            "forceOnly",
            "inline_source",
            "inlineSource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            Force,
            ForceOnly,
            InlineSource,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "parent" => Ok(GeneratedField::Parent),
                            "force" => Ok(GeneratedField::Force),
                            "forceOnly" | "force_only" => Ok(GeneratedField::ForceOnly),
                            "inlineSource" | "inline_source" => Ok(GeneratedField::InlineSource),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportAdminOverridesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ImportAdminOverridesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImportAdminOverridesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut force__ = None;
                let mut force_only__ = None;
                let mut source__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Force => {
                            if force__.is_some() {
                                return Err(serde::de::Error::duplicate_field("force"));
                            }
                            force__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ForceOnly => {
                            if force_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forceOnly"));
                            }
                            force_only__ = Some(map_.next_value::<Vec<QuotaSafetyCheck>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::InlineSource => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inlineSource"));
                            }
                            source__ = map_.next_value::<::std::option::Option<_>>()?.map(import_admin_overrides_request::Source::InlineSource)
;
                        }
                    }
                }
                Ok(ImportAdminOverridesRequest {
                    parent: parent__.unwrap_or_default(),
                    force: force__.unwrap_or_default(),
                    force_only: force_only__.unwrap_or_default(),
                    source: source__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ImportAdminOverridesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportAdminOverridesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.overrides.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ImportAdminOverridesResponse", len)?;
        if !self.overrides.is_empty() {
            struct_ser.serialize_field("overrides", &self.overrides)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportAdminOverridesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "overrides",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Overrides,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "overrides" => Ok(GeneratedField::Overrides),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportAdminOverridesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ImportAdminOverridesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImportAdminOverridesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut overrides__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Overrides => {
                            if overrides__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrides"));
                            }
                            overrides__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ImportAdminOverridesResponse {
                    overrides: overrides__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ImportAdminOverridesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportAdminQuotaPoliciesMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ImportAdminQuotaPoliciesMetadata", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportAdminQuotaPoliciesMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportAdminQuotaPoliciesMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ImportAdminQuotaPoliciesMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImportAdminQuotaPoliciesMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ImportAdminQuotaPoliciesMetadata {
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ImportAdminQuotaPoliciesMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportAdminQuotaPoliciesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.policies.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ImportAdminQuotaPoliciesResponse", len)?;
        if !self.policies.is_empty() {
            struct_ser.serialize_field("policies", &self.policies)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportAdminQuotaPoliciesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "policies",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Policies,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "policies" => Ok(GeneratedField::Policies),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportAdminQuotaPoliciesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ImportAdminQuotaPoliciesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImportAdminQuotaPoliciesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut policies__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Policies => {
                            if policies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policies"));
                            }
                            policies__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ImportAdminQuotaPoliciesResponse {
                    policies: policies__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ImportAdminQuotaPoliciesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportConsumerOverridesMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ImportConsumerOverridesMetadata", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportConsumerOverridesMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportConsumerOverridesMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ImportConsumerOverridesMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImportConsumerOverridesMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ImportConsumerOverridesMetadata {
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ImportConsumerOverridesMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportConsumerOverridesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if self.force {
            len += 1;
        }
        if !self.force_only.is_empty() {
            len += 1;
        }
        if self.source.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ImportConsumerOverridesRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if self.force {
            struct_ser.serialize_field("force", &self.force)?;
        }
        if !self.force_only.is_empty() {
            let v = self.force_only.iter().cloned().map(|v| {
                QuotaSafetyCheck::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("forceOnly", &v)?;
        }
        if let Some(v) = self.source.as_ref() {
            match v {
                import_consumer_overrides_request::Source::InlineSource(v) => {
                    struct_ser.serialize_field("inlineSource", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportConsumerOverridesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "force",
            "force_only",
            "forceOnly",
            "inline_source",
            "inlineSource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            Force,
            ForceOnly,
            InlineSource,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "parent" => Ok(GeneratedField::Parent),
                            "force" => Ok(GeneratedField::Force),
                            "forceOnly" | "force_only" => Ok(GeneratedField::ForceOnly),
                            "inlineSource" | "inline_source" => Ok(GeneratedField::InlineSource),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportConsumerOverridesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ImportConsumerOverridesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImportConsumerOverridesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut force__ = None;
                let mut force_only__ = None;
                let mut source__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Force => {
                            if force__.is_some() {
                                return Err(serde::de::Error::duplicate_field("force"));
                            }
                            force__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ForceOnly => {
                            if force_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forceOnly"));
                            }
                            force_only__ = Some(map_.next_value::<Vec<QuotaSafetyCheck>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::InlineSource => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inlineSource"));
                            }
                            source__ = map_.next_value::<::std::option::Option<_>>()?.map(import_consumer_overrides_request::Source::InlineSource)
;
                        }
                    }
                }
                Ok(ImportConsumerOverridesRequest {
                    parent: parent__.unwrap_or_default(),
                    force: force__.unwrap_or_default(),
                    force_only: force_only__.unwrap_or_default(),
                    source: source__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ImportConsumerOverridesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportConsumerOverridesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.overrides.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ImportConsumerOverridesResponse", len)?;
        if !self.overrides.is_empty() {
            struct_ser.serialize_field("overrides", &self.overrides)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportConsumerOverridesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "overrides",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Overrides,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "overrides" => Ok(GeneratedField::Overrides),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportConsumerOverridesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ImportConsumerOverridesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImportConsumerOverridesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut overrides__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Overrides => {
                            if overrides__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrides"));
                            }
                            overrides__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ImportConsumerOverridesResponse {
                    overrides: overrides__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ImportConsumerOverridesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAdminOverridesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ListAdminOverridesRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAdminOverridesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            PageSize,
            PageToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "parent" => Ok(GeneratedField::Parent),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListAdminOverridesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ListAdminOverridesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAdminOverridesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListAdminOverridesRequest {
                    parent: parent__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ListAdminOverridesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAdminOverridesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.overrides.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ListAdminOverridesResponse", len)?;
        if !self.overrides.is_empty() {
            struct_ser.serialize_field("overrides", &self.overrides)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAdminOverridesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "overrides",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Overrides,
            NextPageToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "overrides" => Ok(GeneratedField::Overrides),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListAdminOverridesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ListAdminOverridesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAdminOverridesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut overrides__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Overrides => {
                            if overrides__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrides"));
                            }
                            overrides__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListAdminOverridesResponse {
                    overrides: overrides__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ListAdminOverridesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListConsumerOverridesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ListConsumerOverridesRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListConsumerOverridesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            PageSize,
            PageToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "parent" => Ok(GeneratedField::Parent),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListConsumerOverridesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ListConsumerOverridesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListConsumerOverridesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListConsumerOverridesRequest {
                    parent: parent__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ListConsumerOverridesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListConsumerOverridesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.overrides.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ListConsumerOverridesResponse", len)?;
        if !self.overrides.is_empty() {
            struct_ser.serialize_field("overrides", &self.overrides)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListConsumerOverridesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "overrides",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Overrides,
            NextPageToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "overrides" => Ok(GeneratedField::Overrides),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListConsumerOverridesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ListConsumerOverridesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListConsumerOverridesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut overrides__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Overrides => {
                            if overrides__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrides"));
                            }
                            overrides__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListConsumerOverridesResponse {
                    overrides: overrides__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ListConsumerOverridesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListConsumerQuotaMetricsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        if self.view != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ListConsumerQuotaMetricsRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if self.view != 0 {
            let v = QuotaView::try_from(self.view)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.view)))?;
            struct_ser.serialize_field("view", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListConsumerQuotaMetricsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
            "view",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            PageSize,
            PageToken,
            View,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "parent" => Ok(GeneratedField::Parent),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "view" => Ok(GeneratedField::View),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListConsumerQuotaMetricsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ListConsumerQuotaMetricsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListConsumerQuotaMetricsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut view__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::View => {
                            if view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("view"));
                            }
                            view__ = Some(map_.next_value::<QuotaView>()? as i32);
                        }
                    }
                }
                Ok(ListConsumerQuotaMetricsRequest {
                    parent: parent__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    view: view__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ListConsumerQuotaMetricsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListConsumerQuotaMetricsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.metrics.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ListConsumerQuotaMetricsResponse", len)?;
        if !self.metrics.is_empty() {
            struct_ser.serialize_field("metrics", &self.metrics)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListConsumerQuotaMetricsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metrics",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metrics,
            NextPageToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "metrics" => Ok(GeneratedField::Metrics),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListConsumerQuotaMetricsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ListConsumerQuotaMetricsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListConsumerQuotaMetricsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metrics__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metrics => {
                            if metrics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metrics"));
                            }
                            metrics__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListConsumerQuotaMetricsResponse {
                    metrics: metrics__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ListConsumerQuotaMetricsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListServicesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        if !self.filter.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ListServicesRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListServicesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
            "filter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            PageSize,
            PageToken,
            Filter,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "parent" => Ok(GeneratedField::Parent),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "filter" => Ok(GeneratedField::Filter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListServicesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ListServicesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListServicesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListServicesRequest {
                    parent: parent__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ListServicesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListServicesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.services.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ListServicesResponse", len)?;
        if !self.services.is_empty() {
            struct_ser.serialize_field("services", &self.services)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListServicesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "services",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Services,
            NextPageToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "services" => Ok(GeneratedField::Services),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListServicesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ListServicesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListServicesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut services__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Services => {
                            if services__.is_some() {
                                return Err(serde::de::Error::duplicate_field("services"));
                            }
                            services__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListServicesResponse {
                    services: services__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ListServicesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OperationMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_names.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.OperationMetadata", len)?;
        if !self.resource_names.is_empty() {
            struct_ser.serialize_field("resourceNames", &self.resource_names)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OperationMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_names",
            "resourceNames",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceNames,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "resourceNames" | "resource_names" => Ok(GeneratedField::ResourceNames),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OperationMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.OperationMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OperationMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_names__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceNames => {
                            if resource_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceNames"));
                            }
                            resource_names__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OperationMetadata {
                    resource_names: resource_names__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.OperationMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OverrideInlineSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.overrides.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.OverrideInlineSource", len)?;
        if !self.overrides.is_empty() {
            struct_ser.serialize_field("overrides", &self.overrides)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OverrideInlineSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "overrides",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Overrides,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "overrides" => Ok(GeneratedField::Overrides),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OverrideInlineSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.OverrideInlineSource")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OverrideInlineSource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut overrides__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Overrides => {
                            if overrides__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrides"));
                            }
                            overrides__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OverrideInlineSource {
                    overrides: overrides__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.OverrideInlineSource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProducerQuotaPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.policy_value != 0 {
            len += 1;
        }
        if !self.dimensions.is_empty() {
            len += 1;
        }
        if !self.metric.is_empty() {
            len += 1;
        }
        if !self.unit.is_empty() {
            len += 1;
        }
        if !self.container.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ProducerQuotaPolicy", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.policy_value != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("policyValue", ToString::to_string(&self.policy_value).as_str())?;
        }
        if !self.dimensions.is_empty() {
            struct_ser.serialize_field("dimensions", &self.dimensions)?;
        }
        if !self.metric.is_empty() {
            struct_ser.serialize_field("metric", &self.metric)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
        }
        if !self.container.is_empty() {
            struct_ser.serialize_field("container", &self.container)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProducerQuotaPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "policy_value",
            "policyValue",
            "dimensions",
            "metric",
            "unit",
            "container",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            PolicyValue,
            Dimensions,
            Metric,
            Unit,
            Container,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "policyValue" | "policy_value" => Ok(GeneratedField::PolicyValue),
                            "dimensions" => Ok(GeneratedField::Dimensions),
                            "metric" => Ok(GeneratedField::Metric),
                            "unit" => Ok(GeneratedField::Unit),
                            "container" => Ok(GeneratedField::Container),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProducerQuotaPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ProducerQuotaPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProducerQuotaPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut policy_value__ = None;
                let mut dimensions__ = None;
                let mut metric__ = None;
                let mut unit__ = None;
                let mut container__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PolicyValue => {
                            if policy_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyValue"));
                            }
                            policy_value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Dimensions => {
                            if dimensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dimensions"));
                            }
                            dimensions__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Metric => {
                            if metric__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metric"));
                            }
                            metric__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Container => {
                            if container__.is_some() {
                                return Err(serde::de::Error::duplicate_field("container"));
                            }
                            container__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProducerQuotaPolicy {
                    name: name__.unwrap_or_default(),
                    policy_value: policy_value__.unwrap_or_default(),
                    dimensions: dimensions__.unwrap_or_default(),
                    metric: metric__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                    container: container__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ProducerQuotaPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaBucket {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.effective_limit != 0 {
            len += 1;
        }
        if self.default_limit != 0 {
            len += 1;
        }
        if self.producer_override.is_some() {
            len += 1;
        }
        if self.consumer_override.is_some() {
            len += 1;
        }
        if self.admin_override.is_some() {
            len += 1;
        }
        if self.producer_quota_policy.is_some() {
            len += 1;
        }
        if !self.dimensions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.QuotaBucket", len)?;
        if self.effective_limit != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("effectiveLimit", ToString::to_string(&self.effective_limit).as_str())?;
        }
        if self.default_limit != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("defaultLimit", ToString::to_string(&self.default_limit).as_str())?;
        }
        if let Some(v) = self.producer_override.as_ref() {
            struct_ser.serialize_field("producerOverride", v)?;
        }
        if let Some(v) = self.consumer_override.as_ref() {
            struct_ser.serialize_field("consumerOverride", v)?;
        }
        if let Some(v) = self.admin_override.as_ref() {
            struct_ser.serialize_field("adminOverride", v)?;
        }
        if let Some(v) = self.producer_quota_policy.as_ref() {
            struct_ser.serialize_field("producerQuotaPolicy", v)?;
        }
        if !self.dimensions.is_empty() {
            struct_ser.serialize_field("dimensions", &self.dimensions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuotaBucket {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "effective_limit",
            "effectiveLimit",
            "default_limit",
            "defaultLimit",
            "producer_override",
            "producerOverride",
            "consumer_override",
            "consumerOverride",
            "admin_override",
            "adminOverride",
            "producer_quota_policy",
            "producerQuotaPolicy",
            "dimensions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EffectiveLimit,
            DefaultLimit,
            ProducerOverride,
            ConsumerOverride,
            AdminOverride,
            ProducerQuotaPolicy,
            Dimensions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "effectiveLimit" | "effective_limit" => Ok(GeneratedField::EffectiveLimit),
                            "defaultLimit" | "default_limit" => Ok(GeneratedField::DefaultLimit),
                            "producerOverride" | "producer_override" => Ok(GeneratedField::ProducerOverride),
                            "consumerOverride" | "consumer_override" => Ok(GeneratedField::ConsumerOverride),
                            "adminOverride" | "admin_override" => Ok(GeneratedField::AdminOverride),
                            "producerQuotaPolicy" | "producer_quota_policy" => Ok(GeneratedField::ProducerQuotaPolicy),
                            "dimensions" => Ok(GeneratedField::Dimensions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaBucket;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.QuotaBucket")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuotaBucket, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut effective_limit__ = None;
                let mut default_limit__ = None;
                let mut producer_override__ = None;
                let mut consumer_override__ = None;
                let mut admin_override__ = None;
                let mut producer_quota_policy__ = None;
                let mut dimensions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EffectiveLimit => {
                            if effective_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectiveLimit"));
                            }
                            effective_limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DefaultLimit => {
                            if default_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultLimit"));
                            }
                            default_limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProducerOverride => {
                            if producer_override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("producerOverride"));
                            }
                            producer_override__ = map_.next_value()?;
                        }
                        GeneratedField::ConsumerOverride => {
                            if consumer_override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consumerOverride"));
                            }
                            consumer_override__ = map_.next_value()?;
                        }
                        GeneratedField::AdminOverride => {
                            if admin_override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adminOverride"));
                            }
                            admin_override__ = map_.next_value()?;
                        }
                        GeneratedField::ProducerQuotaPolicy => {
                            if producer_quota_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("producerQuotaPolicy"));
                            }
                            producer_quota_policy__ = map_.next_value()?;
                        }
                        GeneratedField::Dimensions => {
                            if dimensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dimensions"));
                            }
                            dimensions__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(QuotaBucket {
                    effective_limit: effective_limit__.unwrap_or_default(),
                    default_limit: default_limit__.unwrap_or_default(),
                    producer_override: producer_override__,
                    consumer_override: consumer_override__,
                    admin_override: admin_override__,
                    producer_quota_policy: producer_quota_policy__,
                    dimensions: dimensions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.QuotaBucket", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaOverride {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.override_value != 0 {
            len += 1;
        }
        if !self.dimensions.is_empty() {
            len += 1;
        }
        if !self.metric.is_empty() {
            len += 1;
        }
        if !self.unit.is_empty() {
            len += 1;
        }
        if !self.admin_override_ancestor.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.QuotaOverride", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.override_value != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("overrideValue", ToString::to_string(&self.override_value).as_str())?;
        }
        if !self.dimensions.is_empty() {
            struct_ser.serialize_field("dimensions", &self.dimensions)?;
        }
        if !self.metric.is_empty() {
            struct_ser.serialize_field("metric", &self.metric)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
        }
        if !self.admin_override_ancestor.is_empty() {
            struct_ser.serialize_field("adminOverrideAncestor", &self.admin_override_ancestor)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuotaOverride {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "override_value",
            "overrideValue",
            "dimensions",
            "metric",
            "unit",
            "admin_override_ancestor",
            "adminOverrideAncestor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            OverrideValue,
            Dimensions,
            Metric,
            Unit,
            AdminOverrideAncestor,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "overrideValue" | "override_value" => Ok(GeneratedField::OverrideValue),
                            "dimensions" => Ok(GeneratedField::Dimensions),
                            "metric" => Ok(GeneratedField::Metric),
                            "unit" => Ok(GeneratedField::Unit),
                            "adminOverrideAncestor" | "admin_override_ancestor" => Ok(GeneratedField::AdminOverrideAncestor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaOverride;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.QuotaOverride")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuotaOverride, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut override_value__ = None;
                let mut dimensions__ = None;
                let mut metric__ = None;
                let mut unit__ = None;
                let mut admin_override_ancestor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OverrideValue => {
                            if override_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrideValue"));
                            }
                            override_value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Dimensions => {
                            if dimensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dimensions"));
                            }
                            dimensions__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Metric => {
                            if metric__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metric"));
                            }
                            metric__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AdminOverrideAncestor => {
                            if admin_override_ancestor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adminOverrideAncestor"));
                            }
                            admin_override_ancestor__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QuotaOverride {
                    name: name__.unwrap_or_default(),
                    override_value: override_value__.unwrap_or_default(),
                    dimensions: dimensions__.unwrap_or_default(),
                    metric: metric__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                    admin_override_ancestor: admin_override_ancestor__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.QuotaOverride", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaSafetyCheck {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "QUOTA_SAFETY_CHECK_UNSPECIFIED",
            Self::LimitDecreaseBelowUsage => "LIMIT_DECREASE_BELOW_USAGE",
            Self::LimitDecreasePercentageTooHigh => "LIMIT_DECREASE_PERCENTAGE_TOO_HIGH",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for QuotaSafetyCheck {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "QUOTA_SAFETY_CHECK_UNSPECIFIED",
            "LIMIT_DECREASE_BELOW_USAGE",
            "LIMIT_DECREASE_PERCENTAGE_TOO_HIGH",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaSafetyCheck;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "QUOTA_SAFETY_CHECK_UNSPECIFIED" => Ok(QuotaSafetyCheck::Unspecified),
                    "LIMIT_DECREASE_BELOW_USAGE" => Ok(QuotaSafetyCheck::LimitDecreaseBelowUsage),
                    "LIMIT_DECREASE_PERCENTAGE_TOO_HIGH" => Ok(QuotaSafetyCheck::LimitDecreasePercentageTooHigh),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaView {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "QUOTA_VIEW_UNSPECIFIED",
            Self::Basic => "BASIC",
            Self::Full => "FULL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for QuotaView {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "QUOTA_VIEW_UNSPECIFIED",
            "BASIC",
            "FULL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaView;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "QUOTA_VIEW_UNSPECIFIED" => Ok(QuotaView::Unspecified),
                    "BASIC" => Ok(QuotaView::Basic),
                    "FULL" => Ok(QuotaView::Full),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Service {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.parent.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.Service", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if self.state != 0 {
            let v = State::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Service {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "parent",
            "config",
            "state",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Parent,
            Config,
            State,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "parent" => Ok(GeneratedField::Parent),
                            "config" => Ok(GeneratedField::Config),
                            "state" => Ok(GeneratedField::State),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Service;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.Service")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Service, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut parent__ = None;
                let mut config__ = None;
                let mut state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<State>()? as i32);
                        }
                    }
                }
                Ok(Service {
                    name: name__.unwrap_or_default(),
                    parent: parent__.unwrap_or_default(),
                    config: config__,
                    state: state__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.Service", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.apis.is_empty() {
            len += 1;
        }
        if self.documentation.is_some() {
            len += 1;
        }
        if self.quota.is_some() {
            len += 1;
        }
        if self.authentication.is_some() {
            len += 1;
        }
        if self.usage.is_some() {
            len += 1;
        }
        if !self.endpoints.is_empty() {
            len += 1;
        }
        if !self.monitored_resources.is_empty() {
            len += 1;
        }
        if self.monitoring.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ServiceConfig", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.apis.is_empty() {
            struct_ser.serialize_field("apis", &self.apis)?;
        }
        if let Some(v) = self.documentation.as_ref() {
            struct_ser.serialize_field("documentation", v)?;
        }
        if let Some(v) = self.quota.as_ref() {
            struct_ser.serialize_field("quota", v)?;
        }
        if let Some(v) = self.authentication.as_ref() {
            struct_ser.serialize_field("authentication", v)?;
        }
        if let Some(v) = self.usage.as_ref() {
            struct_ser.serialize_field("usage", v)?;
        }
        if !self.endpoints.is_empty() {
            struct_ser.serialize_field("endpoints", &self.endpoints)?;
        }
        if !self.monitored_resources.is_empty() {
            struct_ser.serialize_field("monitoredResources", &self.monitored_resources)?;
        }
        if let Some(v) = self.monitoring.as_ref() {
            struct_ser.serialize_field("monitoring", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServiceConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "title",
            "apis",
            "documentation",
            "quota",
            "authentication",
            "usage",
            "endpoints",
            "monitored_resources",
            "monitoredResources",
            "monitoring",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Title,
            Apis,
            Documentation,
            Quota,
            Authentication,
            Usage,
            Endpoints,
            MonitoredResources,
            Monitoring,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "title" => Ok(GeneratedField::Title),
                            "apis" => Ok(GeneratedField::Apis),
                            "documentation" => Ok(GeneratedField::Documentation),
                            "quota" => Ok(GeneratedField::Quota),
                            "authentication" => Ok(GeneratedField::Authentication),
                            "usage" => Ok(GeneratedField::Usage),
                            "endpoints" => Ok(GeneratedField::Endpoints),
                            "monitoredResources" | "monitored_resources" => Ok(GeneratedField::MonitoredResources),
                            "monitoring" => Ok(GeneratedField::Monitoring),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ServiceConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ServiceConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut title__ = None;
                let mut apis__ = None;
                let mut documentation__ = None;
                let mut quota__ = None;
                let mut authentication__ = None;
                let mut usage__ = None;
                let mut endpoints__ = None;
                let mut monitored_resources__ = None;
                let mut monitoring__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Apis => {
                            if apis__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apis"));
                            }
                            apis__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Documentation => {
                            if documentation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentation"));
                            }
                            documentation__ = map_.next_value()?;
                        }
                        GeneratedField::Quota => {
                            if quota__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota"));
                            }
                            quota__ = map_.next_value()?;
                        }
                        GeneratedField::Authentication => {
                            if authentication__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authentication"));
                            }
                            authentication__ = map_.next_value()?;
                        }
                        GeneratedField::Usage => {
                            if usage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usage"));
                            }
                            usage__ = map_.next_value()?;
                        }
                        GeneratedField::Endpoints => {
                            if endpoints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoints"));
                            }
                            endpoints__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MonitoredResources => {
                            if monitored_resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("monitoredResources"));
                            }
                            monitored_resources__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Monitoring => {
                            if monitoring__.is_some() {
                                return Err(serde::de::Error::duplicate_field("monitoring"));
                            }
                            monitoring__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ServiceConfig {
                    name: name__.unwrap_or_default(),
                    title: title__.unwrap_or_default(),
                    apis: apis__.unwrap_or_default(),
                    documentation: documentation__,
                    quota: quota__,
                    authentication: authentication__,
                    usage: usage__,
                    endpoints: endpoints__.unwrap_or_default(),
                    monitored_resources: monitored_resources__.unwrap_or_default(),
                    monitoring: monitoring__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ServiceConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceIdentity {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.email.is_empty() {
            len += 1;
        }
        if !self.unique_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.ServiceIdentity", len)?;
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if !self.unique_id.is_empty() {
            struct_ser.serialize_field("uniqueId", &self.unique_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServiceIdentity {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "email",
            "unique_id",
            "uniqueId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Email,
            UniqueId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "email" => Ok(GeneratedField::Email),
                            "uniqueId" | "unique_id" => Ok(GeneratedField::UniqueId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceIdentity;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.ServiceIdentity")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ServiceIdentity, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut email__ = None;
                let mut unique_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UniqueId => {
                            if unique_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uniqueId"));
                            }
                            unique_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ServiceIdentity {
                    email: email__.unwrap_or_default(),
                    unique_id: unique_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.ServiceIdentity", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for State {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "STATE_UNSPECIFIED",
            Self::Disabled => "DISABLED",
            Self::Enabled => "ENABLED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for State {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STATE_UNSPECIFIED",
            "DISABLED",
            "ENABLED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = State;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "STATE_UNSPECIFIED" => Ok(State::Unspecified),
                    "DISABLED" => Ok(State::Disabled),
                    "ENABLED" => Ok(State::Enabled),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAdminOverrideRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.r#override.is_some() {
            len += 1;
        }
        if self.force {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        if !self.force_only.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.UpdateAdminOverrideRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.r#override.as_ref() {
            struct_ser.serialize_field("override", v)?;
        }
        if self.force {
            struct_ser.serialize_field("force", &self.force)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        if !self.force_only.is_empty() {
            let v = self.force_only.iter().cloned().map(|v| {
                QuotaSafetyCheck::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("forceOnly", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAdminOverrideRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "override",
            "force",
            "update_mask",
            "updateMask",
            "force_only",
            "forceOnly",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Override,
            Force,
            UpdateMask,
            ForceOnly,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "override" => Ok(GeneratedField::Override),
                            "force" => Ok(GeneratedField::Force),
                            "updateMask" | "update_mask" => Ok(GeneratedField::UpdateMask),
                            "forceOnly" | "force_only" => Ok(GeneratedField::ForceOnly),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAdminOverrideRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.UpdateAdminOverrideRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAdminOverrideRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut r#override__ = None;
                let mut force__ = None;
                let mut update_mask__ = None;
                let mut force_only__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Override => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("override"));
                            }
                            r#override__ = map_.next_value()?;
                        }
                        GeneratedField::Force => {
                            if force__.is_some() {
                                return Err(serde::de::Error::duplicate_field("force"));
                            }
                            force__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                        GeneratedField::ForceOnly => {
                            if force_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forceOnly"));
                            }
                            force_only__ = Some(map_.next_value::<Vec<QuotaSafetyCheck>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(UpdateAdminOverrideRequest {
                    name: name__.unwrap_or_default(),
                    r#override: r#override__,
                    force: force__.unwrap_or_default(),
                    update_mask: update_mask__,
                    force_only: force_only__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.UpdateAdminOverrideRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAdminQuotaPolicyMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.UpdateAdminQuotaPolicyMetadata", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAdminQuotaPolicyMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAdminQuotaPolicyMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.UpdateAdminQuotaPolicyMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAdminQuotaPolicyMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UpdateAdminQuotaPolicyMetadata {
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.UpdateAdminQuotaPolicyMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateConsumerOverrideRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.r#override.is_some() {
            len += 1;
        }
        if self.force {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        if !self.force_only.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.serviceusage.v1beta1.UpdateConsumerOverrideRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.r#override.as_ref() {
            struct_ser.serialize_field("override", v)?;
        }
        if self.force {
            struct_ser.serialize_field("force", &self.force)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        if !self.force_only.is_empty() {
            let v = self.force_only.iter().cloned().map(|v| {
                QuotaSafetyCheck::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("forceOnly", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateConsumerOverrideRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "override",
            "force",
            "update_mask",
            "updateMask",
            "force_only",
            "forceOnly",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Override,
            Force,
            UpdateMask,
            ForceOnly,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "override" => Ok(GeneratedField::Override),
                            "force" => Ok(GeneratedField::Force),
                            "updateMask" | "update_mask" => Ok(GeneratedField::UpdateMask),
                            "forceOnly" | "force_only" => Ok(GeneratedField::ForceOnly),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateConsumerOverrideRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.serviceusage.v1beta1.UpdateConsumerOverrideRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateConsumerOverrideRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut r#override__ = None;
                let mut force__ = None;
                let mut update_mask__ = None;
                let mut force_only__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Override => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("override"));
                            }
                            r#override__ = map_.next_value()?;
                        }
                        GeneratedField::Force => {
                            if force__.is_some() {
                                return Err(serde::de::Error::duplicate_field("force"));
                            }
                            force__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                        GeneratedField::ForceOnly => {
                            if force_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forceOnly"));
                            }
                            force_only__ = Some(map_.next_value::<Vec<QuotaSafetyCheck>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(UpdateConsumerOverrideRequest {
                    name: name__.unwrap_or_default(),
                    r#override: r#override__,
                    force: force__.unwrap_or_default(),
                    update_mask: update_mask__,
                    force_only: force_only__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.serviceusage.v1beta1.UpdateConsumerOverrideRequest", FIELDS, GeneratedVisitor)
    }
}
