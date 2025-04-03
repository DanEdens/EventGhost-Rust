use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;
use async_trait::async_trait;
use tokio::task;
use serde::{Serialize, Deserialize};
use windows::Win32::System::Registry::{
    HKEY, HKEY_CLASSES_ROOT, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, 
    HKEY_USERS, HKEY_CURRENT_CONFIG, RegOpenKeyExW, RegCloseKey, 
    RegQueryValueExW, RegSetValueExW, RegDeleteKeyW, RegDeleteValueW,
    RegEnumKeyExW, RegEnumValueW, REG_VALUE_TYPE,
    REG_SZ, REG_EXPAND_SZ, REG_BINARY, REG_DWORD, REG_QWORD, KEY_READ, 
    KEY_WRITE, KEY_ALL_ACCESS, KEY_WOW64_64KEY, REG_SAM_FLAGS
};
use windows::core::{PCWSTR, PWSTR};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use crate::core::action::{Action, ActionConfig, ActionResult};
use crate::core::Error;
use crate::core::event::{Event, EventType};
use crate::core::plugin::Plugin;
/// Supported registry operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RegistryOperation {
    /// Read a registry value
    Read,
    /// Write a registry value
    Write,
    /// Delete a registry key
    DeleteKey,
    /// Delete a registry value
    DeleteValue,
    /// Check if a registry key exists
    KeyExists,
    /// Check if a registry value exists
    ValueExists,
    /// Enumerate subkeys of a registry key
    EnumerateKeys,
    /// Enumerate values of a registry key
    EnumerateValues,
}

impl RegistryOperation {
    fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "read" => Some(RegistryOperation::Read),
            "write" => Some(RegistryOperation::Write),
            "deletekey" => Some(RegistryOperation::DeleteKey),
            "deletevalue" => Some(RegistryOperation::DeleteValue),
            "keyexists" => Some(RegistryOperation::KeyExists),
            "valueexists" => Some(RegistryOperation::ValueExists),
            "enumkeys" | "enumeratekeys" => Some(RegistryOperation::EnumerateKeys),
            "enumvalues" | "enumeratevalues" => Some(RegistryOperation::EnumerateValues),
            _ => None,
        }
    }
}
/// Registry hives
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RegistryHive {
    /// HKEY_CLASSES_ROOT
    ClassesRoot,
    /// HKEY_CURRENT_USER
    CurrentUser,
    /// HKEY_LOCAL_MACHINE
    LocalMachine,
    /// HKEY_USERS
    Users,
    /// HKEY_CURRENT_CONFIG
    CurrentConfig,
}

impl RegistryHive {
    fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "hkcr" | "classesroot" | "hkey_classes_root" => Some(RegistryHive::ClassesRoot),
            "hkcu" | "currentuser" | "hkey_current_user" => Some(RegistryHive::CurrentUser),
            "hklm" | "localmachine" | "hkey_local_machine" => Some(RegistryHive::LocalMachine),
            "hku" | "users" | "hkey_users" => Some(RegistryHive::Users),
            "hkcc" | "currentconfig" | "hkey_current_config" => Some(RegistryHive::CurrentConfig),
            _ => None,
        }
    }

    fn to_hkey(&self) -> HKEY {
        match self {
            RegistryHive::ClassesRoot => HKEY_CLASSES_ROOT,
            RegistryHive::CurrentUser => HKEY_CURRENT_USER,
            RegistryHive::LocalMachine => HKEY_LOCAL_MACHINE,
            RegistryHive::Users => HKEY_USERS,
            RegistryHive::CurrentConfig => HKEY_CURRENT_CONFIG,
        }
    }
}
/// Registry value types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RegistryValueType {
    /// String value (REG_SZ)
    String,
    /// Expandable string value (REG_EXPAND_SZ)
    ExpandString,
    /// Binary value (REG_BINARY)
    Binary,
    /// DWORD value (REG_DWORD)
    Dword,
    /// QWORD value (REG_QWORD)
    Qword,
}

