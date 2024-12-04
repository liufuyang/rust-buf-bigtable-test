// @generated
impl serde::Serialize for CheckRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.parsed_expr.is_some() {
            len += 1;
        }
        if !self.type_env.is_empty() {
            len += 1;
        }
        if !self.container.is_empty() {
            len += 1;
        }
        if self.no_std_env {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.conformance.v1alpha1.CheckRequest", len)?;
        if let Some(v) = self.parsed_expr.as_ref() {
            struct_ser.serialize_field("parsedExpr", v)?;
        }
        if !self.type_env.is_empty() {
            struct_ser.serialize_field("typeEnv", &self.type_env)?;
        }
        if !self.container.is_empty() {
            struct_ser.serialize_field("container", &self.container)?;
        }
        if self.no_std_env {
            struct_ser.serialize_field("noStdEnv", &self.no_std_env)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parsed_expr",
            "parsedExpr",
            "type_env",
            "typeEnv",
            "container",
            "no_std_env",
            "noStdEnv",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ParsedExpr,
            TypeEnv,
            Container,
            NoStdEnv,
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
                            "parsedExpr" | "parsed_expr" => Ok(GeneratedField::ParsedExpr),
                            "typeEnv" | "type_env" => Ok(GeneratedField::TypeEnv),
                            "container" => Ok(GeneratedField::Container),
                            "noStdEnv" | "no_std_env" => Ok(GeneratedField::NoStdEnv),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.conformance.v1alpha1.CheckRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parsed_expr__ = None;
                let mut type_env__ = None;
                let mut container__ = None;
                let mut no_std_env__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ParsedExpr => {
                            if parsed_expr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parsedExpr"));
                            }
                            parsed_expr__ = map_.next_value()?;
                        }
                        GeneratedField::TypeEnv => {
                            if type_env__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeEnv"));
                            }
                            type_env__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Container => {
                            if container__.is_some() {
                                return Err(serde::de::Error::duplicate_field("container"));
                            }
                            container__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NoStdEnv => {
                            if no_std_env__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noStdEnv"));
                            }
                            no_std_env__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CheckRequest {
                    parsed_expr: parsed_expr__,
                    type_env: type_env__.unwrap_or_default(),
                    container: container__.unwrap_or_default(),
                    no_std_env: no_std_env__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.conformance.v1alpha1.CheckRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.checked_expr.is_some() {
            len += 1;
        }
        if !self.issues.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.conformance.v1alpha1.CheckResponse", len)?;
        if let Some(v) = self.checked_expr.as_ref() {
            struct_ser.serialize_field("checkedExpr", v)?;
        }
        if !self.issues.is_empty() {
            struct_ser.serialize_field("issues", &self.issues)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "checked_expr",
            "checkedExpr",
            "issues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CheckedExpr,
            Issues,
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
                            "checkedExpr" | "checked_expr" => Ok(GeneratedField::CheckedExpr),
                            "issues" => Ok(GeneratedField::Issues),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.conformance.v1alpha1.CheckResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut checked_expr__ = None;
                let mut issues__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CheckedExpr => {
                            if checked_expr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkedExpr"));
                            }
                            checked_expr__ = map_.next_value()?;
                        }
                        GeneratedField::Issues => {
                            if issues__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issues"));
                            }
                            issues__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CheckResponse {
                    checked_expr: checked_expr__,
                    issues: issues__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.conformance.v1alpha1.CheckResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EvalRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.bindings.is_empty() {
            len += 1;
        }
        if !self.container.is_empty() {
            len += 1;
        }
        if self.expr_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.conformance.v1alpha1.EvalRequest", len)?;
        if !self.bindings.is_empty() {
            struct_ser.serialize_field("bindings", &self.bindings)?;
        }
        if !self.container.is_empty() {
            struct_ser.serialize_field("container", &self.container)?;
        }
        if let Some(v) = self.expr_kind.as_ref() {
            match v {
                eval_request::ExprKind::ParsedExpr(v) => {
                    struct_ser.serialize_field("parsedExpr", v)?;
                }
                eval_request::ExprKind::CheckedExpr(v) => {
                    struct_ser.serialize_field("checkedExpr", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EvalRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bindings",
            "container",
            "parsed_expr",
            "parsedExpr",
            "checked_expr",
            "checkedExpr",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bindings,
            Container,
            ParsedExpr,
            CheckedExpr,
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
                            "bindings" => Ok(GeneratedField::Bindings),
                            "container" => Ok(GeneratedField::Container),
                            "parsedExpr" | "parsed_expr" => Ok(GeneratedField::ParsedExpr),
                            "checkedExpr" | "checked_expr" => Ok(GeneratedField::CheckedExpr),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EvalRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.conformance.v1alpha1.EvalRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EvalRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bindings__ = None;
                let mut container__ = None;
                let mut expr_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Bindings => {
                            if bindings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bindings"));
                            }
                            bindings__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Container => {
                            if container__.is_some() {
                                return Err(serde::de::Error::duplicate_field("container"));
                            }
                            container__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ParsedExpr => {
                            if expr_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parsedExpr"));
                            }
                            expr_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(eval_request::ExprKind::ParsedExpr)
;
                        }
                        GeneratedField::CheckedExpr => {
                            if expr_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkedExpr"));
                            }
                            expr_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(eval_request::ExprKind::CheckedExpr)
;
                        }
                    }
                }
                Ok(EvalRequest {
                    bindings: bindings__.unwrap_or_default(),
                    container: container__.unwrap_or_default(),
                    expr_kind: expr_kind__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.conformance.v1alpha1.EvalRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EvalResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        if !self.issues.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.conformance.v1alpha1.EvalResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        if !self.issues.is_empty() {
            struct_ser.serialize_field("issues", &self.issues)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EvalResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
            "issues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
            Issues,
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
                            "result" => Ok(GeneratedField::Result),
                            "issues" => Ok(GeneratedField::Issues),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EvalResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.conformance.v1alpha1.EvalResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EvalResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                let mut issues__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map_.next_value()?;
                        }
                        GeneratedField::Issues => {
                            if issues__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issues"));
                            }
                            issues__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EvalResponse {
                    result: result__,
                    issues: issues__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.conformance.v1alpha1.EvalResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IssueDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.severity != 0 {
            len += 1;
        }
        if self.position.is_some() {
            len += 1;
        }
        if self.id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.conformance.v1alpha1.IssueDetails", len)?;
        if self.severity != 0 {
            let v = issue_details::Severity::try_from(self.severity)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.severity)))?;
            struct_ser.serialize_field("severity", &v)?;
        }
        if let Some(v) = self.position.as_ref() {
            struct_ser.serialize_field("position", v)?;
        }
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IssueDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "severity",
            "position",
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Severity,
            Position,
            Id,
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
                            "severity" => Ok(GeneratedField::Severity),
                            "position" => Ok(GeneratedField::Position),
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IssueDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.conformance.v1alpha1.IssueDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IssueDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut severity__ = None;
                let mut position__ = None;
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Severity => {
                            if severity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("severity"));
                            }
                            severity__ = Some(map_.next_value::<issue_details::Severity>()? as i32);
                        }
                        GeneratedField::Position => {
                            if position__.is_some() {
                                return Err(serde::de::Error::duplicate_field("position"));
                            }
                            position__ = map_.next_value()?;
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(IssueDetails {
                    severity: severity__.unwrap_or_default(),
                    position: position__,
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.conformance.v1alpha1.IssueDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for issue_details::Severity {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SEVERITY_UNSPECIFIED",
            Self::Deprecation => "DEPRECATION",
            Self::Warning => "WARNING",
            Self::Error => "ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for issue_details::Severity {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SEVERITY_UNSPECIFIED",
            "DEPRECATION",
            "WARNING",
            "ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = issue_details::Severity;

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
                    "SEVERITY_UNSPECIFIED" => Ok(issue_details::Severity::Unspecified),
                    "DEPRECATION" => Ok(issue_details::Severity::Deprecation),
                    "WARNING" => Ok(issue_details::Severity::Warning),
                    "ERROR" => Ok(issue_details::Severity::Error),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ParseRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cel_source.is_empty() {
            len += 1;
        }
        if !self.syntax_version.is_empty() {
            len += 1;
        }
        if !self.source_location.is_empty() {
            len += 1;
        }
        if self.disable_macros {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.conformance.v1alpha1.ParseRequest", len)?;
        if !self.cel_source.is_empty() {
            struct_ser.serialize_field("celSource", &self.cel_source)?;
        }
        if !self.syntax_version.is_empty() {
            struct_ser.serialize_field("syntaxVersion", &self.syntax_version)?;
        }
        if !self.source_location.is_empty() {
            struct_ser.serialize_field("sourceLocation", &self.source_location)?;
        }
        if self.disable_macros {
            struct_ser.serialize_field("disableMacros", &self.disable_macros)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ParseRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cel_source",
            "celSource",
            "syntax_version",
            "syntaxVersion",
            "source_location",
            "sourceLocation",
            "disable_macros",
            "disableMacros",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CelSource,
            SyntaxVersion,
            SourceLocation,
            DisableMacros,
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
                            "celSource" | "cel_source" => Ok(GeneratedField::CelSource),
                            "syntaxVersion" | "syntax_version" => Ok(GeneratedField::SyntaxVersion),
                            "sourceLocation" | "source_location" => Ok(GeneratedField::SourceLocation),
                            "disableMacros" | "disable_macros" => Ok(GeneratedField::DisableMacros),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ParseRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.conformance.v1alpha1.ParseRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ParseRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cel_source__ = None;
                let mut syntax_version__ = None;
                let mut source_location__ = None;
                let mut disable_macros__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CelSource => {
                            if cel_source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("celSource"));
                            }
                            cel_source__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SyntaxVersion => {
                            if syntax_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("syntaxVersion"));
                            }
                            syntax_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourceLocation => {
                            if source_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceLocation"));
                            }
                            source_location__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisableMacros => {
                            if disable_macros__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableMacros"));
                            }
                            disable_macros__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ParseRequest {
                    cel_source: cel_source__.unwrap_or_default(),
                    syntax_version: syntax_version__.unwrap_or_default(),
                    source_location: source_location__.unwrap_or_default(),
                    disable_macros: disable_macros__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.conformance.v1alpha1.ParseRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ParseResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.parsed_expr.is_some() {
            len += 1;
        }
        if !self.issues.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.conformance.v1alpha1.ParseResponse", len)?;
        if let Some(v) = self.parsed_expr.as_ref() {
            struct_ser.serialize_field("parsedExpr", v)?;
        }
        if !self.issues.is_empty() {
            struct_ser.serialize_field("issues", &self.issues)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ParseResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parsed_expr",
            "parsedExpr",
            "issues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ParsedExpr,
            Issues,
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
                            "parsedExpr" | "parsed_expr" => Ok(GeneratedField::ParsedExpr),
                            "issues" => Ok(GeneratedField::Issues),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ParseResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.conformance.v1alpha1.ParseResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ParseResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parsed_expr__ = None;
                let mut issues__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ParsedExpr => {
                            if parsed_expr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parsedExpr"));
                            }
                            parsed_expr__ = map_.next_value()?;
                        }
                        GeneratedField::Issues => {
                            if issues__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issues"));
                            }
                            issues__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ParseResponse {
                    parsed_expr: parsed_expr__,
                    issues: issues__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.conformance.v1alpha1.ParseResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SourcePosition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.location.is_empty() {
            len += 1;
        }
        if self.offset != 0 {
            len += 1;
        }
        if self.line != 0 {
            len += 1;
        }
        if self.column != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.conformance.v1alpha1.SourcePosition", len)?;
        if !self.location.is_empty() {
            struct_ser.serialize_field("location", &self.location)?;
        }
        if self.offset != 0 {
            struct_ser.serialize_field("offset", &self.offset)?;
        }
        if self.line != 0 {
            struct_ser.serialize_field("line", &self.line)?;
        }
        if self.column != 0 {
            struct_ser.serialize_field("column", &self.column)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SourcePosition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "location",
            "offset",
            "line",
            "column",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Location,
            Offset,
            Line,
            Column,
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
                            "location" => Ok(GeneratedField::Location),
                            "offset" => Ok(GeneratedField::Offset),
                            "line" => Ok(GeneratedField::Line),
                            "column" => Ok(GeneratedField::Column),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SourcePosition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.conformance.v1alpha1.SourcePosition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SourcePosition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut location__ = None;
                let mut offset__ = None;
                let mut line__ = None;
                let mut column__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Location => {
                            if location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            location__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Offset => {
                            if offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            offset__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Line => {
                            if line__.is_some() {
                                return Err(serde::de::Error::duplicate_field("line"));
                            }
                            line__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Column => {
                            if column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("column"));
                            }
                            column__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SourcePosition {
                    location: location__.unwrap_or_default(),
                    offset: offset__.unwrap_or_default(),
                    line: line__.unwrap_or_default(),
                    column: column__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.conformance.v1alpha1.SourcePosition", FIELDS, GeneratedVisitor)
    }
}
