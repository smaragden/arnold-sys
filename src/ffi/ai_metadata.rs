use ::std::os::raw::{c_int, c_char};

use super::{
    ai_color::{AtRGB, AtRGBA},
    ai_node_entry::{AtMetaDataIterator, AtNodeEntry},
    ai_string::AtString,
    ai_vector::{AtVector, AtVector2},
};

#[doc = " \\struct AtMetadataStore"]
#[doc = ""]
#[doc = " This structure holds a generic list of metadata items, each of which could"]
#[doc = " optionally be associated to a specific parameter (for node metadata). The actual"]
#[doc = " contents of this struct are private."]
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
}
extern "C" {
    pub fn AiMetaDataSetInt(
        nentry: *mut AtNodeEntry,
        param: AtString,
        name: AtString,
        value: c_int,
    );
}
extern "C" {
    pub fn AiMetaDataSetFlt(nentry: *mut AtNodeEntry, param: AtString, name: AtString, value: f32);
}
extern "C" {
    pub fn AiMetaDataSetRGB(
        nentry: *mut AtNodeEntry,
        param: AtString,
        name: AtString,
        value: AtRGB,
    );
}
extern "C" {
    pub fn AiMetaDataSetRGBA(
        nentry: *mut AtNodeEntry,
        param: AtString,
        name: AtString,
        value: AtRGBA,
    );
}
extern "C" {
    pub fn AiMetaDataSetVec(
        nentry: *mut AtNodeEntry,
        param: AtString,
        name: AtString,
        value: AtVector,
    );
}
extern "C" {
    pub fn AiMetaDataSetVec2(
        nentry: *mut AtNodeEntry,
        param: AtString,
        name: AtString,
        value: AtVector2,
    );
}
extern "C" {
    pub fn AiMetaDataSetStr(
        nentry: *mut AtNodeEntry,
        param: AtString,
        name: AtString,
        value: AtString,
    );
}
extern "C" {
    pub fn AiMetaDataGetBool(
        nentry: *const AtNodeEntry,
        param: AtString,
        name: AtString,
        value: *mut bool,
    ) -> bool;
}
extern "C" {
    pub fn AiMetaDataGetInt(
        nentry: *const AtNodeEntry,
        param: AtString,
        name: AtString,
        value: *mut c_int,
    ) -> bool;
}
extern "C" {
    pub fn AiMetaDataGetFlt(
        nentry: *const AtNodeEntry,
        param: AtString,
        name: AtString,
        value: *mut f32,
    ) -> bool;
}
extern "C" {
    pub fn AiMetaDataGetRGB(
        nentry: *const AtNodeEntry,
        param: AtString,
        name: AtString,
        value: *mut AtRGB,
    ) -> bool;
}
extern "C" {
    pub fn AiMetaDataGetRGBA(
        nentry: *const AtNodeEntry,
        param: AtString,
        name: AtString,
        value: *mut AtRGBA,
    ) -> bool;
}
extern "C" {
    pub fn AiMetaDataGetVec(
        nentry: *const AtNodeEntry,
        param: AtString,
        name: AtString,
        value: *mut AtVector,
    ) -> bool;
}
extern "C" {
    pub fn AiMetaDataGetVec2(
        nentry: *const AtNodeEntry,
        param: AtString,
        name: AtString,
        value: *mut AtVector2,
    ) -> bool;
}
extern "C" {
    pub fn AiMetaDataGetStr(
        nentry: *const AtNodeEntry,
        param: AtString,
        name: AtString,
        value: *mut AtString,
    ) -> bool;
}
extern "C" {
    pub fn AiMetadataStoreSetBool(mds: *mut AtMetadataStore, name: AtString, value: bool);
}
extern "C" {
    pub fn AiMetadataStoreParamSetBool(
        mds: *mut AtMetadataStore,
        param: AtString,
        name: AtString,
        value: bool,
    );
}
extern "C" {
    pub fn AiMetadataStoreSetInt(
        mds: *mut AtMetadataStore,
        name: AtString,
        value: c_int,
    );
}
extern "C" {
    pub fn AiMetadataStoreParamSetInt(
        mds: *mut AtMetadataStore,
        param: AtString,
        name: AtString,
        value: c_int,
    );
}
extern "C" {
    pub fn AiMetadataStoreSetFlt(mds: *mut AtMetadataStore, name: AtString, value: f32);
}
extern "C" {
    pub fn AiMetadataStoreParamSetFlt(
        mds: *mut AtMetadataStore,
        param: AtString,
        name: AtString,
        value: f32,
    );
}
extern "C" {
    pub fn AiMetadataStoreSetRGB(mds: *mut AtMetadataStore, name: AtString, value: AtRGB);
}
extern "C" {
    pub fn AiMetadataStoreParamSetRGB(
        mds: *mut AtMetadataStore,
        param: AtString,
        name: AtString,
        value: AtRGB,
    );
}
extern "C" {
    pub fn AiMetadataStoreSetVec(mds: *mut AtMetadataStore, name: AtString, value: AtVector);
}
extern "C" {
    pub fn AiMetadataStoreParamSetVec(
        mds: *mut AtMetadataStore,
        param: AtString,
        name: AtString,
        value: AtVector,
    );
}
extern "C" {
    pub fn AiMetadataStoreSetVec2(mds: *mut AtMetadataStore, name: AtString, value: AtVector2);
}
extern "C" {
    pub fn AiMetadataStoreParamSetVec2(
        mds: *mut AtMetadataStore,
        param: AtString,
        name: AtString,
        value: AtVector2,
    );
}
extern "C" {
    pub fn AiMetadataStoreSetStr(mds: *mut AtMetadataStore, name: AtString, value: AtString);
}
extern "C" {
    pub fn AiMetadataStoreParamSetStr(
        mds: *mut AtMetadataStore,
        param: AtString,
        name: AtString,
        value: AtString,
    );
}
extern "C" {
    pub fn AiMetadataStoreGetBool(
        mds: *const AtMetadataStore,
        name: AtString,
        value: *mut bool,
    ) -> bool;
}
extern "C" {
    pub fn AiMetadataStoreParamGetBool(
        mds: *const AtMetadataStore,
        param: AtString,
        name: AtString,
        value: *mut bool,
    ) -> bool;
}
extern "C" {
    pub fn AiMetadataStoreGetInt(
        mds: *const AtMetadataStore,
        name: AtString,
        value: *mut c_int,
    ) -> bool;
}
extern "C" {
    pub fn AiMetadataStoreParamGetInt(
        mds: *const AtMetadataStore,
        param: AtString,
        name: AtString,
        value: *mut c_int,
    ) -> bool;
}
extern "C" {
    pub fn AiMetadataStoreGetFlt(
        mds: *const AtMetadataStore,
        name: AtString,
        value: *mut f32,
    ) -> bool;
}
extern "C" {
    pub fn AiMetadataStoreParamGetFlt(
        mds: *const AtMetadataStore,
        param: AtString,
        name: AtString,
        value: *mut f32,
    ) -> bool;
}
extern "C" {
    pub fn AiMetadataStoreGetRGB(
        mds: *const AtMetadataStore,
        name: AtString,
        value: *mut AtRGB,
    ) -> bool;
}
extern "C" {
    pub fn AiMetadataStoreParamGetRGB(
        mds: *const AtMetadataStore,
        param: AtString,
        name: AtString,
        value: *mut AtRGB,
    ) -> bool;
}
extern "C" {
    pub fn AiMetadataStoreGetVec(
        mds: *const AtMetadataStore,
        name: AtString,
        value: *mut AtVector,
    ) -> bool;
}
extern "C" {
    pub fn AiMetadataStoreParamGetVec(
        mds: *const AtMetadataStore,
        param: AtString,
        name: AtString,
        value: *mut AtVector,
    ) -> bool;
}
extern "C" {
    pub fn AiMetadataStoreGetVec2(
        mds: *const AtMetadataStore,
        name: AtString,
        value: *mut AtVector2,
    ) -> bool;
}
extern "C" {
    pub fn AiMetadataStoreParamGetVec2(
        mds: *const AtMetadataStore,
        param: AtString,
        name: AtString,
        value: *mut AtVector2,
    ) -> bool;
}
extern "C" {
    pub fn AiMetadataStoreGetStr(
        mds: *const AtMetadataStore,
        name: AtString,
        value: *mut AtString,
    ) -> bool;
}
extern "C" {
    pub fn AiMetadataStoreParamGetStr(
        mds: *const AtMetadataStore,
        param: AtString,
        name: AtString,
        value: *mut AtString,
    ) -> bool;
}
extern "C" {
    #[doc = " Creates a new metadata store."]
    #[doc = ""]
    #[doc = " \\return  new metadata store object"]
    pub fn AiMetadataStore() -> *mut AtMetadataStore;
}
extern "C" {
    #[doc = " Destroys a metadata store object."]
    #[doc = ""]
    #[doc = " \\param mds     metadata store object to be destroyed"]
    pub fn AiMetadataStoreDestroy(mds: *mut AtMetadataStore);
}
extern "C" {
    #[doc = " Load embedded metadata from an .ass file into a metadata store."]
    #[doc = ""]
    #[doc = " \\param mds     metadata store object where embedded metadata will be loaded"]
    #[doc = " \\param file    filename of the .ass file with the embedded metadata to load"]
    pub fn AiMetadataStoreLoadFromASS(
        mds: *mut AtMetadataStore,
        file: *const c_char,
    ) -> bool;
}
extern "C" {
    #[doc = " Creates a new metadata iterator that traverses all global metadata."]
    #[doc = ""]
    #[doc = " \\param mds     metadata store object to get an iterator for"]
    #[doc = " \\return        an iterator over all global metadata in a metadata store"]
    pub fn AiMetadataStoreGetIterator(mds: *const AtMetadataStore) -> *mut AtMetaDataIterator;
}
extern "C" {
    #[doc = " Creates a new metadata iterator pointing at the first matching entry."]
    #[doc = ""]
    #[doc = " \\param mds        metadata store object to get an iterator for"]
    #[doc = " \\param param      request metadata for a specific param (or global metadata if param is NULL)"]
    #[doc = " \\param recursive  if true and param is NULL, it will traverse both global metadata and param metadata for all params"]
    #[doc = " \\return           an iterator over all metadata in a metadata store"]
    pub fn AiMetadataStoreGetIteratorRecursive(
        mds: *const AtMetadataStore,
        param: *const c_char,
        recursive: bool,
    ) -> *mut AtMetaDataIterator;
}
extern "C" {
    #[doc = " Load a metadata file."]
    #[doc = ""]
    #[doc = " Metadata items loaded from this file will be attached to existing"]
    #[doc = " node entries and their parameters, as especified by the"]
    #[doc = " <a href=\"https://docs.arnoldrenderer.com/x/kQNEB\">.mtd file format</a>"]
    #[doc = ""]
    #[doc = " Usage:"]
    #[doc = " \\code"]
    #[doc = " const char* metadata_file = \"my_metadata_file.mtd\";"]
    #[doc = " bool success = AiMetaDataLoadFile(metadata_file)"]
    #[doc = " if (!success)"]
    #[doc = "    printf(\"\\nError loading metadata file %s\", metadata_file);"]
    #[doc = " \\endcode"]
    #[doc = ""]
    #[doc = " \\param filename    the full path of the metadata file to load"]
    #[doc = " \\return            true when the file could be read succesfully"]
    pub fn AiMetaDataLoadFile(filename: *const c_char) -> bool;
}