impl RegistryValueType {
    fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "string" | "reg_sz" => Some(RegistryValueType::String),
            "expandstring" | "reg_expand_sz" => Some(RegistryValueType::ExpandString),
            "binary" | "reg_binary" => Some(RegistryValueType::Binary),
            "dword" | "reg_dword" => Some(RegistryValueType::Dword),
            "qword" | "reg_qword" => Some(RegistryValueType::Qword),
            _ => None,
        }
    }

    fn to_reg_type(&self) -> REG_VALUE_TYPE {
        match self {
            RegistryValueType::String => REG_SZ,
            RegistryValueType::ExpandString => REG_EXPAND_SZ,
            RegistryValueType::Binary => REG_BINARY,
            RegistryValueType::Dword => REG_DWORD,
            RegistryValueType::Qword => REG_QWORD,
        }
    }
}

/// Registry value data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistryValue {
    /// String value
    String(String),
    /// Expandable string value
    ExpandString(String),
    /// Binary value
    Binary(Vec<u8>),
    /// DWORD value
    Dword(u32),
    /// QWORD value
    Qword(u64),
}
/// Configuration for the RegistryOperationsAction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryOperationsConfig {
    /// The operation to perform
    pub operation: RegistryOperation,
    /// The registry hive
    pub hive: RegistryHive,
    /// The registry key path
    pub key: String,
    /// The registry value name (for value operations)
    pub value_name: Option<String>,
    /// The registry value data (for write operations)
    pub value_data: Option<RegistryValue>,
    /// The value type (for write operations)
    pub value_type: Option<RegistryValueType>,
    /// Whether to use 64-bit registry view
    pub use_64bit: bool,
}

impl Default for RegistryOperationsConfig {
    fn default() -> Self {
        Self {
            operation: RegistryOperation::Read,
            hive: RegistryHive::CurrentUser,
            key: String::new(),
            value_name: None,
            value_data: None,
            value_type: None,
            use_64bit: true,
        }
    }
}
/// An action that performs registry operations
#[derive(Debug, Clone)]
pub struct RegistryOperationsAction {
    id: Uuid,
    plugin: Arc<dyn Plugin>,
    config: RegistryOperationsConfig,
}

impl RegistryOperationsAction {
    /// Create a new RegistryOperationsAction
    pub fn new(plugin: Arc<dyn Plugin>) -> Self {
        Self {
            id: Uuid::new_v4(),
            plugin,
            config: RegistryOperationsConfig::default(),
        }
    }
    
    /// Create a new RegistryOperationsAction with a specific ID
    pub fn with_id(id: Uuid, plugin: Arc<dyn Plugin>) -> Self {
        Self {
            id,
            plugin,
            config: RegistryOperationsConfig::default(),
        }
    }
    
    /// Set the operation to perform
    pub fn with_operation(mut self, operation: RegistryOperation) -> Self {
        self.config.operation = operation;
        self
    }
    
    /// Set the registry hive
    pub fn with_hive(mut self, hive: RegistryHive) -> Self {
        self.config.hive = hive;
        self
    }
    
    /// Set the registry key path
    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.config.key = key.into();
        self
    }
    
    /// Set the registry value name
    pub fn with_value_name(mut self, name: impl Into<String>) -> Self {
        self.config.value_name = Some(name.into());
        self
    }
    
    /// Set the registry value data for string values
    pub fn with_string_value(mut self, value: impl Into<String>) -> Self {
        self.config.value_data = Some(RegistryValue::String(value.into()));
        self.config.value_type = Some(RegistryValueType::String);
        self
    }
    
    /// Set the registry value data for expandable string values
    pub fn with_expand_string_value(mut self, value: impl Into<String>) -> Self {
        self.config.value_data = Some(RegistryValue::ExpandString(value.into()));
        self.config.value_type = Some(RegistryValueType::ExpandString);
        self
    }
    
    /// Set the registry value data for binary values
    pub fn with_binary_value(mut self, value: Vec<u8>) -> Self {
        self.config.value_data = Some(RegistryValue::Binary(value));
        self.config.value_type = Some(RegistryValueType::Binary);
        self
    }
    
    /// Set the registry value data for DWORD values
    pub fn with_dword_value(mut self, value: u32) -> Self {
        self.config.value_data = Some(RegistryValue::Dword(value));
        self.config.value_type = Some(RegistryValueType::Dword);
        self
    }
    
    /// Set the registry value data for QWORD values
    pub fn with_qword_value(mut self, value: u64) -> Self {
        self.config.value_data = Some(RegistryValue::Qword(value));
        self.config.value_type = Some(RegistryValueType::Qword);
        self
    }
    
    /// Set whether to use 64-bit registry view
    pub fn with_64bit_view(mut self, use_64bit: bool) -> Self {
        self.config.use_64bit = use_64bit;
        self
    }
}
impl RegistryOperationsAction {
    /// Convert a Rust string to a wide string for Windows API
    fn to_wide_string(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }
    
