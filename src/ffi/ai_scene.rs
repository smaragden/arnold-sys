use ::std::os::raw::c_char;

use super::{ai_map::AtParamValueMap, ai_metadata::AtMetadataStore, ai_universe::AtUniverse};

extern "C" {
    #[doc = "  Load all nodes from a scene file into a specific Arnold universe"]
    #[doc = ""]
    #[doc = "  If the filename is \"-\", it reads data from stdin (assuming ASS format)"]
    #[doc = ""]
    #[doc = "  This example code loads all node types from a scene file:"]
    #[doc = "  \\code"]
    #[doc = "  AtParamValueMap* params = AiParamValueMap();"]
    #[doc = "  AiParamValueMapSetInt(params, AtString(\"mask\"), AI_NODE_ALL);"]
    #[doc = "  AiSceneLoad(universe, \"scene.ass\", params);"]
    #[doc = "  AiParamValueMapDestroy(params);"]
    #[doc = "  \\endcode"]
    #[doc = ""]
    #[doc = "  Supported format specific params:"]
    #[doc = ""]
    #[doc = "  <table>"]
    #[doc = "  <tr><th>Scene format<th colspan=\"3\">Supported load parameters"]
    #[doc = "  <tr><td>ASS<td> **mask**<td> *INTEGER*<td>Only node types matching this mask will be loaded"]
    #[doc = "  <tr><td rowspan=\"2\">USD<td> **mask**<td> *INTEGER*<td>Only node types matching this mask will be loaded"]
    #[doc = "  <tr><td> **frame**<td> *FLOAT*<td>Load a given frame from the USD file"]
    #[doc = "  </table>"]
    #[doc = ""]
    #[doc = "  \\param universe    universe where the nodes will be created (NULL for default universe)"]
    #[doc = "  \\param filename    input filename (extension will be used to determine scene format)"]
    #[doc = "  \\param params      list of arbitrary params which will be interpreted by the specific scene format"]
    #[doc = "  \\return            true if the file was loaded succesfully, false otherwise"]
    pub fn AiSceneLoad(
        universe: *mut AtUniverse,
        filename: *const c_char,
        params: *const AtParamValueMap,
    ) -> bool;
}
extern "C" {
    #[doc = "  Write all nodes in the given universe to a scene file."]
    #[doc = ""]
    #[doc = "  This function can selectively write all nodes in a given universe to a scene file,"]
    #[doc = "  which format will be determined from the filename extension."]
    #[doc = ""]
    #[doc = "  An arbitrary list of attributes can be passed, and these attributes can be used by"]
    #[doc = "  specific file formats. For example, the .ass file format supports \"mask\", \"binary\""]
    #[doc = "  and \"open_procs\" attributes, which are equivalent to the parameters on \\ref AiASSWrite."]
    #[doc = "  Note these attributes might not work on other file formats."]
    #[doc = ""]
    #[doc = "  For example, to write light nodes and camera nodes only, use:"]
    #[doc = "  \\code"]
    #[doc = "  AtParamValueMap* params = AiParamValueMap();"]
    #[doc = "  AiParamValueMapSetInt(params, AtString(\"mask\"), AI_NODE_LIGHT + AI_NODE_CAMERA);"]
    #[doc = "  AiSceneWrite(universe, \"lightsncams.ass\", params);"]
    #[doc = "  AiParamValueMapDestroy(params);"]
    #[doc = "  \\endcode"]
    #[doc = ""]
    #[doc = "  To write all nodes of all types, use:"]
    #[doc = "  \\code"]
    #[doc = "  AtParamValueMap* params = AiParamValueMap();"]
    #[doc = "  AiParamValueMapSetInt(params, AtString(\"mask\"), AI_NODE_ALL);"]
    #[doc = "  AiSceneWrite(universe, \"everything.ass\", params);"]
    #[doc = "  AiParamValueMapDestroy(params);"]
    #[doc = "  \\endcode"]
    #[doc = ""]
    #[doc = "  Supported format specific params:"]
    #[doc = ""]
    #[doc = "  <table>"]
    #[doc = "  <tr><th>Scene format<th colspan=\"3\">Supported write parameters"]
    #[doc = "  <tr><td rowspan=\"3\">ASS<td> **mask**<td> *INTEGER*<td>Only node types matching this mask will be written"]
    #[doc = "  <tr><td> **binary**<td> *BOOLEAN*<td>Allow binary encoding in .ass files"]
    #[doc = "  <tr><td> **open_procs**<td> *BOOLEAN*<td>Procedurals will be expanded before writing"]
    #[doc = "  <tr><td>USD<td> **mask**<td> *INTEGER*<td>Only node types matching this mask will be written"]
    #[doc = "  </table>"]
    #[doc = ""]
    #[doc = "  \\param universe    universe whose contents will be written to the scene file (NULL for default universe)"]
    #[doc = "  \\param filename    output filename (extension will be used to select scene format)"]
    #[doc = "  \\param params      list of arbitrary params which will be interpreted by the specific scene format"]
    #[doc = "  \\param mds         optional metadata store for writing metadata into the file"]
    #[doc = "  \\return            true if the file was written succesfully, false otherwise"]
    pub fn AiSceneWrite(
        universe: *mut AtUniverse,
        filename: *const c_char,
        params: *const AtParamValueMap,
        mds: *const AtMetadataStore,
    ) -> bool;
}
extern "C" {
    #[doc = "  Check if the scene format corresponding to a given filename extension is supported"]
    #[doc = ""]
    #[doc = "  \\param extension   scene file extension to check for support (should start with \".\")"]
    #[doc = "  \\return            true if the format is supported, false otherwise"]
    pub fn AiSceneFormatSupported(extension: *const c_char) -> bool;
}
#[doc = " \\struct AtSceneFormatIterator"]
#[doc = ""]
#[doc = " Allows iterating over the list of supported scene formats."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtSceneFormatIterator {
    _unused: [u8; 0],
}
#[doc = " \\struct AtSceneFormatExtensionIterator"]
#[doc = ""]
#[doc = " Allows iterating over the list of supported extensions for a given scene format."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtSceneFormatExtensionIterator {
    _unused: [u8; 0],
}
#[doc = " \\struct AtSceneFormatData"]
#[doc = ""]
#[doc = " Provides access to information about the scene format."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtSceneFormatData {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " Get new scene format iterator"]
    #[doc = ""]
    #[doc = " \\return New scene format iterator"]
    pub fn AiSceneFormatIterator() -> *mut AtSceneFormatIterator;
}
extern "C" {
    #[doc = " Destroys scene format iterator and releases any allocated memory"]
    #[doc = ""]
    #[doc = " \\param iter  Scene format iterator to destroy"]
    pub fn AiSceneFormatIteratorDestroy(iter: *mut AtSceneFormatIterator);
}
extern "C" {
    #[doc = " Gets the next supported scene format"]
    #[doc = ""]
    #[doc = " \\param iter Scene format iterator"]
    #[doc = " \\return     Information about the next supported scene format"]
    pub fn AiSceneFormatIteratorGetNext(
        iter: *mut AtSceneFormatIterator,
    ) -> *const AtSceneFormatData;
}
extern "C" {
    #[doc = " Check if there are more scene formats to iterate over"]
    #[doc = ""]
    #[doc = " \\param iter Scene format iterator"]
    #[doc = " \\return     true if the iterator reached the last supported scene format"]
    pub fn AiSceneFormatIteratorFinished(iter: *const AtSceneFormatIterator) -> bool;
}
extern "C" {
    #[doc = " Get an iterator over all supported extensions for this scene format"]
    #[doc = ""]
    #[doc = " \\param format_data Scene format data returned by \\c AiSceneFormatIteratorGetNext"]
    #[doc = " \\return            Iterator"]
    pub fn AiSceneFormatGetExtensionIterator(
        format_data: *const AtSceneFormatData,
    ) -> *mut AtSceneFormatExtensionIterator;
}
extern "C" {
    #[doc = " Destroys scene format extension iterator and releases any allocated memory"]
    #[doc = ""]
    #[doc = " \\param iter  Scene format extension iterator to destroy"]
    pub fn AiSceneFormatExtensionIteratorDestroy(iter: *mut AtSceneFormatExtensionIterator);
}
extern "C" {
    #[doc = " Gets the next supported scene format extension"]
    #[doc = ""]
    #[doc = " \\param iter Scene format extension iterator"]
    #[doc = " \\return     Next supported extension"]
    pub fn AiSceneFormatExtensionIteratorGetNext(
        iter: *mut AtSceneFormatExtensionIterator,
    ) -> *const c_char;
}
extern "C" {
    #[doc = " Check if there are more scene formats extensions to iterate over"]
    #[doc = ""]
    #[doc = " \\param iter Scene format extension iterator"]
    #[doc = " \\return     true if the iterator reached the last supported scene format extension"]
    pub fn AiSceneFormatExtensionIteratorFinished(
        iter: *const AtSceneFormatExtensionIterator,
    ) -> bool;
}
extern "C" {
    #[doc = " Get the name of the scene format"]
    #[doc = ""]
    #[doc = " \\param format_data Scene format data returned by \\c AiSceneFormatIteratorGetNext"]
    #[doc = " \\return            Name of the scene format"]
    pub fn AiSceneFormatGetName(format_data: *const AtSceneFormatData) -> *const c_char;
}
extern "C" {
    #[doc = " Get a description of the scene format"]
    #[doc = ""]
    #[doc = " \\param format_data Scene format data returned by \\c AiSceneFormatIteratorGetNext"]
    #[doc = " \\return            Description of the scene format"]
    pub fn AiSceneFormatGetDescription(format_data: *const AtSceneFormatData) -> *const c_char;
}
extern "C" {
    #[doc = " True if the scene format supports reading from file"]
    #[doc = ""]
    #[doc = " \\param format_data Scene format data returned by \\c AiSceneFormatIteratorGetNext"]
    #[doc = " \\return            true if it can be read from file, false otherwise"]
    pub fn AiSceneFormatSupportsReading(format_data: *const AtSceneFormatData) -> bool;
}
extern "C" {
    #[doc = " True if the scene format supports writing to a file"]
    #[doc = ""]
    #[doc = " \\param format_data Scene format data returned by \\c AiSceneFormatIteratorGetNext"]
    #[doc = " \\return            true if it can be written to a file, false otherwise"]
    pub fn AiSceneFormatSupportsWriting(format_data: *const AtSceneFormatData) -> bool;
}
extern "C" {
    #[doc = " Get metadata for the scene format and its optional parameters"]
    #[doc = ""]
    #[doc = " \\param format_data Scene format data returned by \\c AiSceneFormatIteratorGetNext"]
    #[doc = " \\return            Metadata store with all metadata for the scene format and its optional parameters"]
    pub fn AiSceneFormatGetMetadataStore(
        format_data: *const AtSceneFormatData,
    ) -> *const AtMetadataStore;
}
