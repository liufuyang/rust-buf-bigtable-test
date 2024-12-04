// @generated
impl serde::Serialize for AllocateQuotaRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.service_name.is_empty() {
            len += 1;
        }
        if self.allocate_operation.is_some() {
            len += 1;
        }
        if !self.service_config_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.AllocateQuotaRequest", len)?;
        if !self.service_name.is_empty() {
            struct_ser.serialize_field("serviceName", &self.service_name)?;
        }
        if let Some(v) = self.allocate_operation.as_ref() {
            struct_ser.serialize_field("allocateOperation", v)?;
        }
        if !self.service_config_id.is_empty() {
            struct_ser.serialize_field("serviceConfigId", &self.service_config_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AllocateQuotaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service_name",
            "serviceName",
            "allocate_operation",
            "allocateOperation",
            "service_config_id",
            "serviceConfigId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServiceName,
            AllocateOperation,
            ServiceConfigId,
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
                            "serviceName" | "service_name" => Ok(GeneratedField::ServiceName),
                            "allocateOperation" | "allocate_operation" => Ok(GeneratedField::AllocateOperation),
                            "serviceConfigId" | "service_config_id" => Ok(GeneratedField::ServiceConfigId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AllocateQuotaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.AllocateQuotaRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AllocateQuotaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service_name__ = None;
                let mut allocate_operation__ = None;
                let mut service_config_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ServiceName => {
                            if service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceName"));
                            }
                            service_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllocateOperation => {
                            if allocate_operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allocateOperation"));
                            }
                            allocate_operation__ = map_.next_value()?;
                        }
                        GeneratedField::ServiceConfigId => {
                            if service_config_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceConfigId"));
                            }
                            service_config_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AllocateQuotaRequest {
                    service_name: service_name__.unwrap_or_default(),
                    allocate_operation: allocate_operation__,
                    service_config_id: service_config_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.AllocateQuotaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AllocateQuotaResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.operation_id.is_empty() {
            len += 1;
        }
        if !self.allocate_errors.is_empty() {
            len += 1;
        }
        if !self.quota_metrics.is_empty() {
            len += 1;
        }
        if !self.service_config_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.AllocateQuotaResponse", len)?;
        if !self.operation_id.is_empty() {
            struct_ser.serialize_field("operationId", &self.operation_id)?;
        }
        if !self.allocate_errors.is_empty() {
            struct_ser.serialize_field("allocateErrors", &self.allocate_errors)?;
        }
        if !self.quota_metrics.is_empty() {
            struct_ser.serialize_field("quotaMetrics", &self.quota_metrics)?;
        }
        if !self.service_config_id.is_empty() {
            struct_ser.serialize_field("serviceConfigId", &self.service_config_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AllocateQuotaResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operation_id",
            "operationId",
            "allocate_errors",
            "allocateErrors",
            "quota_metrics",
            "quotaMetrics",
            "service_config_id",
            "serviceConfigId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OperationId,
            AllocateErrors,
            QuotaMetrics,
            ServiceConfigId,
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
                            "operationId" | "operation_id" => Ok(GeneratedField::OperationId),
                            "allocateErrors" | "allocate_errors" => Ok(GeneratedField::AllocateErrors),
                            "quotaMetrics" | "quota_metrics" => Ok(GeneratedField::QuotaMetrics),
                            "serviceConfigId" | "service_config_id" => Ok(GeneratedField::ServiceConfigId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AllocateQuotaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.AllocateQuotaResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AllocateQuotaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operation_id__ = None;
                let mut allocate_errors__ = None;
                let mut quota_metrics__ = None;
                let mut service_config_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OperationId => {
                            if operation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operationId"));
                            }
                            operation_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllocateErrors => {
                            if allocate_errors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allocateErrors"));
                            }
                            allocate_errors__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuotaMetrics => {
                            if quota_metrics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotaMetrics"));
                            }
                            quota_metrics__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServiceConfigId => {
                            if service_config_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceConfigId"));
                            }
                            service_config_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AllocateQuotaResponse {
                    operation_id: operation_id__.unwrap_or_default(),
                    allocate_errors: allocate_errors__.unwrap_or_default(),
                    quota_metrics: quota_metrics__.unwrap_or_default(),
                    service_config_id: service_config_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.AllocateQuotaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if !self.subject.is_empty() {
            len += 1;
        }
        if !self.detail.is_empty() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.CheckError", len)?;
        if self.code != 0 {
            let v = check_error::Code::try_from(self.code)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.code)))?;
            struct_ser.serialize_field("code", &v)?;
        }
        if !self.subject.is_empty() {
            struct_ser.serialize_field("subject", &self.subject)?;
        }
        if !self.detail.is_empty() {
            struct_ser.serialize_field("detail", &self.detail)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "subject",
            "detail",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Subject,
            Detail,
            Status,
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
                            "code" => Ok(GeneratedField::Code),
                            "subject" => Ok(GeneratedField::Subject),
                            "detail" => Ok(GeneratedField::Detail),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.CheckError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut subject__ = None;
                let mut detail__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value::<check_error::Code>()? as i32);
                        }
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Detail => {
                            if detail__.is_some() {
                                return Err(serde::de::Error::duplicate_field("detail"));
                            }
                            detail__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CheckError {
                    code: code__.unwrap_or_default(),
                    subject: subject__.unwrap_or_default(),
                    detail: detail__.unwrap_or_default(),
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.CheckError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for check_error::Code {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::ErrorCodeUnspecified => "ERROR_CODE_UNSPECIFIED",
            Self::NotFound => "NOT_FOUND",
            Self::PermissionDenied => "PERMISSION_DENIED",
            Self::ResourceExhausted => "RESOURCE_EXHAUSTED",
            Self::ServiceNotActivated => "SERVICE_NOT_ACTIVATED",
            Self::BillingDisabled => "BILLING_DISABLED",
            Self::ProjectDeleted => "PROJECT_DELETED",
            Self::ProjectInvalid => "PROJECT_INVALID",
            Self::ConsumerInvalid => "CONSUMER_INVALID",
            Self::IpAddressBlocked => "IP_ADDRESS_BLOCKED",
            Self::RefererBlocked => "REFERER_BLOCKED",
            Self::ClientAppBlocked => "CLIENT_APP_BLOCKED",
            Self::ApiTargetBlocked => "API_TARGET_BLOCKED",
            Self::ApiKeyInvalid => "API_KEY_INVALID",
            Self::ApiKeyExpired => "API_KEY_EXPIRED",
            Self::ApiKeyNotFound => "API_KEY_NOT_FOUND",
            Self::InvalidCredential => "INVALID_CREDENTIAL",
            Self::NamespaceLookupUnavailable => "NAMESPACE_LOOKUP_UNAVAILABLE",
            Self::ServiceStatusUnavailable => "SERVICE_STATUS_UNAVAILABLE",
            Self::BillingStatusUnavailable => "BILLING_STATUS_UNAVAILABLE",
            Self::CloudResourceManagerBackendUnavailable => "CLOUD_RESOURCE_MANAGER_BACKEND_UNAVAILABLE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for check_error::Code {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ERROR_CODE_UNSPECIFIED",
            "NOT_FOUND",
            "PERMISSION_DENIED",
            "RESOURCE_EXHAUSTED",
            "SERVICE_NOT_ACTIVATED",
            "BILLING_DISABLED",
            "PROJECT_DELETED",
            "PROJECT_INVALID",
            "CONSUMER_INVALID",
            "IP_ADDRESS_BLOCKED",
            "REFERER_BLOCKED",
            "CLIENT_APP_BLOCKED",
            "API_TARGET_BLOCKED",
            "API_KEY_INVALID",
            "API_KEY_EXPIRED",
            "API_KEY_NOT_FOUND",
            "INVALID_CREDENTIAL",
            "NAMESPACE_LOOKUP_UNAVAILABLE",
            "SERVICE_STATUS_UNAVAILABLE",
            "BILLING_STATUS_UNAVAILABLE",
            "CLOUD_RESOURCE_MANAGER_BACKEND_UNAVAILABLE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = check_error::Code;

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
                    "ERROR_CODE_UNSPECIFIED" => Ok(check_error::Code::ErrorCodeUnspecified),
                    "NOT_FOUND" => Ok(check_error::Code::NotFound),
                    "PERMISSION_DENIED" => Ok(check_error::Code::PermissionDenied),
                    "RESOURCE_EXHAUSTED" => Ok(check_error::Code::ResourceExhausted),
                    "SERVICE_NOT_ACTIVATED" => Ok(check_error::Code::ServiceNotActivated),
                    "BILLING_DISABLED" => Ok(check_error::Code::BillingDisabled),
                    "PROJECT_DELETED" => Ok(check_error::Code::ProjectDeleted),
                    "PROJECT_INVALID" => Ok(check_error::Code::ProjectInvalid),
                    "CONSUMER_INVALID" => Ok(check_error::Code::ConsumerInvalid),
                    "IP_ADDRESS_BLOCKED" => Ok(check_error::Code::IpAddressBlocked),
                    "REFERER_BLOCKED" => Ok(check_error::Code::RefererBlocked),
                    "CLIENT_APP_BLOCKED" => Ok(check_error::Code::ClientAppBlocked),
                    "API_TARGET_BLOCKED" => Ok(check_error::Code::ApiTargetBlocked),
                    "API_KEY_INVALID" => Ok(check_error::Code::ApiKeyInvalid),
                    "API_KEY_EXPIRED" => Ok(check_error::Code::ApiKeyExpired),
                    "API_KEY_NOT_FOUND" => Ok(check_error::Code::ApiKeyNotFound),
                    "INVALID_CREDENTIAL" => Ok(check_error::Code::InvalidCredential),
                    "NAMESPACE_LOOKUP_UNAVAILABLE" => Ok(check_error::Code::NamespaceLookupUnavailable),
                    "SERVICE_STATUS_UNAVAILABLE" => Ok(check_error::Code::ServiceStatusUnavailable),
                    "BILLING_STATUS_UNAVAILABLE" => Ok(check_error::Code::BillingStatusUnavailable),
                    "CLOUD_RESOURCE_MANAGER_BACKEND_UNAVAILABLE" => Ok(check_error::Code::CloudResourceManagerBackendUnavailable),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CheckRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.service_name.is_empty() {
            len += 1;
        }
        if self.operation.is_some() {
            len += 1;
        }
        if !self.service_config_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.CheckRequest", len)?;
        if !self.service_name.is_empty() {
            struct_ser.serialize_field("serviceName", &self.service_name)?;
        }
        if let Some(v) = self.operation.as_ref() {
            struct_ser.serialize_field("operation", v)?;
        }
        if !self.service_config_id.is_empty() {
            struct_ser.serialize_field("serviceConfigId", &self.service_config_id)?;
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
            "service_name",
            "serviceName",
            "operation",
            "service_config_id",
            "serviceConfigId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServiceName,
            Operation,
            ServiceConfigId,
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
                            "serviceName" | "service_name" => Ok(GeneratedField::ServiceName),
                            "operation" => Ok(GeneratedField::Operation),
                            "serviceConfigId" | "service_config_id" => Ok(GeneratedField::ServiceConfigId),
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
                formatter.write_str("struct google.api.servicecontrol.v1.CheckRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service_name__ = None;
                let mut operation__ = None;
                let mut service_config_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ServiceName => {
                            if service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceName"));
                            }
                            service_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Operation => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation__ = map_.next_value()?;
                        }
                        GeneratedField::ServiceConfigId => {
                            if service_config_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceConfigId"));
                            }
                            service_config_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CheckRequest {
                    service_name: service_name__.unwrap_or_default(),
                    operation: operation__,
                    service_config_id: service_config_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.CheckRequest", FIELDS, GeneratedVisitor)
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
        if !self.operation_id.is_empty() {
            len += 1;
        }
        if !self.check_errors.is_empty() {
            len += 1;
        }
        if !self.service_config_id.is_empty() {
            len += 1;
        }
        if !self.service_rollout_id.is_empty() {
            len += 1;
        }
        if self.check_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.CheckResponse", len)?;
        if !self.operation_id.is_empty() {
            struct_ser.serialize_field("operationId", &self.operation_id)?;
        }
        if !self.check_errors.is_empty() {
            struct_ser.serialize_field("checkErrors", &self.check_errors)?;
        }
        if !self.service_config_id.is_empty() {
            struct_ser.serialize_field("serviceConfigId", &self.service_config_id)?;
        }
        if !self.service_rollout_id.is_empty() {
            struct_ser.serialize_field("serviceRolloutId", &self.service_rollout_id)?;
        }
        if let Some(v) = self.check_info.as_ref() {
            struct_ser.serialize_field("checkInfo", v)?;
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
            "operation_id",
            "operationId",
            "check_errors",
            "checkErrors",
            "service_config_id",
            "serviceConfigId",
            "service_rollout_id",
            "serviceRolloutId",
            "check_info",
            "checkInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OperationId,
            CheckErrors,
            ServiceConfigId,
            ServiceRolloutId,
            CheckInfo,
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
                            "operationId" | "operation_id" => Ok(GeneratedField::OperationId),
                            "checkErrors" | "check_errors" => Ok(GeneratedField::CheckErrors),
                            "serviceConfigId" | "service_config_id" => Ok(GeneratedField::ServiceConfigId),
                            "serviceRolloutId" | "service_rollout_id" => Ok(GeneratedField::ServiceRolloutId),
                            "checkInfo" | "check_info" => Ok(GeneratedField::CheckInfo),
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
                formatter.write_str("struct google.api.servicecontrol.v1.CheckResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operation_id__ = None;
                let mut check_errors__ = None;
                let mut service_config_id__ = None;
                let mut service_rollout_id__ = None;
                let mut check_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OperationId => {
                            if operation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operationId"));
                            }
                            operation_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CheckErrors => {
                            if check_errors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkErrors"));
                            }
                            check_errors__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServiceConfigId => {
                            if service_config_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceConfigId"));
                            }
                            service_config_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServiceRolloutId => {
                            if service_rollout_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceRolloutId"));
                            }
                            service_rollout_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CheckInfo => {
                            if check_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkInfo"));
                            }
                            check_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CheckResponse {
                    operation_id: operation_id__.unwrap_or_default(),
                    check_errors: check_errors__.unwrap_or_default(),
                    service_config_id: service_config_id__.unwrap_or_default(),
                    service_rollout_id: service_rollout_id__.unwrap_or_default(),
                    check_info: check_info__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.CheckResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for check_response::CheckInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.unused_arguments.is_empty() {
            len += 1;
        }
        if self.consumer_info.is_some() {
            len += 1;
        }
        if !self.api_key_uid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.CheckResponse.CheckInfo", len)?;
        if !self.unused_arguments.is_empty() {
            struct_ser.serialize_field("unusedArguments", &self.unused_arguments)?;
        }
        if let Some(v) = self.consumer_info.as_ref() {
            struct_ser.serialize_field("consumerInfo", v)?;
        }
        if !self.api_key_uid.is_empty() {
            struct_ser.serialize_field("apiKeyUid", &self.api_key_uid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for check_response::CheckInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "unused_arguments",
            "unusedArguments",
            "consumer_info",
            "consumerInfo",
            "api_key_uid",
            "apiKeyUid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UnusedArguments,
            ConsumerInfo,
            ApiKeyUid,
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
                            "unusedArguments" | "unused_arguments" => Ok(GeneratedField::UnusedArguments),
                            "consumerInfo" | "consumer_info" => Ok(GeneratedField::ConsumerInfo),
                            "apiKeyUid" | "api_key_uid" => Ok(GeneratedField::ApiKeyUid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = check_response::CheckInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.CheckResponse.CheckInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<check_response::CheckInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut unused_arguments__ = None;
                let mut consumer_info__ = None;
                let mut api_key_uid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UnusedArguments => {
                            if unused_arguments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unusedArguments"));
                            }
                            unused_arguments__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConsumerInfo => {
                            if consumer_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consumerInfo"));
                            }
                            consumer_info__ = map_.next_value()?;
                        }
                        GeneratedField::ApiKeyUid => {
                            if api_key_uid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiKeyUid"));
                            }
                            api_key_uid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(check_response::CheckInfo {
                    unused_arguments: unused_arguments__.unwrap_or_default(),
                    consumer_info: consumer_info__,
                    api_key_uid: api_key_uid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.CheckResponse.CheckInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for check_response::ConsumerInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.project_number != 0 {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if self.consumer_number != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.CheckResponse.ConsumerInfo", len)?;
        if self.project_number != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("projectNumber", ToString::to_string(&self.project_number).as_str())?;
        }
        if self.r#type != 0 {
            let v = check_response::consumer_info::ConsumerType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if self.consumer_number != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("consumerNumber", ToString::to_string(&self.consumer_number).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for check_response::ConsumerInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_number",
            "projectNumber",
            "type",
            "consumer_number",
            "consumerNumber",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectNumber,
            Type,
            ConsumerNumber,
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
                            "projectNumber" | "project_number" => Ok(GeneratedField::ProjectNumber),
                            "type" => Ok(GeneratedField::Type),
                            "consumerNumber" | "consumer_number" => Ok(GeneratedField::ConsumerNumber),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = check_response::ConsumerInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.CheckResponse.ConsumerInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<check_response::ConsumerInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_number__ = None;
                let mut r#type__ = None;
                let mut consumer_number__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectNumber => {
                            if project_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectNumber"));
                            }
                            project_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<check_response::consumer_info::ConsumerType>()? as i32);
                        }
                        GeneratedField::ConsumerNumber => {
                            if consumer_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consumerNumber"));
                            }
                            consumer_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(check_response::ConsumerInfo {
                    project_number: project_number__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    consumer_number: consumer_number__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.CheckResponse.ConsumerInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for check_response::consumer_info::ConsumerType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CONSUMER_TYPE_UNSPECIFIED",
            Self::Project => "PROJECT",
            Self::Folder => "FOLDER",
            Self::Organization => "ORGANIZATION",
            Self::ServiceSpecific => "SERVICE_SPECIFIC",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for check_response::consumer_info::ConsumerType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CONSUMER_TYPE_UNSPECIFIED",
            "PROJECT",
            "FOLDER",
            "ORGANIZATION",
            "SERVICE_SPECIFIC",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = check_response::consumer_info::ConsumerType;

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
                    "CONSUMER_TYPE_UNSPECIFIED" => Ok(check_response::consumer_info::ConsumerType::Unspecified),
                    "PROJECT" => Ok(check_response::consumer_info::ConsumerType::Project),
                    "FOLDER" => Ok(check_response::consumer_info::ConsumerType::Folder),
                    "ORGANIZATION" => Ok(check_response::consumer_info::ConsumerType::Organization),
                    "SERVICE_SPECIFIC" => Ok(check_response::consumer_info::ConsumerType::ServiceSpecific),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Distribution {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.count != 0 {
            len += 1;
        }
        if self.mean != 0. {
            len += 1;
        }
        if self.minimum != 0. {
            len += 1;
        }
        if self.maximum != 0. {
            len += 1;
        }
        if self.sum_of_squared_deviation != 0. {
            len += 1;
        }
        if !self.bucket_counts.is_empty() {
            len += 1;
        }
        if !self.exemplars.is_empty() {
            len += 1;
        }
        if self.bucket_option.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.Distribution", len)?;
        if self.count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("count", ToString::to_string(&self.count).as_str())?;
        }
        if self.mean != 0. {
            struct_ser.serialize_field("mean", &self.mean)?;
        }
        if self.minimum != 0. {
            struct_ser.serialize_field("minimum", &self.minimum)?;
        }
        if self.maximum != 0. {
            struct_ser.serialize_field("maximum", &self.maximum)?;
        }
        if self.sum_of_squared_deviation != 0. {
            struct_ser.serialize_field("sumOfSquaredDeviation", &self.sum_of_squared_deviation)?;
        }
        if !self.bucket_counts.is_empty() {
            struct_ser.serialize_field("bucketCounts", &self.bucket_counts.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.exemplars.is_empty() {
            struct_ser.serialize_field("exemplars", &self.exemplars)?;
        }
        if let Some(v) = self.bucket_option.as_ref() {
            match v {
                distribution::BucketOption::LinearBuckets(v) => {
                    struct_ser.serialize_field("linearBuckets", v)?;
                }
                distribution::BucketOption::ExponentialBuckets(v) => {
                    struct_ser.serialize_field("exponentialBuckets", v)?;
                }
                distribution::BucketOption::ExplicitBuckets(v) => {
                    struct_ser.serialize_field("explicitBuckets", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Distribution {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "count",
            "mean",
            "minimum",
            "maximum",
            "sum_of_squared_deviation",
            "sumOfSquaredDeviation",
            "bucket_counts",
            "bucketCounts",
            "exemplars",
            "linear_buckets",
            "linearBuckets",
            "exponential_buckets",
            "exponentialBuckets",
            "explicit_buckets",
            "explicitBuckets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Count,
            Mean,
            Minimum,
            Maximum,
            SumOfSquaredDeviation,
            BucketCounts,
            Exemplars,
            LinearBuckets,
            ExponentialBuckets,
            ExplicitBuckets,
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
                            "count" => Ok(GeneratedField::Count),
                            "mean" => Ok(GeneratedField::Mean),
                            "minimum" => Ok(GeneratedField::Minimum),
                            "maximum" => Ok(GeneratedField::Maximum),
                            "sumOfSquaredDeviation" | "sum_of_squared_deviation" => Ok(GeneratedField::SumOfSquaredDeviation),
                            "bucketCounts" | "bucket_counts" => Ok(GeneratedField::BucketCounts),
                            "exemplars" => Ok(GeneratedField::Exemplars),
                            "linearBuckets" | "linear_buckets" => Ok(GeneratedField::LinearBuckets),
                            "exponentialBuckets" | "exponential_buckets" => Ok(GeneratedField::ExponentialBuckets),
                            "explicitBuckets" | "explicit_buckets" => Ok(GeneratedField::ExplicitBuckets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Distribution;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.Distribution")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Distribution, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut count__ = None;
                let mut mean__ = None;
                let mut minimum__ = None;
                let mut maximum__ = None;
                let mut sum_of_squared_deviation__ = None;
                let mut bucket_counts__ = None;
                let mut exemplars__ = None;
                let mut bucket_option__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Count => {
                            if count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Mean => {
                            if mean__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mean"));
                            }
                            mean__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Minimum => {
                            if minimum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minimum"));
                            }
                            minimum__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Maximum => {
                            if maximum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maximum"));
                            }
                            maximum__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SumOfSquaredDeviation => {
                            if sum_of_squared_deviation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sumOfSquaredDeviation"));
                            }
                            sum_of_squared_deviation__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BucketCounts => {
                            if bucket_counts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucketCounts"));
                            }
                            bucket_counts__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Exemplars => {
                            if exemplars__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exemplars"));
                            }
                            exemplars__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LinearBuckets => {
                            if bucket_option__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linearBuckets"));
                            }
                            bucket_option__ = map_.next_value::<::std::option::Option<_>>()?.map(distribution::BucketOption::LinearBuckets)
;
                        }
                        GeneratedField::ExponentialBuckets => {
                            if bucket_option__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exponentialBuckets"));
                            }
                            bucket_option__ = map_.next_value::<::std::option::Option<_>>()?.map(distribution::BucketOption::ExponentialBuckets)
;
                        }
                        GeneratedField::ExplicitBuckets => {
                            if bucket_option__.is_some() {
                                return Err(serde::de::Error::duplicate_field("explicitBuckets"));
                            }
                            bucket_option__ = map_.next_value::<::std::option::Option<_>>()?.map(distribution::BucketOption::ExplicitBuckets)
;
                        }
                    }
                }
                Ok(Distribution {
                    count: count__.unwrap_or_default(),
                    mean: mean__.unwrap_or_default(),
                    minimum: minimum__.unwrap_or_default(),
                    maximum: maximum__.unwrap_or_default(),
                    sum_of_squared_deviation: sum_of_squared_deviation__.unwrap_or_default(),
                    bucket_counts: bucket_counts__.unwrap_or_default(),
                    exemplars: exemplars__.unwrap_or_default(),
                    bucket_option: bucket_option__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.Distribution", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for distribution::ExplicitBuckets {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.bounds.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.Distribution.ExplicitBuckets", len)?;
        if !self.bounds.is_empty() {
            struct_ser.serialize_field("bounds", &self.bounds)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for distribution::ExplicitBuckets {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bounds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bounds,
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
                            "bounds" => Ok(GeneratedField::Bounds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = distribution::ExplicitBuckets;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.Distribution.ExplicitBuckets")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<distribution::ExplicitBuckets, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bounds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Bounds => {
                            if bounds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bounds"));
                            }
                            bounds__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(distribution::ExplicitBuckets {
                    bounds: bounds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.Distribution.ExplicitBuckets", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for distribution::ExponentialBuckets {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.num_finite_buckets != 0 {
            len += 1;
        }
        if self.growth_factor != 0. {
            len += 1;
        }
        if self.scale != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.Distribution.ExponentialBuckets", len)?;
        if self.num_finite_buckets != 0 {
            struct_ser.serialize_field("numFiniteBuckets", &self.num_finite_buckets)?;
        }
        if self.growth_factor != 0. {
            struct_ser.serialize_field("growthFactor", &self.growth_factor)?;
        }
        if self.scale != 0. {
            struct_ser.serialize_field("scale", &self.scale)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for distribution::ExponentialBuckets {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "num_finite_buckets",
            "numFiniteBuckets",
            "growth_factor",
            "growthFactor",
            "scale",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NumFiniteBuckets,
            GrowthFactor,
            Scale,
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
                            "numFiniteBuckets" | "num_finite_buckets" => Ok(GeneratedField::NumFiniteBuckets),
                            "growthFactor" | "growth_factor" => Ok(GeneratedField::GrowthFactor),
                            "scale" => Ok(GeneratedField::Scale),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = distribution::ExponentialBuckets;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.Distribution.ExponentialBuckets")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<distribution::ExponentialBuckets, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut num_finite_buckets__ = None;
                let mut growth_factor__ = None;
                let mut scale__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NumFiniteBuckets => {
                            if num_finite_buckets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numFiniteBuckets"));
                            }
                            num_finite_buckets__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GrowthFactor => {
                            if growth_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("growthFactor"));
                            }
                            growth_factor__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Scale => {
                            if scale__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scale"));
                            }
                            scale__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(distribution::ExponentialBuckets {
                    num_finite_buckets: num_finite_buckets__.unwrap_or_default(),
                    growth_factor: growth_factor__.unwrap_or_default(),
                    scale: scale__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.Distribution.ExponentialBuckets", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for distribution::LinearBuckets {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.num_finite_buckets != 0 {
            len += 1;
        }
        if self.width != 0. {
            len += 1;
        }
        if self.offset != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.Distribution.LinearBuckets", len)?;
        if self.num_finite_buckets != 0 {
            struct_ser.serialize_field("numFiniteBuckets", &self.num_finite_buckets)?;
        }
        if self.width != 0. {
            struct_ser.serialize_field("width", &self.width)?;
        }
        if self.offset != 0. {
            struct_ser.serialize_field("offset", &self.offset)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for distribution::LinearBuckets {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "num_finite_buckets",
            "numFiniteBuckets",
            "width",
            "offset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NumFiniteBuckets,
            Width,
            Offset,
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
                            "numFiniteBuckets" | "num_finite_buckets" => Ok(GeneratedField::NumFiniteBuckets),
                            "width" => Ok(GeneratedField::Width),
                            "offset" => Ok(GeneratedField::Offset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = distribution::LinearBuckets;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.Distribution.LinearBuckets")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<distribution::LinearBuckets, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut num_finite_buckets__ = None;
                let mut width__ = None;
                let mut offset__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NumFiniteBuckets => {
                            if num_finite_buckets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numFiniteBuckets"));
                            }
                            num_finite_buckets__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Width => {
                            if width__.is_some() {
                                return Err(serde::de::Error::duplicate_field("width"));
                            }
                            width__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Offset => {
                            if offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            offset__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(distribution::LinearBuckets {
                    num_finite_buckets: num_finite_buckets__.unwrap_or_default(),
                    width: width__.unwrap_or_default(),
                    offset: offset__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.Distribution.LinearBuckets", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request_method.is_empty() {
            len += 1;
        }
        if !self.request_url.is_empty() {
            len += 1;
        }
        if self.request_size != 0 {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.response_size != 0 {
            len += 1;
        }
        if !self.user_agent.is_empty() {
            len += 1;
        }
        if !self.remote_ip.is_empty() {
            len += 1;
        }
        if !self.server_ip.is_empty() {
            len += 1;
        }
        if !self.referer.is_empty() {
            len += 1;
        }
        if self.latency.is_some() {
            len += 1;
        }
        if self.cache_lookup {
            len += 1;
        }
        if self.cache_hit {
            len += 1;
        }
        if self.cache_validated_with_origin_server {
            len += 1;
        }
        if self.cache_fill_bytes != 0 {
            len += 1;
        }
        if !self.protocol.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.HttpRequest", len)?;
        if !self.request_method.is_empty() {
            struct_ser.serialize_field("requestMethod", &self.request_method)?;
        }
        if !self.request_url.is_empty() {
            struct_ser.serialize_field("requestUrl", &self.request_url)?;
        }
        if self.request_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("requestSize", ToString::to_string(&self.request_size).as_str())?;
        }
        if self.status != 0 {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if self.response_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("responseSize", ToString::to_string(&self.response_size).as_str())?;
        }
        if !self.user_agent.is_empty() {
            struct_ser.serialize_field("userAgent", &self.user_agent)?;
        }
        if !self.remote_ip.is_empty() {
            struct_ser.serialize_field("remoteIp", &self.remote_ip)?;
        }
        if !self.server_ip.is_empty() {
            struct_ser.serialize_field("serverIp", &self.server_ip)?;
        }
        if !self.referer.is_empty() {
            struct_ser.serialize_field("referer", &self.referer)?;
        }
        if let Some(v) = self.latency.as_ref() {
            struct_ser.serialize_field("latency", v)?;
        }
        if self.cache_lookup {
            struct_ser.serialize_field("cacheLookup", &self.cache_lookup)?;
        }
        if self.cache_hit {
            struct_ser.serialize_field("cacheHit", &self.cache_hit)?;
        }
        if self.cache_validated_with_origin_server {
            struct_ser.serialize_field("cacheValidatedWithOriginServer", &self.cache_validated_with_origin_server)?;
        }
        if self.cache_fill_bytes != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("cacheFillBytes", ToString::to_string(&self.cache_fill_bytes).as_str())?;
        }
        if !self.protocol.is_empty() {
            struct_ser.serialize_field("protocol", &self.protocol)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_method",
            "requestMethod",
            "request_url",
            "requestUrl",
            "request_size",
            "requestSize",
            "status",
            "response_size",
            "responseSize",
            "user_agent",
            "userAgent",
            "remote_ip",
            "remoteIp",
            "server_ip",
            "serverIp",
            "referer",
            "latency",
            "cache_lookup",
            "cacheLookup",
            "cache_hit",
            "cacheHit",
            "cache_validated_with_origin_server",
            "cacheValidatedWithOriginServer",
            "cache_fill_bytes",
            "cacheFillBytes",
            "protocol",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestMethod,
            RequestUrl,
            RequestSize,
            Status,
            ResponseSize,
            UserAgent,
            RemoteIp,
            ServerIp,
            Referer,
            Latency,
            CacheLookup,
            CacheHit,
            CacheValidatedWithOriginServer,
            CacheFillBytes,
            Protocol,
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
                            "requestMethod" | "request_method" => Ok(GeneratedField::RequestMethod),
                            "requestUrl" | "request_url" => Ok(GeneratedField::RequestUrl),
                            "requestSize" | "request_size" => Ok(GeneratedField::RequestSize),
                            "status" => Ok(GeneratedField::Status),
                            "responseSize" | "response_size" => Ok(GeneratedField::ResponseSize),
                            "userAgent" | "user_agent" => Ok(GeneratedField::UserAgent),
                            "remoteIp" | "remote_ip" => Ok(GeneratedField::RemoteIp),
                            "serverIp" | "server_ip" => Ok(GeneratedField::ServerIp),
                            "referer" => Ok(GeneratedField::Referer),
                            "latency" => Ok(GeneratedField::Latency),
                            "cacheLookup" | "cache_lookup" => Ok(GeneratedField::CacheLookup),
                            "cacheHit" | "cache_hit" => Ok(GeneratedField::CacheHit),
                            "cacheValidatedWithOriginServer" | "cache_validated_with_origin_server" => Ok(GeneratedField::CacheValidatedWithOriginServer),
                            "cacheFillBytes" | "cache_fill_bytes" => Ok(GeneratedField::CacheFillBytes),
                            "protocol" => Ok(GeneratedField::Protocol),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.HttpRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_method__ = None;
                let mut request_url__ = None;
                let mut request_size__ = None;
                let mut status__ = None;
                let mut response_size__ = None;
                let mut user_agent__ = None;
                let mut remote_ip__ = None;
                let mut server_ip__ = None;
                let mut referer__ = None;
                let mut latency__ = None;
                let mut cache_lookup__ = None;
                let mut cache_hit__ = None;
                let mut cache_validated_with_origin_server__ = None;
                let mut cache_fill_bytes__ = None;
                let mut protocol__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RequestMethod => {
                            if request_method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestMethod"));
                            }
                            request_method__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestUrl => {
                            if request_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestUrl"));
                            }
                            request_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestSize => {
                            if request_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestSize"));
                            }
                            request_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResponseSize => {
                            if response_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseSize"));
                            }
                            response_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UserAgent => {
                            if user_agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAgent"));
                            }
                            user_agent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RemoteIp => {
                            if remote_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteIp"));
                            }
                            remote_ip__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServerIp => {
                            if server_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverIp"));
                            }
                            server_ip__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Referer => {
                            if referer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referer"));
                            }
                            referer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Latency => {
                            if latency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("latency"));
                            }
                            latency__ = map_.next_value()?;
                        }
                        GeneratedField::CacheLookup => {
                            if cache_lookup__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cacheLookup"));
                            }
                            cache_lookup__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CacheHit => {
                            if cache_hit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cacheHit"));
                            }
                            cache_hit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CacheValidatedWithOriginServer => {
                            if cache_validated_with_origin_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cacheValidatedWithOriginServer"));
                            }
                            cache_validated_with_origin_server__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CacheFillBytes => {
                            if cache_fill_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cacheFillBytes"));
                            }
                            cache_fill_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HttpRequest {
                    request_method: request_method__.unwrap_or_default(),
                    request_url: request_url__.unwrap_or_default(),
                    request_size: request_size__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    response_size: response_size__.unwrap_or_default(),
                    user_agent: user_agent__.unwrap_or_default(),
                    remote_ip: remote_ip__.unwrap_or_default(),
                    server_ip: server_ip__.unwrap_or_default(),
                    referer: referer__.unwrap_or_default(),
                    latency: latency__,
                    cache_lookup: cache_lookup__.unwrap_or_default(),
                    cache_hit: cache_hit__.unwrap_or_default(),
                    cache_validated_with_origin_server: cache_validated_with_origin_server__.unwrap_or_default(),
                    cache_fill_bytes: cache_fill_bytes__.unwrap_or_default(),
                    protocol: protocol__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.HttpRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogEntry {
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
        if self.timestamp.is_some() {
            len += 1;
        }
        if self.severity != 0 {
            len += 1;
        }
        if self.http_request.is_some() {
            len += 1;
        }
        if !self.trace.is_empty() {
            len += 1;
        }
        if !self.insert_id.is_empty() {
            len += 1;
        }
        if !self.labels.is_empty() {
            len += 1;
        }
        if self.operation.is_some() {
            len += 1;
        }
        if self.source_location.is_some() {
            len += 1;
        }
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.LogEntry", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if self.severity != 0 {
            let v = super::super::super::logging::r#type::LogSeverity::try_from(self.severity)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.severity)))?;
            struct_ser.serialize_field("severity", &v)?;
        }
        if let Some(v) = self.http_request.as_ref() {
            struct_ser.serialize_field("httpRequest", v)?;
        }
        if !self.trace.is_empty() {
            struct_ser.serialize_field("trace", &self.trace)?;
        }
        if !self.insert_id.is_empty() {
            struct_ser.serialize_field("insertId", &self.insert_id)?;
        }
        if !self.labels.is_empty() {
            struct_ser.serialize_field("labels", &self.labels)?;
        }
        if let Some(v) = self.operation.as_ref() {
            struct_ser.serialize_field("operation", v)?;
        }
        if let Some(v) = self.source_location.as_ref() {
            struct_ser.serialize_field("sourceLocation", v)?;
        }
        if let Some(v) = self.payload.as_ref() {
            match v {
                log_entry::Payload::ProtoPayload(v) => {
                    struct_ser.serialize_field("protoPayload", v)?;
                }
                log_entry::Payload::TextPayload(v) => {
                    struct_ser.serialize_field("textPayload", v)?;
                }
                log_entry::Payload::StructPayload(v) => {
                    struct_ser.serialize_field("structPayload", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "timestamp",
            "severity",
            "http_request",
            "httpRequest",
            "trace",
            "insert_id",
            "insertId",
            "labels",
            "operation",
            "source_location",
            "sourceLocation",
            "proto_payload",
            "protoPayload",
            "text_payload",
            "textPayload",
            "struct_payload",
            "structPayload",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Timestamp,
            Severity,
            HttpRequest,
            Trace,
            InsertId,
            Labels,
            Operation,
            SourceLocation,
            ProtoPayload,
            TextPayload,
            StructPayload,
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
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "severity" => Ok(GeneratedField::Severity),
                            "httpRequest" | "http_request" => Ok(GeneratedField::HttpRequest),
                            "trace" => Ok(GeneratedField::Trace),
                            "insertId" | "insert_id" => Ok(GeneratedField::InsertId),
                            "labels" => Ok(GeneratedField::Labels),
                            "operation" => Ok(GeneratedField::Operation),
                            "sourceLocation" | "source_location" => Ok(GeneratedField::SourceLocation),
                            "protoPayload" | "proto_payload" => Ok(GeneratedField::ProtoPayload),
                            "textPayload" | "text_payload" => Ok(GeneratedField::TextPayload),
                            "structPayload" | "struct_payload" => Ok(GeneratedField::StructPayload),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.LogEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LogEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut timestamp__ = None;
                let mut severity__ = None;
                let mut http_request__ = None;
                let mut trace__ = None;
                let mut insert_id__ = None;
                let mut labels__ = None;
                let mut operation__ = None;
                let mut source_location__ = None;
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::Severity => {
                            if severity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("severity"));
                            }
                            severity__ = Some(map_.next_value::<super::super::super::logging::r#type::LogSeverity>()? as i32);
                        }
                        GeneratedField::HttpRequest => {
                            if http_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpRequest"));
                            }
                            http_request__ = map_.next_value()?;
                        }
                        GeneratedField::Trace => {
                            if trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trace"));
                            }
                            trace__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InsertId => {
                            if insert_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("insertId"));
                            }
                            insert_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Labels => {
                            if labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labels"));
                            }
                            labels__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Operation => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation__ = map_.next_value()?;
                        }
                        GeneratedField::SourceLocation => {
                            if source_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceLocation"));
                            }
                            source_location__ = map_.next_value()?;
                        }
                        GeneratedField::ProtoPayload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protoPayload"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(log_entry::Payload::ProtoPayload)
;
                        }
                        GeneratedField::TextPayload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("textPayload"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(log_entry::Payload::TextPayload);
                        }
                        GeneratedField::StructPayload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structPayload"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(log_entry::Payload::StructPayload)
;
                        }
                    }
                }
                Ok(LogEntry {
                    name: name__.unwrap_or_default(),
                    timestamp: timestamp__,
                    severity: severity__.unwrap_or_default(),
                    http_request: http_request__,
                    trace: trace__.unwrap_or_default(),
                    insert_id: insert_id__.unwrap_or_default(),
                    labels: labels__.unwrap_or_default(),
                    operation: operation__,
                    source_location: source_location__,
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.LogEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogEntryOperation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.producer.is_empty() {
            len += 1;
        }
        if self.first {
            len += 1;
        }
        if self.last {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.LogEntryOperation", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.producer.is_empty() {
            struct_ser.serialize_field("producer", &self.producer)?;
        }
        if self.first {
            struct_ser.serialize_field("first", &self.first)?;
        }
        if self.last {
            struct_ser.serialize_field("last", &self.last)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogEntryOperation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "producer",
            "first",
            "last",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Producer,
            First,
            Last,
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
                            "id" => Ok(GeneratedField::Id),
                            "producer" => Ok(GeneratedField::Producer),
                            "first" => Ok(GeneratedField::First),
                            "last" => Ok(GeneratedField::Last),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogEntryOperation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.LogEntryOperation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LogEntryOperation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut producer__ = None;
                let mut first__ = None;
                let mut last__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Producer => {
                            if producer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("producer"));
                            }
                            producer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::First => {
                            if first__.is_some() {
                                return Err(serde::de::Error::duplicate_field("first"));
                            }
                            first__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Last => {
                            if last__.is_some() {
                                return Err(serde::de::Error::duplicate_field("last"));
                            }
                            last__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LogEntryOperation {
                    id: id__.unwrap_or_default(),
                    producer: producer__.unwrap_or_default(),
                    first: first__.unwrap_or_default(),
                    last: last__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.LogEntryOperation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogEntrySourceLocation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file.is_empty() {
            len += 1;
        }
        if self.line != 0 {
            len += 1;
        }
        if !self.function.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.LogEntrySourceLocation", len)?;
        if !self.file.is_empty() {
            struct_ser.serialize_field("file", &self.file)?;
        }
        if self.line != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("line", ToString::to_string(&self.line).as_str())?;
        }
        if !self.function.is_empty() {
            struct_ser.serialize_field("function", &self.function)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogEntrySourceLocation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file",
            "line",
            "function",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            File,
            Line,
            Function,
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
                            "file" => Ok(GeneratedField::File),
                            "line" => Ok(GeneratedField::Line),
                            "function" => Ok(GeneratedField::Function),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogEntrySourceLocation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.LogEntrySourceLocation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LogEntrySourceLocation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file__ = None;
                let mut line__ = None;
                let mut function__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::File => {
                            if file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("file"));
                            }
                            file__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Line => {
                            if line__.is_some() {
                                return Err(serde::de::Error::duplicate_field("line"));
                            }
                            line__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Function => {
                            if function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            function__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LogEntrySourceLocation {
                    file: file__.unwrap_or_default(),
                    line: line__.unwrap_or_default(),
                    function: function__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.LogEntrySourceLocation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MetricValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.labels.is_empty() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.MetricValue", len)?;
        if !self.labels.is_empty() {
            struct_ser.serialize_field("labels", &self.labels)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            match v {
                metric_value::Value::BoolValue(v) => {
                    struct_ser.serialize_field("boolValue", v)?;
                }
                metric_value::Value::Int64Value(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("int64Value", ToString::to_string(&v).as_str())?;
                }
                metric_value::Value::DoubleValue(v) => {
                    struct_ser.serialize_field("doubleValue", v)?;
                }
                metric_value::Value::StringValue(v) => {
                    struct_ser.serialize_field("stringValue", v)?;
                }
                metric_value::Value::DistributionValue(v) => {
                    struct_ser.serialize_field("distributionValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MetricValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "labels",
            "start_time",
            "startTime",
            "end_time",
            "endTime",
            "bool_value",
            "boolValue",
            "int64_value",
            "int64Value",
            "double_value",
            "doubleValue",
            "string_value",
            "stringValue",
            "distribution_value",
            "distributionValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Labels,
            StartTime,
            EndTime,
            BoolValue,
            Int64Value,
            DoubleValue,
            StringValue,
            DistributionValue,
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
                            "labels" => Ok(GeneratedField::Labels),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            "boolValue" | "bool_value" => Ok(GeneratedField::BoolValue),
                            "int64Value" | "int64_value" => Ok(GeneratedField::Int64Value),
                            "doubleValue" | "double_value" => Ok(GeneratedField::DoubleValue),
                            "stringValue" | "string_value" => Ok(GeneratedField::StringValue),
                            "distributionValue" | "distribution_value" => Ok(GeneratedField::DistributionValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MetricValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.MetricValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MetricValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut labels__ = None;
                let mut start_time__ = None;
                let mut end_time__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Labels => {
                            if labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labels"));
                            }
                            labels__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::EndTime => {
                            if end_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTime"));
                            }
                            end_time__ = map_.next_value()?;
                        }
                        GeneratedField::BoolValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("boolValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(metric_value::Value::BoolValue);
                        }
                        GeneratedField::Int64Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("int64Value"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| metric_value::Value::Int64Value(x.0));
                        }
                        GeneratedField::DoubleValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("doubleValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| metric_value::Value::DoubleValue(x.0));
                        }
                        GeneratedField::StringValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(metric_value::Value::StringValue);
                        }
                        GeneratedField::DistributionValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("distributionValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(metric_value::Value::DistributionValue)
;
                        }
                    }
                }
                Ok(MetricValue {
                    labels: labels__.unwrap_or_default(),
                    start_time: start_time__,
                    end_time: end_time__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.MetricValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MetricValueSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.metric_name.is_empty() {
            len += 1;
        }
        if !self.metric_values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.MetricValueSet", len)?;
        if !self.metric_name.is_empty() {
            struct_ser.serialize_field("metricName", &self.metric_name)?;
        }
        if !self.metric_values.is_empty() {
            struct_ser.serialize_field("metricValues", &self.metric_values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MetricValueSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metric_name",
            "metricName",
            "metric_values",
            "metricValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MetricName,
            MetricValues,
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
                            "metricName" | "metric_name" => Ok(GeneratedField::MetricName),
                            "metricValues" | "metric_values" => Ok(GeneratedField::MetricValues),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MetricValueSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.MetricValueSet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MetricValueSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metric_name__ = None;
                let mut metric_values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MetricName => {
                            if metric_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metricName"));
                            }
                            metric_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MetricValues => {
                            if metric_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metricValues"));
                            }
                            metric_values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MetricValueSet {
                    metric_name: metric_name__.unwrap_or_default(),
                    metric_values: metric_values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.MetricValueSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Operation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.operation_id.is_empty() {
            len += 1;
        }
        if !self.operation_name.is_empty() {
            len += 1;
        }
        if !self.consumer_id.is_empty() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        if !self.labels.is_empty() {
            len += 1;
        }
        if !self.metric_value_sets.is_empty() {
            len += 1;
        }
        if !self.log_entries.is_empty() {
            len += 1;
        }
        if self.importance != 0 {
            len += 1;
        }
        if !self.extensions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.Operation", len)?;
        if !self.operation_id.is_empty() {
            struct_ser.serialize_field("operationId", &self.operation_id)?;
        }
        if !self.operation_name.is_empty() {
            struct_ser.serialize_field("operationName", &self.operation_name)?;
        }
        if !self.consumer_id.is_empty() {
            struct_ser.serialize_field("consumerId", &self.consumer_id)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
        }
        if !self.labels.is_empty() {
            struct_ser.serialize_field("labels", &self.labels)?;
        }
        if !self.metric_value_sets.is_empty() {
            struct_ser.serialize_field("metricValueSets", &self.metric_value_sets)?;
        }
        if !self.log_entries.is_empty() {
            struct_ser.serialize_field("logEntries", &self.log_entries)?;
        }
        if self.importance != 0 {
            let v = operation::Importance::try_from(self.importance)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.importance)))?;
            struct_ser.serialize_field("importance", &v)?;
        }
        if !self.extensions.is_empty() {
            struct_ser.serialize_field("extensions", &self.extensions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Operation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operation_id",
            "operationId",
            "operation_name",
            "operationName",
            "consumer_id",
            "consumerId",
            "start_time",
            "startTime",
            "end_time",
            "endTime",
            "labels",
            "metric_value_sets",
            "metricValueSets",
            "log_entries",
            "logEntries",
            "importance",
            "extensions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OperationId,
            OperationName,
            ConsumerId,
            StartTime,
            EndTime,
            Labels,
            MetricValueSets,
            LogEntries,
            Importance,
            Extensions,
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
                            "operationId" | "operation_id" => Ok(GeneratedField::OperationId),
                            "operationName" | "operation_name" => Ok(GeneratedField::OperationName),
                            "consumerId" | "consumer_id" => Ok(GeneratedField::ConsumerId),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            "labels" => Ok(GeneratedField::Labels),
                            "metricValueSets" | "metric_value_sets" => Ok(GeneratedField::MetricValueSets),
                            "logEntries" | "log_entries" => Ok(GeneratedField::LogEntries),
                            "importance" => Ok(GeneratedField::Importance),
                            "extensions" => Ok(GeneratedField::Extensions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Operation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.Operation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Operation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operation_id__ = None;
                let mut operation_name__ = None;
                let mut consumer_id__ = None;
                let mut start_time__ = None;
                let mut end_time__ = None;
                let mut labels__ = None;
                let mut metric_value_sets__ = None;
                let mut log_entries__ = None;
                let mut importance__ = None;
                let mut extensions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OperationId => {
                            if operation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operationId"));
                            }
                            operation_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OperationName => {
                            if operation_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operationName"));
                            }
                            operation_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConsumerId => {
                            if consumer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consumerId"));
                            }
                            consumer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::EndTime => {
                            if end_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTime"));
                            }
                            end_time__ = map_.next_value()?;
                        }
                        GeneratedField::Labels => {
                            if labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labels"));
                            }
                            labels__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::MetricValueSets => {
                            if metric_value_sets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metricValueSets"));
                            }
                            metric_value_sets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LogEntries => {
                            if log_entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logEntries"));
                            }
                            log_entries__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Importance => {
                            if importance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("importance"));
                            }
                            importance__ = Some(map_.next_value::<operation::Importance>()? as i32);
                        }
                        GeneratedField::Extensions => {
                            if extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensions"));
                            }
                            extensions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Operation {
                    operation_id: operation_id__.unwrap_or_default(),
                    operation_name: operation_name__.unwrap_or_default(),
                    consumer_id: consumer_id__.unwrap_or_default(),
                    start_time: start_time__,
                    end_time: end_time__,
                    labels: labels__.unwrap_or_default(),
                    metric_value_sets: metric_value_sets__.unwrap_or_default(),
                    log_entries: log_entries__.unwrap_or_default(),
                    importance: importance__.unwrap_or_default(),
                    extensions: extensions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.Operation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for operation::Importance {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Low => "LOW",
            Self::High => "HIGH",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for operation::Importance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "LOW",
            "HIGH",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = operation::Importance;

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
                    "LOW" => Ok(operation::Importance::Low),
                    "HIGH" => Ok(operation::Importance::High),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if !self.subject.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.QuotaError", len)?;
        if self.code != 0 {
            let v = quota_error::Code::try_from(self.code)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.code)))?;
            struct_ser.serialize_field("code", &v)?;
        }
        if !self.subject.is_empty() {
            struct_ser.serialize_field("subject", &self.subject)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuotaError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "subject",
            "description",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Subject,
            Description,
            Status,
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
                            "code" => Ok(GeneratedField::Code),
                            "subject" => Ok(GeneratedField::Subject),
                            "description" => Ok(GeneratedField::Description),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.QuotaError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuotaError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut subject__ = None;
                let mut description__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value::<quota_error::Code>()? as i32);
                        }
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QuotaError {
                    code: code__.unwrap_or_default(),
                    subject: subject__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.QuotaError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for quota_error::Code {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::ResourceExhausted => "RESOURCE_EXHAUSTED",
            Self::BillingNotActive => "BILLING_NOT_ACTIVE",
            Self::ProjectDeleted => "PROJECT_DELETED",
            Self::ApiKeyInvalid => "API_KEY_INVALID",
            Self::ApiKeyExpired => "API_KEY_EXPIRED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for quota_error::Code {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNSPECIFIED",
            "RESOURCE_EXHAUSTED",
            "BILLING_NOT_ACTIVE",
            "PROJECT_DELETED",
            "API_KEY_INVALID",
            "API_KEY_EXPIRED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = quota_error::Code;

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
                    "UNSPECIFIED" => Ok(quota_error::Code::Unspecified),
                    "RESOURCE_EXHAUSTED" => Ok(quota_error::Code::ResourceExhausted),
                    "BILLING_NOT_ACTIVE" => Ok(quota_error::Code::BillingNotActive),
                    "PROJECT_DELETED" => Ok(quota_error::Code::ProjectDeleted),
                    "API_KEY_INVALID" => Ok(quota_error::Code::ApiKeyInvalid),
                    "API_KEY_EXPIRED" => Ok(quota_error::Code::ApiKeyExpired),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaOperation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.operation_id.is_empty() {
            len += 1;
        }
        if !self.method_name.is_empty() {
            len += 1;
        }
        if !self.consumer_id.is_empty() {
            len += 1;
        }
        if !self.labels.is_empty() {
            len += 1;
        }
        if !self.quota_metrics.is_empty() {
            len += 1;
        }
        if self.quota_mode != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.QuotaOperation", len)?;
        if !self.operation_id.is_empty() {
            struct_ser.serialize_field("operationId", &self.operation_id)?;
        }
        if !self.method_name.is_empty() {
            struct_ser.serialize_field("methodName", &self.method_name)?;
        }
        if !self.consumer_id.is_empty() {
            struct_ser.serialize_field("consumerId", &self.consumer_id)?;
        }
        if !self.labels.is_empty() {
            struct_ser.serialize_field("labels", &self.labels)?;
        }
        if !self.quota_metrics.is_empty() {
            struct_ser.serialize_field("quotaMetrics", &self.quota_metrics)?;
        }
        if self.quota_mode != 0 {
            let v = quota_operation::QuotaMode::try_from(self.quota_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.quota_mode)))?;
            struct_ser.serialize_field("quotaMode", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuotaOperation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operation_id",
            "operationId",
            "method_name",
            "methodName",
            "consumer_id",
            "consumerId",
            "labels",
            "quota_metrics",
            "quotaMetrics",
            "quota_mode",
            "quotaMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OperationId,
            MethodName,
            ConsumerId,
            Labels,
            QuotaMetrics,
            QuotaMode,
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
                            "operationId" | "operation_id" => Ok(GeneratedField::OperationId),
                            "methodName" | "method_name" => Ok(GeneratedField::MethodName),
                            "consumerId" | "consumer_id" => Ok(GeneratedField::ConsumerId),
                            "labels" => Ok(GeneratedField::Labels),
                            "quotaMetrics" | "quota_metrics" => Ok(GeneratedField::QuotaMetrics),
                            "quotaMode" | "quota_mode" => Ok(GeneratedField::QuotaMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaOperation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.QuotaOperation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuotaOperation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operation_id__ = None;
                let mut method_name__ = None;
                let mut consumer_id__ = None;
                let mut labels__ = None;
                let mut quota_metrics__ = None;
                let mut quota_mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OperationId => {
                            if operation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operationId"));
                            }
                            operation_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MethodName => {
                            if method_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("methodName"));
                            }
                            method_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConsumerId => {
                            if consumer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consumerId"));
                            }
                            consumer_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Labels => {
                            if labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labels"));
                            }
                            labels__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::QuotaMetrics => {
                            if quota_metrics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotaMetrics"));
                            }
                            quota_metrics__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuotaMode => {
                            if quota_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotaMode"));
                            }
                            quota_mode__ = Some(map_.next_value::<quota_operation::QuotaMode>()? as i32);
                        }
                    }
                }
                Ok(QuotaOperation {
                    operation_id: operation_id__.unwrap_or_default(),
                    method_name: method_name__.unwrap_or_default(),
                    consumer_id: consumer_id__.unwrap_or_default(),
                    labels: labels__.unwrap_or_default(),
                    quota_metrics: quota_metrics__.unwrap_or_default(),
                    quota_mode: quota_mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.QuotaOperation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for quota_operation::QuotaMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::Normal => "NORMAL",
            Self::BestEffort => "BEST_EFFORT",
            Self::CheckOnly => "CHECK_ONLY",
            Self::QueryOnly => "QUERY_ONLY",
            Self::AdjustOnly => "ADJUST_ONLY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for quota_operation::QuotaMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNSPECIFIED",
            "NORMAL",
            "BEST_EFFORT",
            "CHECK_ONLY",
            "QUERY_ONLY",
            "ADJUST_ONLY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = quota_operation::QuotaMode;

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
                    "UNSPECIFIED" => Ok(quota_operation::QuotaMode::Unspecified),
                    "NORMAL" => Ok(quota_operation::QuotaMode::Normal),
                    "BEST_EFFORT" => Ok(quota_operation::QuotaMode::BestEffort),
                    "CHECK_ONLY" => Ok(quota_operation::QuotaMode::CheckOnly),
                    "QUERY_ONLY" => Ok(quota_operation::QuotaMode::QueryOnly),
                    "ADJUST_ONLY" => Ok(quota_operation::QuotaMode::AdjustOnly),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ReportRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.service_name.is_empty() {
            len += 1;
        }
        if !self.operations.is_empty() {
            len += 1;
        }
        if !self.service_config_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.ReportRequest", len)?;
        if !self.service_name.is_empty() {
            struct_ser.serialize_field("serviceName", &self.service_name)?;
        }
        if !self.operations.is_empty() {
            struct_ser.serialize_field("operations", &self.operations)?;
        }
        if !self.service_config_id.is_empty() {
            struct_ser.serialize_field("serviceConfigId", &self.service_config_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service_name",
            "serviceName",
            "operations",
            "service_config_id",
            "serviceConfigId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServiceName,
            Operations,
            ServiceConfigId,
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
                            "serviceName" | "service_name" => Ok(GeneratedField::ServiceName),
                            "operations" => Ok(GeneratedField::Operations),
                            "serviceConfigId" | "service_config_id" => Ok(GeneratedField::ServiceConfigId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.ReportRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service_name__ = None;
                let mut operations__ = None;
                let mut service_config_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ServiceName => {
                            if service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceName"));
                            }
                            service_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Operations => {
                            if operations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operations"));
                            }
                            operations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServiceConfigId => {
                            if service_config_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceConfigId"));
                            }
                            service_config_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReportRequest {
                    service_name: service_name__.unwrap_or_default(),
                    operations: operations__.unwrap_or_default(),
                    service_config_id: service_config_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.ReportRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.report_errors.is_empty() {
            len += 1;
        }
        if !self.service_config_id.is_empty() {
            len += 1;
        }
        if !self.service_rollout_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.ReportResponse", len)?;
        if !self.report_errors.is_empty() {
            struct_ser.serialize_field("reportErrors", &self.report_errors)?;
        }
        if !self.service_config_id.is_empty() {
            struct_ser.serialize_field("serviceConfigId", &self.service_config_id)?;
        }
        if !self.service_rollout_id.is_empty() {
            struct_ser.serialize_field("serviceRolloutId", &self.service_rollout_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report_errors",
            "reportErrors",
            "service_config_id",
            "serviceConfigId",
            "service_rollout_id",
            "serviceRolloutId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportErrors,
            ServiceConfigId,
            ServiceRolloutId,
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
                            "reportErrors" | "report_errors" => Ok(GeneratedField::ReportErrors),
                            "serviceConfigId" | "service_config_id" => Ok(GeneratedField::ServiceConfigId),
                            "serviceRolloutId" | "service_rollout_id" => Ok(GeneratedField::ServiceRolloutId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.ReportResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report_errors__ = None;
                let mut service_config_id__ = None;
                let mut service_rollout_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReportErrors => {
                            if report_errors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportErrors"));
                            }
                            report_errors__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServiceConfigId => {
                            if service_config_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceConfigId"));
                            }
                            service_config_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServiceRolloutId => {
                            if service_rollout_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceRolloutId"));
                            }
                            service_rollout_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReportResponse {
                    report_errors: report_errors__.unwrap_or_default(),
                    service_config_id: service_config_id__.unwrap_or_default(),
                    service_rollout_id: service_rollout_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.ReportResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for report_response::ReportError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.operation_id.is_empty() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.servicecontrol.v1.ReportResponse.ReportError", len)?;
        if !self.operation_id.is_empty() {
            struct_ser.serialize_field("operationId", &self.operation_id)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for report_response::ReportError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operation_id",
            "operationId",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OperationId,
            Status,
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
                            "operationId" | "operation_id" => Ok(GeneratedField::OperationId),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = report_response::ReportError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.servicecontrol.v1.ReportResponse.ReportError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<report_response::ReportError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operation_id__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OperationId => {
                            if operation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operationId"));
                            }
                            operation_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(report_response::ReportError {
                    operation_id: operation_id__.unwrap_or_default(),
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.servicecontrol.v1.ReportResponse.ReportError", FIELDS, GeneratedVisitor)
    }
}