    /// Open a registry key
    fn open_key(&self, hkey: HKEY, subkey: &str, writable: bool) -> windows::core::Result<HKEY> {
        let subkey_wide = Self::to_wide_string(subkey);
        let mut result = HKEY::default();
        let access = if writable { 
            KEY_WRITE.0 | KEY_READ.0 
        } else { 
            KEY_READ.0 
        };
        
        let access = if self.config.use_64bit {
            access | KEY_WOW64_64KEY.0
        } else {
            access
        };
        
        unsafe {
            RegOpenKeyExW(
                hkey,
                PCWSTR::from_raw(subkey_wide.as_ptr()),
                0,
                REG_SAM_FLAGS(access),
                &mut result,
            )?;
        }
        
        Ok(result)
    }
    
    /// Read a registry value
    async fn read_value(&self) -> Result<RegistryValue, Error> {
        // Define constants for pattern matching
        const REG_SZ_VALUE: u32 = REG_SZ.0;
        const REG_EXPAND_SZ_VALUE: u32 = REG_EXPAND_SZ.0;
        const REG_BINARY_VALUE: u32 = REG_BINARY.0;
        const REG_DWORD_VALUE: u32 = REG_DWORD.0;
        const REG_QWORD_VALUE: u32 = REG_QWORD.0;
        
        let hkey = self.config.hive.to_hkey();
        let key_handle = self.open_key(hkey, &self.config.key, false)
            .map_err(|e| Error::InvalidOperation(format!("Failed to open registry key: {}", e)))?;
        
        let value_name = self.config.value_name.as_deref().unwrap_or("");
        let value_name_wide = Self::to_wide_string(value_name);
        
        // First call to get the data type and size
        let mut value_type: u32 = 0;
        let mut data_size = 0u32;
        
        let result = unsafe {
            RegQueryValueExW(
                key_handle,
                PCWSTR::from_raw(value_name_wide.as_ptr()),
                None,
                Some(&mut value_type as *mut u32 as *mut _),
                None,
                Some(&mut data_size),
            )
        };
        
        if let Err(e) = result {
            unsafe { RegCloseKey(key_handle) };
            return Err(Error::InvalidOperation(format!("Failed to query registry value: {}", e)));
        }
        
        // Allocate buffer based on the size
        let mut buffer = vec![0u8; data_size as usize];
        
        // Second call to get the data
        let result = unsafe {
            RegQueryValueExW(
                key_handle,
                PCWSTR::from_raw(value_name_wide.as_ptr()),
                None,
                Some(&mut value_type as *mut u32 as *mut _),
                Some(buffer.as_mut_ptr()),
                Some(&mut data_size),
            )
        };
        
        unsafe { RegCloseKey(key_handle) };
        
        if let Err(e) = result {
            return Err(Error::InvalidOperation(format!("Failed to read registry value: {}", e)));
        }
        
        // Convert the raw buffer to the appropriate type based on value_type
        match value_type {
            // Use constants for pattern matching
            v if v == REG_SZ_VALUE || v == REG_EXPAND_SZ_VALUE => {
                // Convert wide string buffer to Rust string
                let len = buffer.len() / 2;
                if len == 0 {
                    return Ok(RegistryValue::String(String::new()));
                }
                
                let wide_chars = unsafe {
                    std::slice::from_raw_parts(buffer.as_ptr() as *const u16, len)
                };
                
                // Find null terminator if present
                let end = wide_chars.iter().position(|&c| c == 0).unwrap_or(len);
                let os_string = OsString::from_wide(&wide_chars[0..end]);
                let string = os_string.to_string_lossy().into_owned();
                
                if value_type == REG_SZ_VALUE {
                    Ok(RegistryValue::String(string))
                } else {
                    Ok(RegistryValue::ExpandString(string))
                }
            },
            v if v == REG_DWORD_VALUE => {
                let value = u32::from_ne_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                Ok(RegistryValue::Dword(value))
            },
            v if v == REG_QWORD_VALUE => {
                let value = u64::from_ne_bytes([
                    buffer[0], buffer[1], buffer[2], buffer[3],
                    buffer[4], buffer[5], buffer[6], buffer[7],
                ]);
                Ok(RegistryValue::Qword(value))
            },
            v if v == REG_BINARY_VALUE => {
                Ok(RegistryValue::Binary(buffer))
            },
            _ => {
                Err(Error::InvalidOperation(format!("Unsupported registry value type: {}", value_type)))
            }
        }
    }
    
