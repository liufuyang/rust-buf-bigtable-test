// @generated
impl serde::Serialize for AndroidApplication {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sha1_fingerprint.is_empty() {
            len += 1;
        }
        if !self.package_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.apikeys.v2.AndroidApplication", len)?;
        if !self.sha1_fingerprint.is_empty() {
            struct_ser.serialize_field("sha1Fingerprint", &self.sha1_fingerprint)?;
        }
        if !self.package_name.is_empty() {
            struct_ser.serialize_field("packageName", &self.package_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AndroidApplication {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sha1_fingerprint",
            "sha1Fingerprint",
            "package_name",
            "packageName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sha1Fingerprint,
            PackageName,
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
                            "sha1Fingerprint" | "sha1_fingerprint" => Ok(GeneratedField::Sha1Fingerprint),
                            "packageName" | "package_name" => Ok(GeneratedField::PackageName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AndroidApplication;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.apikeys.v2.AndroidApplication")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AndroidApplication, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sha1_fingerprint__ = None;
                let mut package_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sha1Fingerprint => {
                            if sha1_fingerprint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sha1Fingerprint"));
                            }
                            sha1_fingerprint__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PackageName => {
                            if package_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packageName"));
                            }
                            package_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AndroidApplication {
                    sha1_fingerprint: sha1_fingerprint__.unwrap_or_default(),
                    package_name: package_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.apikeys.v2.AndroidApplication", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AndroidKeyRestrictions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.allowed_applications.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.apikeys.v2.AndroidKeyRestrictions", len)?;
        if !self.allowed_applications.is_empty() {
            struct_ser.serialize_field("allowedApplications", &self.allowed_applications)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AndroidKeyRestrictions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allowed_applications",
            "allowedApplications",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowedApplications,
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
                            "allowedApplications" | "allowed_applications" => Ok(GeneratedField::AllowedApplications),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AndroidKeyRestrictions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.apikeys.v2.AndroidKeyRestrictions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AndroidKeyRestrictions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allowed_applications__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AllowedApplications => {
                            if allowed_applications__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedApplications"));
                            }
                            allowed_applications__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AndroidKeyRestrictions {
                    allowed_applications: allowed_applications__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.apikeys.v2.AndroidKeyRestrictions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ApiTarget {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.service.is_empty() {
            len += 1;
        }
        if !self.methods.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.apikeys.v2.ApiTarget", len)?;
        if !self.service.is_empty() {
            struct_ser.serialize_field("service", &self.service)?;
        }
        if !self.methods.is_empty() {
            struct_ser.serialize_field("methods", &self.methods)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApiTarget {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service",
            "methods",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Service,
            Methods,
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
                            "service" => Ok(GeneratedField::Service),
                            "methods" => Ok(GeneratedField::Methods),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApiTarget;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.apikeys.v2.ApiTarget")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ApiTarget, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service__ = None;
                let mut methods__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Service => {
                            if service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("service"));
                            }
                            service__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Methods => {
                            if methods__.is_some() {
                                return Err(serde::de::Error::duplicate_field("methods"));
                            }
                            methods__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ApiTarget {
                    service: service__.unwrap_or_default(),
                    methods: methods__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.apikeys.v2.ApiTarget", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BrowserKeyRestrictions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.allowed_referrers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.apikeys.v2.BrowserKeyRestrictions", len)?;
        if !self.allowed_referrers.is_empty() {
            struct_ser.serialize_field("allowedReferrers", &self.allowed_referrers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BrowserKeyRestrictions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allowed_referrers",
            "allowedReferrers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowedReferrers,
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
                            "allowedReferrers" | "allowed_referrers" => Ok(GeneratedField::AllowedReferrers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BrowserKeyRestrictions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.apikeys.v2.BrowserKeyRestrictions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BrowserKeyRestrictions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allowed_referrers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AllowedReferrers => {
                            if allowed_referrers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedReferrers"));
                            }
                            allowed_referrers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BrowserKeyRestrictions {
                    allowed_referrers: allowed_referrers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.apikeys.v2.BrowserKeyRestrictions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateKeyRequest {
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
        if self.key.is_some() {
            len += 1;
        }
        if !self.key_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.apikeys.v2.CreateKeyRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if !self.key_id.is_empty() {
            struct_ser.serialize_field("keyId", &self.key_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "key",
            "key_id",
            "keyId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            Key,
            KeyId,
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
                            "key" => Ok(GeneratedField::Key),
                            "keyId" | "key_id" => Ok(GeneratedField::KeyId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.apikeys.v2.CreateKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut key__ = None;
                let mut key_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                        GeneratedField::KeyId => {
                            if key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyId"));
                            }
                            key_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateKeyRequest {
                    parent: parent__.unwrap_or_default(),
                    key: key__,
                    key_id: key_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.apikeys.v2.CreateKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteKeyRequest {
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
        if !self.etag.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.apikeys.v2.DeleteKeyRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.etag.is_empty() {
            struct_ser.serialize_field("etag", &self.etag)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "etag",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Etag,
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
                            "etag" => Ok(GeneratedField::Etag),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.apikeys.v2.DeleteKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut etag__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Etag => {
                            if etag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("etag"));
                            }
                            etag__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteKeyRequest {
                    name: name__.unwrap_or_default(),
                    etag: etag__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.apikeys.v2.DeleteKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetKeyRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.api.apikeys.v2.GetKeyRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetKeyRequest {
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
            type Value = GetKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.apikeys.v2.GetKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetKeyRequest, V::Error>
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
                Ok(GetKeyRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.apikeys.v2.GetKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetKeyStringRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.api.apikeys.v2.GetKeyStringRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetKeyStringRequest {
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
            type Value = GetKeyStringRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.apikeys.v2.GetKeyStringRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetKeyStringRequest, V::Error>
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
                Ok(GetKeyStringRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.apikeys.v2.GetKeyStringRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetKeyStringResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key_string.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.apikeys.v2.GetKeyStringResponse", len)?;
        if !self.key_string.is_empty() {
            struct_ser.serialize_field("keyString", &self.key_string)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetKeyStringResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_string",
            "keyString",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyString,
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
                            "keyString" | "key_string" => Ok(GeneratedField::KeyString),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetKeyStringResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.apikeys.v2.GetKeyStringResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetKeyStringResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_string__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyString => {
                            if key_string__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyString"));
                            }
                            key_string__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetKeyStringResponse {
                    key_string: key_string__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.apikeys.v2.GetKeyStringResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IosKeyRestrictions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.allowed_bundle_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.apikeys.v2.IosKeyRestrictions", len)?;
        if !self.allowed_bundle_ids.is_empty() {
            struct_ser.serialize_field("allowedBundleIds", &self.allowed_bundle_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IosKeyRestrictions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allowed_bundle_ids",
            "allowedBundleIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowedBundleIds,
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
                            "allowedBundleIds" | "allowed_bundle_ids" => Ok(GeneratedField::AllowedBundleIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IosKeyRestrictions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.apikeys.v2.IosKeyRestrictions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IosKeyRestrictions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allowed_bundle_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AllowedBundleIds => {
                            if allowed_bundle_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedBundleIds"));
                            }
                            allowed_bundle_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(IosKeyRestrictions {
                    allowed_bundle_ids: allowed_bundle_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.apikeys.v2.IosKeyRestrictions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Key {
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
        if !self.uid.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.key_string.is_empty() {
            len += 1;
        }
        if self.create_time.is_some() {
            len += 1;
        }
        if self.update_time.is_some() {
            len += 1;
        }
        if self.delete_time.is_some() {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        if self.restrictions.is_some() {
            len += 1;
        }
        if !self.etag.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.apikeys.v2.Key", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.uid.is_empty() {
            struct_ser.serialize_field("uid", &self.uid)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.key_string.is_empty() {
            struct_ser.serialize_field("keyString", &self.key_string)?;
        }
        if let Some(v) = self.create_time.as_ref() {
            struct_ser.serialize_field("createTime", v)?;
        }
        if let Some(v) = self.update_time.as_ref() {
            struct_ser.serialize_field("updateTime", v)?;
        }
        if let Some(v) = self.delete_time.as_ref() {
            struct_ser.serialize_field("deleteTime", v)?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if let Some(v) = self.restrictions.as_ref() {
            struct_ser.serialize_field("restrictions", v)?;
        }
        if !self.etag.is_empty() {
            struct_ser.serialize_field("etag", &self.etag)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Key {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "uid",
            "display_name",
            "displayName",
            "key_string",
            "keyString",
            "create_time",
            "createTime",
            "update_time",
            "updateTime",
            "delete_time",
            "deleteTime",
            "annotations",
            "restrictions",
            "etag",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Uid,
            DisplayName,
            KeyString,
            CreateTime,
            UpdateTime,
            DeleteTime,
            Annotations,
            Restrictions,
            Etag,
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
                            "uid" => Ok(GeneratedField::Uid),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "keyString" | "key_string" => Ok(GeneratedField::KeyString),
                            "createTime" | "create_time" => Ok(GeneratedField::CreateTime),
                            "updateTime" | "update_time" => Ok(GeneratedField::UpdateTime),
                            "deleteTime" | "delete_time" => Ok(GeneratedField::DeleteTime),
                            "annotations" => Ok(GeneratedField::Annotations),
                            "restrictions" => Ok(GeneratedField::Restrictions),
                            "etag" => Ok(GeneratedField::Etag),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Key;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.apikeys.v2.Key")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Key, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut uid__ = None;
                let mut display_name__ = None;
                let mut key_string__ = None;
                let mut create_time__ = None;
                let mut update_time__ = None;
                let mut delete_time__ = None;
                let mut annotations__ = None;
                let mut restrictions__ = None;
                let mut etag__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Uid => {
                            if uid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uid"));
                            }
                            uid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KeyString => {
                            if key_string__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyString"));
                            }
                            key_string__ = Some(map_.next_value()?);
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
                        GeneratedField::DeleteTime => {
                            if delete_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleteTime"));
                            }
                            delete_time__ = map_.next_value()?;
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Restrictions => {
                            if restrictions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restrictions"));
                            }
                            restrictions__ = map_.next_value()?;
                        }
                        GeneratedField::Etag => {
                            if etag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("etag"));
                            }
                            etag__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Key {
                    name: name__.unwrap_or_default(),
                    uid: uid__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    key_string: key_string__.unwrap_or_default(),
                    create_time: create_time__,
                    update_time: update_time__,
                    delete_time: delete_time__,
                    annotations: annotations__.unwrap_or_default(),
                    restrictions: restrictions__,
                    etag: etag__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.apikeys.v2.Key", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListKeysRequest {
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
        if self.show_deleted {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.apikeys.v2.ListKeysRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if self.show_deleted {
            struct_ser.serialize_field("showDeleted", &self.show_deleted)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListKeysRequest {
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
            "show_deleted",
            "showDeleted",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            PageSize,
            PageToken,
            ShowDeleted,
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
                            "showDeleted" | "show_deleted" => Ok(GeneratedField::ShowDeleted),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListKeysRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.apikeys.v2.ListKeysRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListKeysRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut show_deleted__ = None;
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
                        GeneratedField::ShowDeleted => {
                            if show_deleted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("showDeleted"));
                            }
                            show_deleted__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListKeysRequest {
                    parent: parent__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    show_deleted: show_deleted__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.apikeys.v2.ListKeysRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListKeysResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.keys.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.apikeys.v2.ListKeysResponse", len)?;
        if !self.keys.is_empty() {
            struct_ser.serialize_field("keys", &self.keys)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListKeysResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "keys",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Keys,
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
                            "keys" => Ok(GeneratedField::Keys),
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
            type Value = ListKeysResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.apikeys.v2.ListKeysResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListKeysResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut keys__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Keys => {
                            if keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keys"));
                            }
                            keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListKeysResponse {
                    keys: keys__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.apikeys.v2.ListKeysResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LookupKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key_string.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.apikeys.v2.LookupKeyRequest", len)?;
        if !self.key_string.is_empty() {
            struct_ser.serialize_field("keyString", &self.key_string)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LookupKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_string",
            "keyString",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyString,
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
                            "keyString" | "key_string" => Ok(GeneratedField::KeyString),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LookupKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.apikeys.v2.LookupKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LookupKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_string__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyString => {
                            if key_string__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyString"));
                            }
                            key_string__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LookupKeyRequest {
                    key_string: key_string__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.apikeys.v2.LookupKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LookupKeyResponse {
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
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.apikeys.v2.LookupKeyResponse", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LookupKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
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
                            "parent" => Ok(GeneratedField::Parent),
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
            type Value = LookupKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.apikeys.v2.LookupKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LookupKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LookupKeyResponse {
                    parent: parent__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.apikeys.v2.LookupKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Restrictions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.api_targets.is_empty() {
            len += 1;
        }
        if self.client_restrictions.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.apikeys.v2.Restrictions", len)?;
        if !self.api_targets.is_empty() {
            struct_ser.serialize_field("apiTargets", &self.api_targets)?;
        }
        if let Some(v) = self.client_restrictions.as_ref() {
            match v {
                restrictions::ClientRestrictions::BrowserKeyRestrictions(v) => {
                    struct_ser.serialize_field("browserKeyRestrictions", v)?;
                }
                restrictions::ClientRestrictions::ServerKeyRestrictions(v) => {
                    struct_ser.serialize_field("serverKeyRestrictions", v)?;
                }
                restrictions::ClientRestrictions::AndroidKeyRestrictions(v) => {
                    struct_ser.serialize_field("androidKeyRestrictions", v)?;
                }
                restrictions::ClientRestrictions::IosKeyRestrictions(v) => {
                    struct_ser.serialize_field("iosKeyRestrictions", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Restrictions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "api_targets",
            "apiTargets",
            "browser_key_restrictions",
            "browserKeyRestrictions",
            "server_key_restrictions",
            "serverKeyRestrictions",
            "android_key_restrictions",
            "androidKeyRestrictions",
            "ios_key_restrictions",
            "iosKeyRestrictions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ApiTargets,
            BrowserKeyRestrictions,
            ServerKeyRestrictions,
            AndroidKeyRestrictions,
            IosKeyRestrictions,
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
                            "apiTargets" | "api_targets" => Ok(GeneratedField::ApiTargets),
                            "browserKeyRestrictions" | "browser_key_restrictions" => Ok(GeneratedField::BrowserKeyRestrictions),
                            "serverKeyRestrictions" | "server_key_restrictions" => Ok(GeneratedField::ServerKeyRestrictions),
                            "androidKeyRestrictions" | "android_key_restrictions" => Ok(GeneratedField::AndroidKeyRestrictions),
                            "iosKeyRestrictions" | "ios_key_restrictions" => Ok(GeneratedField::IosKeyRestrictions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Restrictions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.apikeys.v2.Restrictions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Restrictions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut api_targets__ = None;
                let mut client_restrictions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ApiTargets => {
                            if api_targets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiTargets"));
                            }
                            api_targets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BrowserKeyRestrictions => {
                            if client_restrictions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("browserKeyRestrictions"));
                            }
                            client_restrictions__ = map_.next_value::<::std::option::Option<_>>()?.map(restrictions::ClientRestrictions::BrowserKeyRestrictions)
;
                        }
                        GeneratedField::ServerKeyRestrictions => {
                            if client_restrictions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverKeyRestrictions"));
                            }
                            client_restrictions__ = map_.next_value::<::std::option::Option<_>>()?.map(restrictions::ClientRestrictions::ServerKeyRestrictions)
;
                        }
                        GeneratedField::AndroidKeyRestrictions => {
                            if client_restrictions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("androidKeyRestrictions"));
                            }
                            client_restrictions__ = map_.next_value::<::std::option::Option<_>>()?.map(restrictions::ClientRestrictions::AndroidKeyRestrictions)
;
                        }
                        GeneratedField::IosKeyRestrictions => {
                            if client_restrictions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iosKeyRestrictions"));
                            }
                            client_restrictions__ = map_.next_value::<::std::option::Option<_>>()?.map(restrictions::ClientRestrictions::IosKeyRestrictions)
;
                        }
                    }
                }
                Ok(Restrictions {
                    api_targets: api_targets__.unwrap_or_default(),
                    client_restrictions: client_restrictions__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.apikeys.v2.Restrictions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServerKeyRestrictions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.allowed_ips.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.apikeys.v2.ServerKeyRestrictions", len)?;
        if !self.allowed_ips.is_empty() {
            struct_ser.serialize_field("allowedIps", &self.allowed_ips)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServerKeyRestrictions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allowed_ips",
            "allowedIps",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowedIps,
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
                            "allowedIps" | "allowed_ips" => Ok(GeneratedField::AllowedIps),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServerKeyRestrictions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.apikeys.v2.ServerKeyRestrictions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ServerKeyRestrictions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allowed_ips__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AllowedIps => {
                            if allowed_ips__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedIps"));
                            }
                            allowed_ips__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ServerKeyRestrictions {
                    allowed_ips: allowed_ips__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.apikeys.v2.ServerKeyRestrictions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UndeleteKeyRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.api.apikeys.v2.UndeleteKeyRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UndeleteKeyRequest {
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
            type Value = UndeleteKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.apikeys.v2.UndeleteKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UndeleteKeyRequest, V::Error>
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
                Ok(UndeleteKeyRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.apikeys.v2.UndeleteKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.apikeys.v2.UpdateKeyRequest", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            UpdateMask,
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
                            "key" => Ok(GeneratedField::Key),
                            "updateMask" | "update_mask" => Ok(GeneratedField::UpdateMask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.apikeys.v2.UpdateKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateKeyRequest {
                    key: key__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.apikeys.v2.UpdateKeyRequest", FIELDS, GeneratedVisitor)
    }
}
