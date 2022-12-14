use ::std::os::raw::{c_char, c_int};

use super::{
    ai_color::{AtRGB, AtRGBA},
    ai_node_entry::{AtMetaDataIterator, AtNodeEntry},
    ai_string::AtString,
    ai_vector::{AtVector, AtVector2},
};

/// \\struct AtMetadataStore
///
/// This structure holds a generic list of metadata items, each of which could
/// optionally be associated to a specific parameter (for node metadata). The actual
/// contents of this struct are private.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtMetadataStore {
    _unused: [u8; 0],
}
extern "C" {
    pub fn AiMetaDataSetBool(
        nentry: *mut AtNodeEntry,
        param: AtString,
        name: AtString,
        value: bool,
    );

    pub fn AiMetaDataSetInt(
        nentry: *mut AtNodeEntry,
        param: AtString,
        name: AtString,
        value: c_int,
    );

    pub fn AiMetaDataSetFlt(nentry: *mut AtNodeEntry, param: AtString, name: AtString, value: f32);

    pub fn AiMetaDataSetRGB(
        nentry: *mut AtNodeEntry,
        param: AtString,
        name: AtString,
        value: AtRGB,
    );

    pub fn AiMetaDataSetRGBA(
        nentry: *mut AtNodeEntry,
        param: AtString,
        name: AtString,
        value: AtRGBA,
    );

    pub fn AiMetaDataSetVec(
        nentry: *mut AtNodeEntry,
        param: AtString,
        name: AtString,
        value: AtVector,
    );

    pub fn AiMetaDataSetVec2(
        nentry: *mut AtNodeEntry,
        param: AtString,
        name: AtString,
        value: AtVector2,
    );

    pub fn AiMetaDataSetStr(
        nentry: *mut AtNodeEntry,
        param: AtString,
        name: AtString,
        value: AtString,
    );

    pub fn AiMetaDataGetBool(
        nentry: *const AtNodeEntry,
        param: AtString,
        name: AtString,
        value: *mut bool,
    ) -> bool;

    pub fn AiMetaDataGetInt(
        nentry: *const AtNodeEntry,
        param: AtString,
        name: AtString,
        value: *mut c_int,
    ) -> bool;

    pub fn AiMetaDataGetFlt(
        nentry: *const AtNodeEntry,
        param: AtString,
        name: AtString,
        value: *mut f32,
    ) -> bool;

    pub fn AiMetaDataGetRGB(
        nentry: *const AtNodeEntry,
        param: AtString,
        name: AtString,
        value: *mut AtRGB,
    ) -> bool;

    pub fn AiMetaDataGetRGBA(
        nentry: *const AtNodeEntry,
        param: AtString,
        name: AtString,
        value: *mut AtRGBA,
    ) -> bool;

    pub fn AiMetaDataGetVec(
        nentry: *const AtNodeEntry,
        param: AtString,
        name: AtString,
        value: *mut AtVector,
    ) -> bool;

    pub fn AiMetaDataGetVec2(
        nentry: *const AtNodeEntry,
        param: AtString,
        name: AtString,
        value: *mut AtVector2,
    ) -> bool;

    pub fn AiMetaDataGetStr(
        nentry: *const AtNodeEntry,
        param: AtString,
        name: AtString,
        value: *mut AtString,
    ) -> bool;

    pub fn AiMetadataStoreSetBool(mds: *mut AtMetadataStore, name: AtString, value: bool);

    pub fn AiMetadataStoreParamSetBool(
        mds: *mut AtMetadataStore,
        param: AtString,
        name: AtString,
        value: bool,
    );

    pub fn AiMetadataStoreSetInt(mds: *mut AtMetadataStore, name: AtString, value: c_int);

    pub fn AiMetadataStoreParamSetInt(
        mds: *mut AtMetadataStore,
        param: AtString,
        name: AtString,
        value: c_int,
    );

    pub fn AiMetadataStoreSetFlt(mds: *mut AtMetadataStore, name: AtString, value: f32);

    pub fn AiMetadataStoreParamSetFlt(
        mds: *mut AtMetadataStore,
        param: AtString,
        name: AtString,
        value: f32,
    );

    pub fn AiMetadataStoreSetRGB(mds: *mut AtMetadataStore, name: AtString, value: AtRGB);

    pub fn AiMetadataStoreParamSetRGB(
        mds: *mut AtMetadataStore,
        param: AtString,
        name: AtString,
        value: AtRGB,
    );

    pub fn AiMetadataStoreSetVec(mds: *mut AtMetadataStore, name: AtString, value: AtVector);

    pub fn AiMetadataStoreParamSetVec(
        mds: *mut AtMetadataStore,
        param: AtString,
        name: AtString,
        value: AtVector,
    );

    pub fn AiMetadataStoreSetVec2(mds: *mut AtMetadataStore, name: AtString, value: AtVector2);

    pub fn AiMetadataStoreParamSetVec2(
        mds: *mut AtMetadataStore,
        param: AtString,
        name: AtString,
        value: AtVector2,
    );

    pub fn AiMetadataStoreSetStr(mds: *mut AtMetadataStore, name: AtString, value: AtString);

    pub fn AiMetadataStoreParamSetStr(
        mds: *mut AtMetadataStore,
        param: AtString,
        name: AtString,
        value: AtString,
    );

    pub fn AiMetadataStoreGetBool(
        mds: *const AtMetadataStore,
        name: AtString,
        value: *mut bool,
    ) -> bool;

    pub fn AiMetadataStoreParamGetBool(
        mds: *const AtMetadataStore,
        param: AtString,
        name: AtString,
        value: *mut bool,
    ) -> bool;

    pub fn AiMetadataStoreGetInt(
        mds: *const AtMetadataStore,
        name: AtString,
        value: *mut c_int,
    ) -> bool;

    pub fn AiMetadataStoreParamGetInt(
        mds: *const AtMetadataStore,
        param: AtString,
        name: AtString,
        value: *mut c_int,
    ) -> bool;

    pub fn AiMetadataStoreGetFlt(
        mds: *const AtMetadataStore,
        name: AtString,
        value: *mut f32,
    ) -> bool;

    pub fn AiMetadataStoreParamGetFlt(
        mds: *const AtMetadataStore,
        param: AtString,
        name: AtString,
        value: *mut f32,
    ) -> bool;

    pub fn AiMetadataStoreGetRGB(
        mds: *const AtMetadataStore,
        name: AtString,
        value: *mut AtRGB,
    ) -> bool;

    pub fn AiMetadataStoreParamGetRGB(
        mds: *const AtMetadataStore,
        param: AtString,
        name: AtString,
        value: *mut AtRGB,
    ) -> bool;

    pub fn AiMetadataStoreGetVec(
        mds: *const AtMetadataStore,
        name: AtString,
        value: *mut AtVector,
    ) -> bool;

    pub fn AiMetadataStoreParamGetVec(
        mds: *const AtMetadataStore,
        param: AtString,
        name: AtString,
        value: *mut AtVector,
    ) -> bool;

    pub fn AiMetadataStoreGetVec2(
        mds: *const AtMetadataStore,
        name: AtString,
        value: *mut AtVector2,
    ) -> bool;

    pub fn AiMetadataStoreParamGetVec2(
        mds: *const AtMetadataStore,
        param: AtString,
        name: AtString,
        value: *mut AtVector2,
    ) -> bool;

    pub fn AiMetadataStoreGetStr(
        mds: *const AtMetadataStore,
        name: AtString,
        value: *mut AtString,
    ) -> bool;

    pub fn AiMetadataStoreParamGetStr(
        mds: *const AtMetadataStore,
        param: AtString,
        name: AtString,
        value: *mut AtString,
    ) -> bool;

    /// Creates a new metadata store.
    ///
    /// \\return  new metadata store object
    pub fn AiMetadataStore() -> *mut AtMetadataStore;

    /// Destroys a metadata store object.
    ///
    /// \\param mds     metadata store object to be destroyed
    pub fn AiMetadataStoreDestroy(mds: *mut AtMetadataStore);

    /// Load embedded metadata from an .ass file into a metadata store.
    ///
    /// \\param mds     metadata store object where embedded metadata will be loaded
    /// \\param file    filename of the .ass file with the embedded metadata to load
    pub fn AiMetadataStoreLoadFromASS(mds: *mut AtMetadataStore, file: *const c_char) -> bool;

    /// Creates a new metadata iterator that traverses all global metadata.
    ///
    /// \\param mds     metadata store object to get an iterator for
    /// \\return        an iterator over all global metadata in a metadata store
    pub fn AiMetadataStoreGetIterator(mds: *const AtMetadataStore) -> *mut AtMetaDataIterator;

    /// Creates a new metadata iterator pointing at the first matching entry.
    ///
    /// \\param mds        metadata store object to get an iterator for
    /// \\param param      request metadata for a specific param (or global metadata if param is NULL)
    /// \\param recursive  if true and param is NULL, it will traverse both global metadata and param metadata for all params
    /// \\return           an iterator over all metadata in a metadata store
    pub fn AiMetadataStoreGetIteratorRecursive(
        mds: *const AtMetadataStore,
        param: *const c_char,
        recursive: bool,
    ) -> *mut AtMetaDataIterator;

    /// Load a metadata file.
    ///
    /// Metadata items loaded from this file will be attached to existing
    /// node entries and their parameters, as especified by the
    /// [.mtd file format](https://docs.arnoldrenderer.com/x/kQNEB\)
    ///
    /// Usage:
    /// ```c
    /// const char* metadata_file = \"my_metadata_file.mtd\";
    /// bool success = AiMetaDataLoadFile(metadata_file)
    /// if (!success)
    ///    printf(\"\\nError loading metadata file %s\", metadata_file);
    /// ```
    ///
    /// \\param filename    the full path of the metadata file to load
    /// \\return            true when the file could be read succesfully
    pub fn AiMetaDataLoadFile(filename: *const c_char) -> bool;
}