    /// Write a registry value
    async fn write_value(&self) -> Result<(), Error> {
        if self.config.value_data.is_none() || self.config.value_type.is_none() {
            return Err(Error::InvalidOperation("Value data and type must be specified for write operation".to_string()));
        }
        
        let hkey = self.config.hive.to_hkey();
        let key_handle = self.open_key(hkey, &self.config.key, true)
            .map_err(|e| Error::InvalidOperation(format!("Failed to open registry key: {}", e)))?;
            
        let value_name = self.config.value_name.as_deref().unwrap_or("");
        let value_name_wide = Self::to_wide_string(value_name);
        
        let result = match &self.config.value_data {
            Some(RegistryValue::String(s)) => {
                let wide = Self::to_wide_string(s);
                unsafe {
                    RegSetValueExW(
                        key_handle,
                        PCWSTR::from_raw(value_name_wide.as_ptr()),
                        0,
                        REG_SZ,
                        Some(std::slice::from_raw_parts(wide.as_ptr() as *const u8, wide.len() * 2)),
                    )
                }
            },
            Some(RegistryValue::ExpandString(s)) => {
                let wide = Self::to_wide_string(s);
                unsafe {
                    RegSetValueExW(
                        key_handle,
                        PCWSTR::from_raw(value_name_wide.as_ptr()),
                        0,
                        REG_EXPAND_SZ,
                        Some(std::slice::from_raw_parts(wide.as_ptr() as *const u8, wide.len() * 2)),
                    )
                }
            },
            Some(RegistryValue::Dword(val)) => {
                let bytes = val.to_ne_bytes();
                unsafe {
                    RegSetValueExW(
                        key_handle,
                        PCWSTR::from_raw(value_name_wide.as_ptr()),
                        0,
                        REG_DWORD,
                        Some(&bytes),
                    )
                }
            },
            Some(RegistryValue::Qword(val)) => {
                let bytes = val.to_ne_bytes();
                unsafe {
                    RegSetValueExW(
                        key_handle,
                        PCWSTR::from_raw(value_name_wide.as_ptr()),
                        0,
                        REG_QWORD,
                        Some(&bytes),
                    )
                }
            },
            Some(RegistryValue::Binary(bytes)) => {
                unsafe {
                    RegSetValueExW(
                        key_handle,
                        PCWSTR::from_raw(value_name_wide.as_ptr()),
                        0,
                        REG_BINARY,
                        Some(bytes),
                    )
                }
            },
            None => {
                unsafe { RegCloseKey(key_handle) };
                return Err(Error::InvalidOperation("No value data specified".to_string()));
            }
        };
        
        unsafe { RegCloseKey(key_handle) };
        
        result.map_err(|e| Error::InvalidOperation(format!("Failed to write registry value: {}", e)))
    }
    
    /// Delete a registry key
    async fn delete_key(&self) -> Result<(), Error> {
        let hkey = self.config.hive.to_hkey();
        let key_wide = Self::to_wide_string(&self.config.key);
        
        let result = unsafe {
            RegDeleteKeyW(
                hkey,
                PCWSTR::from_raw(key_wide.as_ptr()),
            )
        };
        
        result.map_err(|e| Error::InvalidOperation(format!("Failed to delete registry key: {}", e)))
    }
    
    /// Delete a registry value
    async fn delete_value(&self) -> Result<(), Error> {
        let value_name = match &self.config.value_name {
            Some(name) => name,
            None => return Err(Error::InvalidOperation("Value name must be specified for delete value operation".to_string())),
        };
        
        let hkey = self.config.hive.to_hkey();
        let key_handle = self.open_key(hkey, &self.config.key, true)
            .map_err(|e| Error::InvalidOperation(format!("Failed to open registry key: {}", e)))?;
            
        let value_name_wide = Self::to_wide_string(value_name);
        
        let result = unsafe {
            RegDeleteValueW(
                key_handle,
                PCWSTR::from_raw(value_name_wide.as_ptr()),
            )
        };
        
        unsafe { RegCloseKey(key_handle) };
        
        result.map_err(|e| Error::InvalidOperation(format!("Failed to delete registry value: {}", e)))
    }
    
