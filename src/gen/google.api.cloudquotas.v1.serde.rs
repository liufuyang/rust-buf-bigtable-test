// @generated
impl serde::Serialize for CreateQuotaPreferenceRequest {
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
        if !self.quota_preference_id.is_empty() {
            len += 1;
        }
        if self.quota_preference.is_some() {
            len += 1;
        }
        if !self.ignore_safety_checks.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.cloudquotas.v1.CreateQuotaPreferenceRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if !self.quota_preference_id.is_empty() {
            struct_ser.serialize_field("quotaPreferenceId", &self.quota_preference_id)?;
        }
        if let Some(v) = self.quota_preference.as_ref() {
            struct_ser.serialize_field("quotaPreference", v)?;
        }
        if !self.ignore_safety_checks.is_empty() {
            let v = self.ignore_safety_checks.iter().cloned().map(|v| {
                QuotaSafetyCheck::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("ignoreSafetyChecks", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateQuotaPreferenceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "quota_preference_id",
            "quotaPreferenceId",
            "quota_preference",
            "quotaPreference",
            "ignore_safety_checks",
            "ignoreSafetyChecks",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            QuotaPreferenceId,
            QuotaPreference,
            IgnoreSafetyChecks,
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
                            "quotaPreferenceId" | "quota_preference_id" => Ok(GeneratedField::QuotaPreferenceId),
                            "quotaPreference" | "quota_preference" => Ok(GeneratedField::QuotaPreference),
                            "ignoreSafetyChecks" | "ignore_safety_checks" => Ok(GeneratedField::IgnoreSafetyChecks),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateQuotaPreferenceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.cloudquotas.v1.CreateQuotaPreferenceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateQuotaPreferenceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut quota_preference_id__ = None;
                let mut quota_preference__ = None;
                let mut ignore_safety_checks__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuotaPreferenceId => {
                            if quota_preference_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotaPreferenceId"));
                            }
                            quota_preference_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuotaPreference => {
                            if quota_preference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotaPreference"));
                            }
                            quota_preference__ = map_.next_value()?;
                        }
                        GeneratedField::IgnoreSafetyChecks => {
                            if ignore_safety_checks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreSafetyChecks"));
                            }
                            ignore_safety_checks__ = Some(map_.next_value::<Vec<QuotaSafetyCheck>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(CreateQuotaPreferenceRequest {
                    parent: parent__.unwrap_or_default(),
                    quota_preference_id: quota_preference_id__.unwrap_or_default(),
                    quota_preference: quota_preference__,
                    ignore_safety_checks: ignore_safety_checks__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.cloudquotas.v1.CreateQuotaPreferenceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DimensionsInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.dimensions.is_empty() {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        if !self.applicable_locations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.cloudquotas.v1.DimensionsInfo", len)?;
        if !self.dimensions.is_empty() {
            struct_ser.serialize_field("dimensions", &self.dimensions)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.applicable_locations.is_empty() {
            struct_ser.serialize_field("applicableLocations", &self.applicable_locations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DimensionsInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dimensions",
            "details",
            "applicable_locations",
            "applicableLocations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Dimensions,
            Details,
            ApplicableLocations,
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
                            "dimensions" => Ok(GeneratedField::Dimensions),
                            "details" => Ok(GeneratedField::Details),
                            "applicableLocations" | "applicable_locations" => Ok(GeneratedField::ApplicableLocations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DimensionsInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.cloudquotas.v1.DimensionsInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DimensionsInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dimensions__ = None;
                let mut details__ = None;
                let mut applicable_locations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Dimensions => {
                            if dimensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dimensions"));
                            }
                            dimensions__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::ApplicableLocations => {
                            if applicable_locations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applicableLocations"));
                            }
                            applicable_locations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DimensionsInfo {
                    dimensions: dimensions__.unwrap_or_default(),
                    details: details__,
                    applicable_locations: applicable_locations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.cloudquotas.v1.DimensionsInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetQuotaInfoRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.api.cloudquotas.v1.GetQuotaInfoRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetQuotaInfoRequest {
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
            type Value = GetQuotaInfoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.cloudquotas.v1.GetQuotaInfoRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetQuotaInfoRequest, V::Error>
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
                Ok(GetQuotaInfoRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.cloudquotas.v1.GetQuotaInfoRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetQuotaPreferenceRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.api.cloudquotas.v1.GetQuotaPreferenceRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetQuotaPreferenceRequest {
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
            type Value = GetQuotaPreferenceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.cloudquotas.v1.GetQuotaPreferenceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetQuotaPreferenceRequest, V::Error>
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
                Ok(GetQuotaPreferenceRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.cloudquotas.v1.GetQuotaPreferenceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListQuotaInfosRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.api.cloudquotas.v1.ListQuotaInfosRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListQuotaInfosRequest {
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
            type Value = ListQuotaInfosRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.cloudquotas.v1.ListQuotaInfosRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListQuotaInfosRequest, V::Error>
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
                Ok(ListQuotaInfosRequest {
                    parent: parent__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.cloudquotas.v1.ListQuotaInfosRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListQuotaInfosResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.quota_infos.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.cloudquotas.v1.ListQuotaInfosResponse", len)?;
        if !self.quota_infos.is_empty() {
            struct_ser.serialize_field("quotaInfos", &self.quota_infos)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListQuotaInfosResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "quota_infos",
            "quotaInfos",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QuotaInfos,
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
                            "quotaInfos" | "quota_infos" => Ok(GeneratedField::QuotaInfos),
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
            type Value = ListQuotaInfosResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.cloudquotas.v1.ListQuotaInfosResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListQuotaInfosResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut quota_infos__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QuotaInfos => {
                            if quota_infos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotaInfos"));
                            }
                            quota_infos__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListQuotaInfosResponse {
                    quota_infos: quota_infos__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.cloudquotas.v1.ListQuotaInfosResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListQuotaPreferencesRequest {
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
        if !self.order_by.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.cloudquotas.v1.ListQuotaPreferencesRequest", len)?;
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
        if !self.order_by.is_empty() {
            struct_ser.serialize_field("orderBy", &self.order_by)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListQuotaPreferencesRequest {
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
            "order_by",
            "orderBy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            PageSize,
            PageToken,
            Filter,
            OrderBy,
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
                            "orderBy" | "order_by" => Ok(GeneratedField::OrderBy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListQuotaPreferencesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.cloudquotas.v1.ListQuotaPreferencesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListQuotaPreferencesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut order_by__ = None;
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
                        GeneratedField::OrderBy => {
                            if order_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderBy"));
                            }
                            order_by__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListQuotaPreferencesRequest {
                    parent: parent__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.cloudquotas.v1.ListQuotaPreferencesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListQuotaPreferencesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.quota_preferences.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        if !self.unreachable.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.cloudquotas.v1.ListQuotaPreferencesResponse", len)?;
        if !self.quota_preferences.is_empty() {
            struct_ser.serialize_field("quotaPreferences", &self.quota_preferences)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        if !self.unreachable.is_empty() {
            struct_ser.serialize_field("unreachable", &self.unreachable)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListQuotaPreferencesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "quota_preferences",
            "quotaPreferences",
            "next_page_token",
            "nextPageToken",
            "unreachable",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QuotaPreferences,
            NextPageToken,
            Unreachable,
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
                            "quotaPreferences" | "quota_preferences" => Ok(GeneratedField::QuotaPreferences),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            "unreachable" => Ok(GeneratedField::Unreachable),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListQuotaPreferencesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.cloudquotas.v1.ListQuotaPreferencesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListQuotaPreferencesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut quota_preferences__ = None;
                let mut next_page_token__ = None;
                let mut unreachable__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QuotaPreferences => {
                            if quota_preferences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotaPreferences"));
                            }
                            quota_preferences__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unreachable => {
                            if unreachable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unreachable"));
                            }
                            unreachable__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListQuotaPreferencesResponse {
                    quota_preferences: quota_preferences__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                    unreachable: unreachable__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.cloudquotas.v1.ListQuotaPreferencesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.preferred_value != 0 {
            len += 1;
        }
        if !self.state_detail.is_empty() {
            len += 1;
        }
        if self.granted_value.is_some() {
            len += 1;
        }
        if !self.trace_id.is_empty() {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        if self.request_origin != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.cloudquotas.v1.QuotaConfig", len)?;
        if self.preferred_value != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("preferredValue", ToString::to_string(&self.preferred_value).as_str())?;
        }
        if !self.state_detail.is_empty() {
            struct_ser.serialize_field("stateDetail", &self.state_detail)?;
        }
        if let Some(v) = self.granted_value.as_ref() {
            struct_ser.serialize_field("grantedValue", v)?;
        }
        if !self.trace_id.is_empty() {
            struct_ser.serialize_field("traceId", &self.trace_id)?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if self.request_origin != 0 {
            let v = quota_config::Origin::try_from(self.request_origin)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.request_origin)))?;
            struct_ser.serialize_field("requestOrigin", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuotaConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "preferred_value",
            "preferredValue",
            "state_detail",
            "stateDetail",
            "granted_value",
            "grantedValue",
            "trace_id",
            "traceId",
            "annotations",
            "request_origin",
            "requestOrigin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PreferredValue,
            StateDetail,
            GrantedValue,
            TraceId,
            Annotations,
            RequestOrigin,
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
                            "preferredValue" | "preferred_value" => Ok(GeneratedField::PreferredValue),
                            "stateDetail" | "state_detail" => Ok(GeneratedField::StateDetail),
                            "grantedValue" | "granted_value" => Ok(GeneratedField::GrantedValue),
                            "traceId" | "trace_id" => Ok(GeneratedField::TraceId),
                            "annotations" => Ok(GeneratedField::Annotations),
                            "requestOrigin" | "request_origin" => Ok(GeneratedField::RequestOrigin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.cloudquotas.v1.QuotaConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuotaConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut preferred_value__ = None;
                let mut state_detail__ = None;
                let mut granted_value__ = None;
                let mut trace_id__ = None;
                let mut annotations__ = None;
                let mut request_origin__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PreferredValue => {
                            if preferred_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferredValue"));
                            }
                            preferred_value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StateDetail => {
                            if state_detail__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stateDetail"));
                            }
                            state_detail__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GrantedValue => {
                            if granted_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grantedValue"));
                            }
                            granted_value__ = map_.next_value()?;
                        }
                        GeneratedField::TraceId => {
                            if trace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceId"));
                            }
                            trace_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::RequestOrigin => {
                            if request_origin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestOrigin"));
                            }
                            request_origin__ = Some(map_.next_value::<quota_config::Origin>()? as i32);
                        }
                    }
                }
                Ok(QuotaConfig {
                    preferred_value: preferred_value__.unwrap_or_default(),
                    state_detail: state_detail__.unwrap_or_default(),
                    granted_value: granted_value__,
                    trace_id: trace_id__.unwrap_or_default(),
                    annotations: annotations__.unwrap_or_default(),
                    request_origin: request_origin__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.cloudquotas.v1.QuotaConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for quota_config::Origin {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ORIGIN_UNSPECIFIED",
            Self::CloudConsole => "CLOUD_CONSOLE",
            Self::AutoAdjuster => "AUTO_ADJUSTER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for quota_config::Origin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ORIGIN_UNSPECIFIED",
            "CLOUD_CONSOLE",
            "AUTO_ADJUSTER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = quota_config::Origin;

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
                    "ORIGIN_UNSPECIFIED" => Ok(quota_config::Origin::Unspecified),
                    "CLOUD_CONSOLE" => Ok(quota_config::Origin::CloudConsole),
                    "AUTO_ADJUSTER" => Ok(quota_config::Origin::AutoAdjuster),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value != 0 {
            len += 1;
        }
        if self.rollout_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.cloudquotas.v1.QuotaDetails", len)?;
        if self.value != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("value", ToString::to_string(&self.value).as_str())?;
        }
        if let Some(v) = self.rollout_info.as_ref() {
            struct_ser.serialize_field("rolloutInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuotaDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
            "rollout_info",
            "rolloutInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            RolloutInfo,
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
                            "value" => Ok(GeneratedField::Value),
                            "rolloutInfo" | "rollout_info" => Ok(GeneratedField::RolloutInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.cloudquotas.v1.QuotaDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuotaDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                let mut rollout_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RolloutInfo => {
                            if rollout_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rolloutInfo"));
                            }
                            rollout_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QuotaDetails {
                    value: value__.unwrap_or_default(),
                    rollout_info: rollout_info__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.cloudquotas.v1.QuotaDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaIncreaseEligibility {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.is_eligible {
            len += 1;
        }
        if self.ineligibility_reason != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.cloudquotas.v1.QuotaIncreaseEligibility", len)?;
        if self.is_eligible {
            struct_ser.serialize_field("isEligible", &self.is_eligible)?;
        }
        if self.ineligibility_reason != 0 {
            let v = quota_increase_eligibility::IneligibilityReason::try_from(self.ineligibility_reason)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.ineligibility_reason)))?;
            struct_ser.serialize_field("ineligibilityReason", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuotaIncreaseEligibility {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "is_eligible",
            "isEligible",
            "ineligibility_reason",
            "ineligibilityReason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IsEligible,
            IneligibilityReason,
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
                            "isEligible" | "is_eligible" => Ok(GeneratedField::IsEligible),
                            "ineligibilityReason" | "ineligibility_reason" => Ok(GeneratedField::IneligibilityReason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaIncreaseEligibility;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.cloudquotas.v1.QuotaIncreaseEligibility")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuotaIncreaseEligibility, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut is_eligible__ = None;
                let mut ineligibility_reason__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IsEligible => {
                            if is_eligible__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isEligible"));
                            }
                            is_eligible__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IneligibilityReason => {
                            if ineligibility_reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ineligibilityReason"));
                            }
                            ineligibility_reason__ = Some(map_.next_value::<quota_increase_eligibility::IneligibilityReason>()? as i32);
                        }
                    }
                }
                Ok(QuotaIncreaseEligibility {
                    is_eligible: is_eligible__.unwrap_or_default(),
                    ineligibility_reason: ineligibility_reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.cloudquotas.v1.QuotaIncreaseEligibility", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for quota_increase_eligibility::IneligibilityReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "INELIGIBILITY_REASON_UNSPECIFIED",
            Self::NoValidBillingAccount => "NO_VALID_BILLING_ACCOUNT",
            Self::NotSupported => "NOT_SUPPORTED",
            Self::NotEnoughUsageHistory => "NOT_ENOUGH_USAGE_HISTORY",
            Self::Other => "OTHER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for quota_increase_eligibility::IneligibilityReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "INELIGIBILITY_REASON_UNSPECIFIED",
            "NO_VALID_BILLING_ACCOUNT",
            "NOT_SUPPORTED",
            "NOT_ENOUGH_USAGE_HISTORY",
            "OTHER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = quota_increase_eligibility::IneligibilityReason;

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
                    "INELIGIBILITY_REASON_UNSPECIFIED" => Ok(quota_increase_eligibility::IneligibilityReason::Unspecified),
                    "NO_VALID_BILLING_ACCOUNT" => Ok(quota_increase_eligibility::IneligibilityReason::NoValidBillingAccount),
                    "NOT_SUPPORTED" => Ok(quota_increase_eligibility::IneligibilityReason::NotSupported),
                    "NOT_ENOUGH_USAGE_HISTORY" => Ok(quota_increase_eligibility::IneligibilityReason::NotEnoughUsageHistory),
                    "OTHER" => Ok(quota_increase_eligibility::IneligibilityReason::Other),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaInfo {
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
        if !self.quota_id.is_empty() {
            len += 1;
        }
        if !self.metric.is_empty() {
            len += 1;
        }
        if !self.service.is_empty() {
            len += 1;
        }
        if self.is_precise {
            len += 1;
        }
        if !self.refresh_interval.is_empty() {
            len += 1;
        }
        if self.container_type != 0 {
            len += 1;
        }
        if !self.dimensions.is_empty() {
            len += 1;
        }
        if !self.metric_display_name.is_empty() {
            len += 1;
        }
        if !self.quota_display_name.is_empty() {
            len += 1;
        }
        if !self.metric_unit.is_empty() {
            len += 1;
        }
        if self.quota_increase_eligibility.is_some() {
            len += 1;
        }
        if self.is_fixed {
            len += 1;
        }
        if !self.dimensions_infos.is_empty() {
            len += 1;
        }
        if self.is_concurrent {
            len += 1;
        }
        if !self.service_request_quota_uri.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.cloudquotas.v1.QuotaInfo", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.quota_id.is_empty() {
            struct_ser.serialize_field("quotaId", &self.quota_id)?;
        }
        if !self.metric.is_empty() {
            struct_ser.serialize_field("metric", &self.metric)?;
        }
        if !self.service.is_empty() {
            struct_ser.serialize_field("service", &self.service)?;
        }
        if self.is_precise {
            struct_ser.serialize_field("isPrecise", &self.is_precise)?;
        }
        if !self.refresh_interval.is_empty() {
            struct_ser.serialize_field("refreshInterval", &self.refresh_interval)?;
        }
        if self.container_type != 0 {
            let v = quota_info::ContainerType::try_from(self.container_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.container_type)))?;
            struct_ser.serialize_field("containerType", &v)?;
        }
        if !self.dimensions.is_empty() {
            struct_ser.serialize_field("dimensions", &self.dimensions)?;
        }
        if !self.metric_display_name.is_empty() {
            struct_ser.serialize_field("metricDisplayName", &self.metric_display_name)?;
        }
        if !self.quota_display_name.is_empty() {
            struct_ser.serialize_field("quotaDisplayName", &self.quota_display_name)?;
        }
        if !self.metric_unit.is_empty() {
            struct_ser.serialize_field("metricUnit", &self.metric_unit)?;
        }
        if let Some(v) = self.quota_increase_eligibility.as_ref() {
            struct_ser.serialize_field("quotaIncreaseEligibility", v)?;
        }
        if self.is_fixed {
            struct_ser.serialize_field("isFixed", &self.is_fixed)?;
        }
        if !self.dimensions_infos.is_empty() {
            struct_ser.serialize_field("dimensionsInfos", &self.dimensions_infos)?;
        }
        if self.is_concurrent {
            struct_ser.serialize_field("isConcurrent", &self.is_concurrent)?;
        }
        if !self.service_request_quota_uri.is_empty() {
            struct_ser.serialize_field("serviceRequestQuotaUri", &self.service_request_quota_uri)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuotaInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "quota_id",
            "quotaId",
            "metric",
            "service",
            "is_precise",
            "isPrecise",
            "refresh_interval",
            "refreshInterval",
            "container_type",
            "containerType",
            "dimensions",
            "metric_display_name",
            "metricDisplayName",
            "quota_display_name",
            "quotaDisplayName",
            "metric_unit",
            "metricUnit",
            "quota_increase_eligibility",
            "quotaIncreaseEligibility",
            "is_fixed",
            "isFixed",
            "dimensions_infos",
            "dimensionsInfos",
            "is_concurrent",
            "isConcurrent",
            "service_request_quota_uri",
            "serviceRequestQuotaUri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            QuotaId,
            Metric,
            Service,
            IsPrecise,
            RefreshInterval,
            ContainerType,
            Dimensions,
            MetricDisplayName,
            QuotaDisplayName,
            MetricUnit,
            QuotaIncreaseEligibility,
            IsFixed,
            DimensionsInfos,
            IsConcurrent,
            ServiceRequestQuotaUri,
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
                            "quotaId" | "quota_id" => Ok(GeneratedField::QuotaId),
                            "metric" => Ok(GeneratedField::Metric),
                            "service" => Ok(GeneratedField::Service),
                            "isPrecise" | "is_precise" => Ok(GeneratedField::IsPrecise),
                            "refreshInterval" | "refresh_interval" => Ok(GeneratedField::RefreshInterval),
                            "containerType" | "container_type" => Ok(GeneratedField::ContainerType),
                            "dimensions" => Ok(GeneratedField::Dimensions),
                            "metricDisplayName" | "metric_display_name" => Ok(GeneratedField::MetricDisplayName),
                            "quotaDisplayName" | "quota_display_name" => Ok(GeneratedField::QuotaDisplayName),
                            "metricUnit" | "metric_unit" => Ok(GeneratedField::MetricUnit),
                            "quotaIncreaseEligibility" | "quota_increase_eligibility" => Ok(GeneratedField::QuotaIncreaseEligibility),
                            "isFixed" | "is_fixed" => Ok(GeneratedField::IsFixed),
                            "dimensionsInfos" | "dimensions_infos" => Ok(GeneratedField::DimensionsInfos),
                            "isConcurrent" | "is_concurrent" => Ok(GeneratedField::IsConcurrent),
                            "serviceRequestQuotaUri" | "service_request_quota_uri" => Ok(GeneratedField::ServiceRequestQuotaUri),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.cloudquotas.v1.QuotaInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuotaInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut quota_id__ = None;
                let mut metric__ = None;
                let mut service__ = None;
                let mut is_precise__ = None;
                let mut refresh_interval__ = None;
                let mut container_type__ = None;
                let mut dimensions__ = None;
                let mut metric_display_name__ = None;
                let mut quota_display_name__ = None;
                let mut metric_unit__ = None;
                let mut quota_increase_eligibility__ = None;
                let mut is_fixed__ = None;
                let mut dimensions_infos__ = None;
                let mut is_concurrent__ = None;
                let mut service_request_quota_uri__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuotaId => {
                            if quota_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotaId"));
                            }
                            quota_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metric => {
                            if metric__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metric"));
                            }
                            metric__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Service => {
                            if service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("service"));
                            }
                            service__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsPrecise => {
                            if is_precise__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPrecise"));
                            }
                            is_precise__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RefreshInterval => {
                            if refresh_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshInterval"));
                            }
                            refresh_interval__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContainerType => {
                            if container_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("containerType"));
                            }
                            container_type__ = Some(map_.next_value::<quota_info::ContainerType>()? as i32);
                        }
                        GeneratedField::Dimensions => {
                            if dimensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dimensions"));
                            }
                            dimensions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MetricDisplayName => {
                            if metric_display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metricDisplayName"));
                            }
                            metric_display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuotaDisplayName => {
                            if quota_display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotaDisplayName"));
                            }
                            quota_display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MetricUnit => {
                            if metric_unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metricUnit"));
                            }
                            metric_unit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuotaIncreaseEligibility => {
                            if quota_increase_eligibility__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotaIncreaseEligibility"));
                            }
                            quota_increase_eligibility__ = map_.next_value()?;
                        }
                        GeneratedField::IsFixed => {
                            if is_fixed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isFixed"));
                            }
                            is_fixed__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DimensionsInfos => {
                            if dimensions_infos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dimensionsInfos"));
                            }
                            dimensions_infos__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsConcurrent => {
                            if is_concurrent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isConcurrent"));
                            }
                            is_concurrent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServiceRequestQuotaUri => {
                            if service_request_quota_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceRequestQuotaUri"));
                            }
                            service_request_quota_uri__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QuotaInfo {
                    name: name__.unwrap_or_default(),
                    quota_id: quota_id__.unwrap_or_default(),
                    metric: metric__.unwrap_or_default(),
                    service: service__.unwrap_or_default(),
                    is_precise: is_precise__.unwrap_or_default(),
                    refresh_interval: refresh_interval__.unwrap_or_default(),
                    container_type: container_type__.unwrap_or_default(),
                    dimensions: dimensions__.unwrap_or_default(),
                    metric_display_name: metric_display_name__.unwrap_or_default(),
                    quota_display_name: quota_display_name__.unwrap_or_default(),
                    metric_unit: metric_unit__.unwrap_or_default(),
                    quota_increase_eligibility: quota_increase_eligibility__,
                    is_fixed: is_fixed__.unwrap_or_default(),
                    dimensions_infos: dimensions_infos__.unwrap_or_default(),
                    is_concurrent: is_concurrent__.unwrap_or_default(),
                    service_request_quota_uri: service_request_quota_uri__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.cloudquotas.v1.QuotaInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for quota_info::ContainerType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CONTAINER_TYPE_UNSPECIFIED",
            Self::Project => "PROJECT",
            Self::Folder => "FOLDER",
            Self::Organization => "ORGANIZATION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for quota_info::ContainerType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CONTAINER_TYPE_UNSPECIFIED",
            "PROJECT",
            "FOLDER",
            "ORGANIZATION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = quota_info::ContainerType;

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
                    "CONTAINER_TYPE_UNSPECIFIED" => Ok(quota_info::ContainerType::Unspecified),
                    "PROJECT" => Ok(quota_info::ContainerType::Project),
                    "FOLDER" => Ok(quota_info::ContainerType::Folder),
                    "ORGANIZATION" => Ok(quota_info::ContainerType::Organization),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaPreference {
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
        if !self.dimensions.is_empty() {
            len += 1;
        }
        if self.quota_config.is_some() {
            len += 1;
        }
        if !self.etag.is_empty() {
            len += 1;
        }
        if self.create_time.is_some() {
            len += 1;
        }
        if self.update_time.is_some() {
            len += 1;
        }
        if !self.service.is_empty() {
            len += 1;
        }
        if !self.quota_id.is_empty() {
            len += 1;
        }
        if self.reconciling {
            len += 1;
        }
        if !self.justification.is_empty() {
            len += 1;
        }
        if !self.contact_email.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.cloudquotas.v1.QuotaPreference", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.dimensions.is_empty() {
            struct_ser.serialize_field("dimensions", &self.dimensions)?;
        }
        if let Some(v) = self.quota_config.as_ref() {
            struct_ser.serialize_field("quotaConfig", v)?;
        }
        if !self.etag.is_empty() {
            struct_ser.serialize_field("etag", &self.etag)?;
        }
        if let Some(v) = self.create_time.as_ref() {
            struct_ser.serialize_field("createTime", v)?;
        }
        if let Some(v) = self.update_time.as_ref() {
            struct_ser.serialize_field("updateTime", v)?;
        }
        if !self.service.is_empty() {
            struct_ser.serialize_field("service", &self.service)?;
        }
        if !self.quota_id.is_empty() {
            struct_ser.serialize_field("quotaId", &self.quota_id)?;
        }
        if self.reconciling {
            struct_ser.serialize_field("reconciling", &self.reconciling)?;
        }
        if !self.justification.is_empty() {
            struct_ser.serialize_field("justification", &self.justification)?;
        }
        if !self.contact_email.is_empty() {
            struct_ser.serialize_field("contactEmail", &self.contact_email)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuotaPreference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "dimensions",
            "quota_config",
            "quotaConfig",
            "etag",
            "create_time",
            "createTime",
            "update_time",
            "updateTime",
            "service",
            "quota_id",
            "quotaId",
            "reconciling",
            "justification",
            "contact_email",
            "contactEmail",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Dimensions,
            QuotaConfig,
            Etag,
            CreateTime,
            UpdateTime,
            Service,
            QuotaId,
            Reconciling,
            Justification,
            ContactEmail,
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
                            "dimensions" => Ok(GeneratedField::Dimensions),
                            "quotaConfig" | "quota_config" => Ok(GeneratedField::QuotaConfig),
                            "etag" => Ok(GeneratedField::Etag),
                            "createTime" | "create_time" => Ok(GeneratedField::CreateTime),
                            "updateTime" | "update_time" => Ok(GeneratedField::UpdateTime),
                            "service" => Ok(GeneratedField::Service),
                            "quotaId" | "quota_id" => Ok(GeneratedField::QuotaId),
                            "reconciling" => Ok(GeneratedField::Reconciling),
                            "justification" => Ok(GeneratedField::Justification),
                            "contactEmail" | "contact_email" => Ok(GeneratedField::ContactEmail),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaPreference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.cloudquotas.v1.QuotaPreference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuotaPreference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut dimensions__ = None;
                let mut quota_config__ = None;
                let mut etag__ = None;
                let mut create_time__ = None;
                let mut update_time__ = None;
                let mut service__ = None;
                let mut quota_id__ = None;
                let mut reconciling__ = None;
                let mut justification__ = None;
                let mut contact_email__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Dimensions => {
                            if dimensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dimensions"));
                            }
                            dimensions__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::QuotaConfig => {
                            if quota_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotaConfig"));
                            }
                            quota_config__ = map_.next_value()?;
                        }
                        GeneratedField::Etag => {
                            if etag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("etag"));
                            }
                            etag__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreateTime => {
                            if create_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createTime"));
                            }
                            create_time__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateTime => {
                            if update_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateTime"));
                            }
                            update_time__ = map_.next_value()?;
                        }
                        GeneratedField::Service => {
                            if service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("service"));
                            }
                            service__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuotaId => {
                            if quota_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotaId"));
                            }
                            quota_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Reconciling => {
                            if reconciling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reconciling"));
                            }
                            reconciling__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Justification => {
                            if justification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("justification"));
                            }
                            justification__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContactEmail => {
                            if contact_email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contactEmail"));
                            }
                            contact_email__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QuotaPreference {
                    name: name__.unwrap_or_default(),
                    dimensions: dimensions__.unwrap_or_default(),
                    quota_config: quota_config__,
                    etag: etag__.unwrap_or_default(),
                    create_time: create_time__,
                    update_time: update_time__,
                    service: service__.unwrap_or_default(),
                    quota_id: quota_id__.unwrap_or_default(),
                    reconciling: reconciling__.unwrap_or_default(),
                    justification: justification__.unwrap_or_default(),
                    contact_email: contact_email__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.cloudquotas.v1.QuotaPreference", FIELDS, GeneratedVisitor)
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
            Self::QuotaDecreaseBelowUsage => "QUOTA_DECREASE_BELOW_USAGE",
            Self::QuotaDecreasePercentageTooHigh => "QUOTA_DECREASE_PERCENTAGE_TOO_HIGH",
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
            "QUOTA_DECREASE_BELOW_USAGE",
            "QUOTA_DECREASE_PERCENTAGE_TOO_HIGH",
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
                    "QUOTA_DECREASE_BELOW_USAGE" => Ok(QuotaSafetyCheck::QuotaDecreaseBelowUsage),
                    "QUOTA_DECREASE_PERCENTAGE_TOO_HIGH" => Ok(QuotaSafetyCheck::QuotaDecreasePercentageTooHigh),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for RolloutInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ongoing_rollout {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.cloudquotas.v1.RolloutInfo", len)?;
        if self.ongoing_rollout {
            struct_ser.serialize_field("ongoingRollout", &self.ongoing_rollout)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RolloutInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ongoing_rollout",
            "ongoingRollout",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OngoingRollout,
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
                            "ongoingRollout" | "ongoing_rollout" => Ok(GeneratedField::OngoingRollout),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RolloutInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.cloudquotas.v1.RolloutInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RolloutInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ongoing_rollout__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OngoingRollout => {
                            if ongoing_rollout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ongoingRollout"));
                            }
                            ongoing_rollout__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RolloutInfo {
                    ongoing_rollout: ongoing_rollout__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.cloudquotas.v1.RolloutInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateQuotaPreferenceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.update_mask.is_some() {
            len += 1;
        }
        if self.quota_preference.is_some() {
            len += 1;
        }
        if self.allow_missing {
            len += 1;
        }
        if self.validate_only {
            len += 1;
        }
        if !self.ignore_safety_checks.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.cloudquotas.v1.UpdateQuotaPreferenceRequest", len)?;
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        if let Some(v) = self.quota_preference.as_ref() {
            struct_ser.serialize_field("quotaPreference", v)?;
        }
        if self.allow_missing {
            struct_ser.serialize_field("allowMissing", &self.allow_missing)?;
        }
        if self.validate_only {
            struct_ser.serialize_field("validateOnly", &self.validate_only)?;
        }
        if !self.ignore_safety_checks.is_empty() {
            let v = self.ignore_safety_checks.iter().cloned().map(|v| {
                QuotaSafetyCheck::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("ignoreSafetyChecks", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateQuotaPreferenceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "update_mask",
            "updateMask",
            "quota_preference",
            "quotaPreference",
            "allow_missing",
            "allowMissing",
            "validate_only",
            "validateOnly",
            "ignore_safety_checks",
            "ignoreSafetyChecks",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UpdateMask,
            QuotaPreference,
            AllowMissing,
            ValidateOnly,
            IgnoreSafetyChecks,
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
                            "updateMask" | "update_mask" => Ok(GeneratedField::UpdateMask),
                            "quotaPreference" | "quota_preference" => Ok(GeneratedField::QuotaPreference),
                            "allowMissing" | "allow_missing" => Ok(GeneratedField::AllowMissing),
                            "validateOnly" | "validate_only" => Ok(GeneratedField::ValidateOnly),
                            "ignoreSafetyChecks" | "ignore_safety_checks" => Ok(GeneratedField::IgnoreSafetyChecks),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateQuotaPreferenceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.cloudquotas.v1.UpdateQuotaPreferenceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateQuotaPreferenceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut update_mask__ = None;
                let mut quota_preference__ = None;
                let mut allow_missing__ = None;
                let mut validate_only__ = None;
                let mut ignore_safety_checks__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                        GeneratedField::QuotaPreference => {
                            if quota_preference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotaPreference"));
                            }
                            quota_preference__ = map_.next_value()?;
                        }
                        GeneratedField::AllowMissing => {
                            if allow_missing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowMissing"));
                            }
                            allow_missing__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValidateOnly => {
                            if validate_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validateOnly"));
                            }
                            validate_only__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IgnoreSafetyChecks => {
                            if ignore_safety_checks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreSafetyChecks"));
                            }
                            ignore_safety_checks__ = Some(map_.next_value::<Vec<QuotaSafetyCheck>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(UpdateQuotaPreferenceRequest {
                    update_mask: update_mask__,
                    quota_preference: quota_preference__,
                    allow_missing: allow_missing__.unwrap_or_default(),
                    validate_only: validate_only__.unwrap_or_default(),
                    ignore_safety_checks: ignore_safety_checks__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.cloudquotas.v1.UpdateQuotaPreferenceRequest", FIELDS, GeneratedVisitor)
    }
}