    /// Check if a registry key exists
    async fn key_exists(&self) -> Result<bool, Error> {
        let hkey = self.config.hive.to_hkey();
        match self.open_key(hkey, &self.config.key, false) {
            Ok(key_handle) => {
                unsafe { RegCloseKey(key_handle) };
                Ok(true)
            },
            Err(_) => Ok(false),
        }
    }
    
    /// Check if a registry value exists
    async fn value_exists(&self) -> Result<bool, Error> {
        let hkey = self.config.hive.to_hkey();
        let key_handle = match self.open_key(hkey, &self.config.key, false) {
            Ok(handle) => handle,
            Err(_) => return Ok(false),
        };
        
        let value_name = self.config.value_name.as_deref().unwrap_or("");
        let value_name_wide = Self::to_wide_string(value_name);
        
        let mut value_type: u32 = 0;
        let mut data_size = 0u32;
        
        let result = unsafe {
            RegQueryValueExW(
                key_handle,
                PCWSTR::from_raw(value_name_wide.as_ptr()),
                None,
                Some(&mut value_type as *mut u32 as *mut _),
                None,
                Some(&mut data_size),
            )
        };
        
        unsafe { RegCloseKey(key_handle) };
        
        Ok(result.is_ok())
    }
    
    /// Enumerate registry keys
    async fn enumerate_keys(&self) -> Result<Vec<String>, Error> {
        let hkey = self.config.hive.to_hkey();
        let key_handle = self.open_key(hkey, &self.config.key, false)
            .map_err(|e| Error::InvalidOperation(format!("Failed to open registry key: {}", e)))?;
            
        let mut keys = Vec::new();
        let mut index = 0;
        
        loop {
            let mut name_buf = vec![0u16; 256]; // Maximum registry key name length
            let mut name_size = name_buf.len() as u32;
            
            let result = unsafe {
                RegEnumKeyExW(
                    key_handle,
                    index,
                    PWSTR::from_raw(name_buf.as_mut_ptr()),
                    &mut name_size,
                    None,
                    PWSTR::null(),
                    None,
                    None,
                )
            };
            
            if result.is_err() {
                break;
            }
            
            if name_size > 0 {
                let os_string = OsString::from_wide(&name_buf[0..name_size as usize]);
                keys.push(os_string.to_string_lossy().into_owned());
            }
            
            index += 1;
        }
        
        unsafe { RegCloseKey(key_handle) };
        
        Ok(keys)
    }
    
    /// Enumerate registry values
    async fn enumerate_values(&self) -> Result<Vec<(String, RegistryValueType)>, Error> {
        // Define constants for pattern matching
        const REG_SZ_VALUE: u32 = REG_SZ.0;
        const REG_EXPAND_SZ_VALUE: u32 = REG_EXPAND_SZ.0;
        const REG_BINARY_VALUE: u32 = REG_BINARY.0;
        const REG_DWORD_VALUE: u32 = REG_DWORD.0;
        const REG_QWORD_VALUE: u32 = REG_QWORD.0;
        
        let hkey = self.config.hive.to_hkey();
        let key_handle = self.open_key(hkey, &self.config.key, false)
            .map_err(|e| Error::InvalidOperation(format!("Failed to open registry key: {}", e)))?;
            
        let mut values = Vec::new();
        let mut index = 0;
        
        loop {
            let mut name_buf = vec![0u16; 16383]; // Maximum registry value name length
            let mut name_size = name_buf.len() as u32;
            let mut value_type: u32 = 0;
            
            let result = unsafe {
                RegEnumValueW(
                    key_handle,
                    index,
                    PWSTR::from_raw(name_buf.as_mut_ptr()),
                    &mut name_size,
                    None,
                    Some(&mut value_type as *mut u32 as *mut _),
                    None,
                    None,
                )
            };
            
            if result.is_err() {
                break;
            }
            
            if name_size > 0 {
                let os_string = OsString::from_wide(&name_buf[0..name_size as usize]);
                let name = os_string.to_string_lossy().into_owned();
                
                let value_type = match value_type {
                    x if x == REG_SZ_VALUE => RegistryValueType::String,
                    x if x == REG_EXPAND_SZ_VALUE => RegistryValueType::ExpandString,
                    x if x == REG_BINARY_VALUE => RegistryValueType::Binary,
                    x if x == REG_DWORD_VALUE => RegistryValueType::Dword,
                    x if x == REG_QWORD_VALUE => RegistryValueType::Qword,
                    _ => {
                        // Skip unsupported types
                        index += 1;
                        continue;
                    }
                };
                
                values.push((name, value_type));
            }
            
            index += 1;
        }
        
        unsafe { RegCloseKey(key_handle) };
        
        Ok(values)
    }
}
#[async_trait]
impl Action for RegistryOperationsAction {
    fn get_id(&self) -> Uuid {
        self.id
    }
    
    fn get_name(&self) -> &str {
        "Registry Operations"
    }
    
    fn get_description(&self) -> &str {
        "Performs operations on the Windows Registry"
    }
    
    fn get_supported_event_types(&self) -> Vec<EventType> {
        // Support all event types by returning an empty vector
        Vec::new()
    }
    
    fn get_plugin(&self) -> Arc<dyn Plugin> {
        self.plugin.clone()
    }
    
    async fn configure(&mut self, config: ActionConfig) -> Result<(), Error> {
        // Process configuration arguments
        let mut i = 0;
        while i < config.args.len() {
            let arg = &config.args[i];
            match arg.as_str() {
                "-op" | "--operation" if i + 1 < config.args.len() => {
                    if let Some(op) = RegistryOperation::from_string(&config.args[i + 1]) {
                        self.config.operation = op;
                    } else {
                        return Err(Error::InvalidArgument(format!(
                            "Unknown registry operation: {}", config.args[i + 1]
                        )));
                    }
                    i += 2;
                },
                "-h" | "--hive" if i + 1 < config.args.len() => {
                    if let Some(hive) = RegistryHive::from_string(&config.args[i + 1]) {
                        self.config.hive = hive;
                    } else {
                        return Err(Error::InvalidArgument(format!(
                            "Unknown registry hive: {}", config.args[i + 1]
                        )));
                    }
                    i += 2;
                },
                "-k" | "--key" if i + 1 < config.args.len() => {
                    self.config.key = config.args[i + 1].clone();
                    i += 2;
                },
                "-n" | "--name" if i + 1 < config.args.len() => {
                    self.config.value_name = Some(config.args[i + 1].clone());
                    i += 2;
                },
                "-t" | "--type" if i + 1 < config.args.len() => {
                    if let Some(vtype) = RegistryValueType::from_string(&config.args[i + 1]) {
                        self.config.value_type = Some(vtype);
                    } else {
                        return Err(Error::InvalidArgument(format!(
                            "Unknown registry value type: {}", config.args[i + 1]
                        )));
                    }
                    i += 2;
                },
                "-v" | "--value" if i + 1 < config.args.len() => {
                    let value_type = self.config.value_type.clone()
                        .ok_or_else(|| Error::InvalidArgument(
                            "Value type must be specified before value".to_string()
                        ))?;
                    
                    match value_type {
                        RegistryValueType::String => {
                            self.config.value_data = Some(RegistryValue::String(config.args[i + 1].clone()));
                        },
                        RegistryValueType::ExpandString => {
                            self.config.value_data = Some(RegistryValue::ExpandString(config.args[i + 1].clone()));
                        },
                        RegistryValueType::Dword => {
                            let value = config.args[i + 1].parse::<u32>()
                                .map_err(|_| Error::InvalidArgument(
                                    format!("Invalid DWORD value: {}", config.args[i + 1])
                                ))?;
                            self.config.value_data = Some(RegistryValue::Dword(value));
                        },
                        RegistryValueType::Qword => {
                            let value = config.args[i + 1].parse::<u64>()
                                .map_err(|_| Error::InvalidArgument(
                                    format!("Invalid QWORD value: {}", config.args[i + 1])
                                ))?;
                            self.config.value_data = Some(RegistryValue::Qword(value));
                        },
                        RegistryValueType::Binary => {
                            // Parse hex string as binary
                            let hex = config.args[i + 1].replace(" ", "");
                            if hex.len() % 2 != 0 {
                                return Err(Error::InvalidArgument(
                                    "Binary value must have an even number of hex digits".to_string()
                                ));
                            }
                            
                            let mut bytes = Vec::with_capacity(hex.len() / 2);
                            let mut i = 0;
                            while i < hex.len() {
                                let byte = u8::from_str_radix(&hex[i..i+2], 16)
                                    .map_err(|_| Error::InvalidArgument(
                                        format!("Invalid hex value: {}", &hex[i..i+2])
                                    ))?;
                                bytes.push(byte);
                                i += 2;
                            }
                            
                            self.config.value_data = Some(RegistryValue::Binary(bytes));
                        },
                    }
                    i += 2;
                },
                "--64bit" => {
                    self.config.use_64bit = true;
                    i += 1;
                },
                "--32bit" => {
                    self.config.use_64bit = false;
                    i += 1;
                },
                _ => {
                    i += 1;
                }
            }
        }
        
        Ok(())
    }
    
    async fn execute(&mut self, _event: &dyn Event) -> Result<ActionResult, Error> {
        // Validate the configuration first
        self.validate()?;
        
        // Execute the requested operation
        let result = match self.config.operation {
            RegistryOperation::Read => {
                let value = task::spawn_blocking({
                    let this = self.clone();
                    move || {
                        tokio::runtime::Handle::current().block_on(this.read_value())
                    }
                }).await.map_err(|e| Error::Other(e.to_string()))??;
                
                ActionResult::success().with_data(value)
            },
            RegistryOperation::Write => {
                task::spawn_blocking({
                    let this = self.clone();
                    move || {
                        tokio::runtime::Handle::current().block_on(this.write_value())
                    }
                }).await.map_err(|e| Error::Other(e.to_string()))??;
                
                ActionResult::success()
            },
            RegistryOperation::DeleteKey => {
                task::spawn_blocking({
                    let this = self.clone();
                    move || {
                        tokio::runtime::Handle::current().block_on(this.delete_key())
                    }
                }).await.map_err(|e| Error::Other(e.to_string()))??;
                
                ActionResult::success()
            },
            RegistryOperation::DeleteValue => {
                task::spawn_blocking({
                    let this = self.clone();
                    move || {
                        tokio::runtime::Handle::current().block_on(this.delete_value())
                    }
                }).await.map_err(|e| Error::Other(e.to_string()))??;
                
                ActionResult::success()
            },
            RegistryOperation::KeyExists => {
                let exists = task::spawn_blocking({
                    let this = self.clone();
                    move || {
                        tokio::runtime::Handle::current().block_on(this.key_exists())
                    }
                }).await.map_err(|e| Error::Other(e.to_string()))??;
                
                ActionResult::success().with_data(exists)
            },
            RegistryOperation::ValueExists => {
                let exists = task::spawn_blocking({
                    let this = self.clone();
                    move || {
                        tokio::runtime::Handle::current().block_on(this.value_exists())
                    }
                }).await.map_err(|e| Error::Other(e.to_string()))??;
                
                ActionResult::success().with_data(exists)
            },
            RegistryOperation::EnumerateKeys => {
                let keys = task::spawn_blocking({
                    let this = self.clone();
                    move || {
                        tokio::runtime::Handle::current().block_on(this.enumerate_keys())
                    }
                }).await.map_err(|e| Error::Other(e.to_string()))??;
                
                ActionResult::success().with_data(keys)
            },
            RegistryOperation::EnumerateValues => {
                let values = task::spawn_blocking({
                    let this = self.clone();
                    move || {
                        tokio::runtime::Handle::current().block_on(this.enumerate_values())
                    }
                }).await.map_err(|e| Error::Other(e.to_string()))??;
                
                ActionResult::success().with_data(values)
            },
        };
        
        Ok(result)
    }
    
    fn validate(&self) -> Result<(), Error> {
        // Key must be specified for all operations
        if self.config.key.is_empty() {
            return Err(Error::InvalidArgument("Registry key path must be specified".to_string()));
        }
        
        // Check operation-specific requirements
        match self.config.operation {
            RegistryOperation::Read | RegistryOperation::DeleteValue | RegistryOperation::ValueExists => {
                if self.config.value_name.is_none() {
                    return Err(Error::InvalidArgument("Value name must be specified for this operation".to_string()));
                }
            },
            RegistryOperation::Write => {
                if self.config.value_name.is_none() {
                    return Err(Error::InvalidArgument("Value name must be specified for write operation".to_string()));
                }
                if self.config.value_data.is_none() {
                    return Err(Error::InvalidArgument("Value data must be specified for write operation".to_string()));
                }
                if self.config.value_type.is_none() {
                    return Err(Error::InvalidArgument("Value type must be specified for write operation".to_string()));
                }
            },
            _ => {}
        }
        
        Ok(())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}