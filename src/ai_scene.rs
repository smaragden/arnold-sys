use ::std::os::raw::c_char;

use super::{ai_map::AtParamValueMap, ai_metadata::AtMetadataStore, ai_universe::AtUniverse};

extern "C" {
    ///  Load all nodes from a scene file into a specific Arnold universe
    ///
    ///  If the filename is \"-\", it reads data from stdin (assuming ASS format)
    ///
    ///  This example code loads all node types from a scene file:
    ///  ```c
    ///  AtParamValueMap* params = AiParamValueMap();
    ///  AiParamValueMapSetInt(params, AtString(\"mask\"), AI_NODE_ALL);
    ///  AiSceneLoad(universe, \"scene.ass\", params);
    ///  AiParamValueMapDestroy(params);
    ///  ```
    ///
    ///  Supported format specific params:
    ///
    ///  <table>
    ///  <tr><th>Scene format<th colspan=\"3\">Supported load parameters
    ///  <tr><td>ASS<td> **mask**<td> *INTEGER*<td>Only node types matching this mask will be loaded
    ///  <tr><td rowspan=\"2\">USD<td> **mask**<td> *INTEGER*<td>Only node types matching this mask will be loaded
    ///  <tr><td> **frame**<td> *FLOAT*<td>Load a given frame from the USD file
    ///  </table>
    ///
    ///  \\param universe    universe where the nodes will be created (NULL for default universe)
    ///  \\param filename    input filename (extension will be used to determine scene format)
    ///  \\param params      list of arbitrary params which will be interpreted by the specific scene format
    ///  \\return            true if the file was loaded succesfully, false otherwise
    pub fn AiSceneLoad(
        universe: *mut AtUniverse,
        filename: *const c_char,
        params: *const AtParamValueMap,
    ) -> bool;
}
extern "C" {
    ///  Write all nodes in the given universe to a scene file.
    ///
    ///  This function can selectively write all nodes in a given universe to a scene file,
    ///  which format will be determined from the filename extension.
    ///
    ///  An arbitrary list of attributes can be passed, and these attributes can be used by
    ///  specific file formats. For example, the .ass file format supports \"mask\", \"binary\"
    ///  and \"open_procs\" attributes, which are equivalent to the parameters on `AiASSWrite`.
    ///  Note these attributes might not work on other file formats.
    ///
    ///  For example, to write light nodes and camera nodes only, use:
    ///  ```c
    ///  AtParamValueMap* params = AiParamValueMap();
    ///  AiParamValueMapSetInt(params, AtString(\"mask\"), AI_NODE_LIGHT + AI_NODE_CAMERA);
    ///  AiSceneWrite(universe, \"lightsncams.ass\", params);
    ///  AiParamValueMapDestroy(params);
    ///  ```
    ///
    ///  To write all nodes of all types, use:
    ///  ```c
    ///  AtParamValueMap* params = AiParamValueMap();
    ///  AiParamValueMapSetInt(params, AtString(\"mask\"), AI_NODE_ALL);
    ///  AiSceneWrite(universe, \"everything.ass\", params);
    ///  AiParamValueMapDestroy(params);
    ///  ```
    ///
    ///  Supported format specific params:
    ///
    ///  <table>
    ///  <tr><th>Scene format<th colspan=\"3\">Supported write parameters
    ///  <tr><td rowspan=\"3\">ASS<td> **mask**<td> *INTEGER*<td>Only node types matching this mask will be written
    ///  <tr><td> **binary**<td> *BOOLEAN*<td>Allow binary encoding in .ass files
    ///  <tr><td> **open_procs**<td> *BOOLEAN*<td>Procedurals will be expanded before writing
    ///  <tr><td>USD<td> **mask**<td> *INTEGER*<td>Only node types matching this mask will be written
    ///  </table>
    ///
    ///  \\param universe    universe whose contents will be written to the scene file (NULL for default universe)
    ///  \\param filename    output filename (extension will be used to select scene format)
    ///  \\param params      list of arbitrary params which will be interpreted by the specific scene format
    ///  \\param mds         optional metadata store for writing metadata into the file
    ///  \\return            true if the file was written succesfully, false otherwise
    pub fn AiSceneWrite(
        universe: *mut AtUniverse,
        filename: *const c_char,
        params: *const AtParamValueMap,
        mds: *const AtMetadataStore,
    ) -> bool;
}
extern "C" {
    ///  Check if the scene format corresponding to a given filename extension is supported
    ///
    ///  \\param extension   scene file extension to check for support (should start with \".\")
    ///  \\return            true if the format is supported, false otherwise
    pub fn AiSceneFormatSupported(extension: *const c_char) -> bool;
}
/// \\struct AtSceneFormatIterator
///
/// Allows iterating over the list of supported scene formats.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtSceneFormatIterator {
    _unused: [u8; 0],
}
/// \\struct AtSceneFormatExtensionIterator
///
/// Allows iterating over the list of supported extensions for a given scene format.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtSceneFormatExtensionIterator {
    _unused: [u8; 0],
}
/// \\struct AtSceneFormatData
///
/// Provides access to information about the scene format.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtSceneFormatData {
    _unused: [u8; 0],
}
extern "C" {
    /// Get new scene format iterator
    ///
    /// \\return New scene format iterator
    pub fn AiSceneFormatIterator() -> *mut AtSceneFormatIterator;
}
extern "C" {
    /// Destroys scene format iterator and releases any allocated memory
    ///
    /// \\param iter  Scene format iterator to destroy
    pub fn AiSceneFormatIteratorDestroy(iter: *mut AtSceneFormatIterator);
}
extern "C" {
    /// Gets the next supported scene format
    ///
    /// \\param iter Scene format iterator
    /// \\return     Information about the next supported scene format
    pub fn AiSceneFormatIteratorGetNext(
        iter: *mut AtSceneFormatIterator,
    ) -> *const AtSceneFormatData;
}
extern "C" {
    /// Check if there are more scene formats to iterate over
    ///
    /// \\param iter Scene format iterator
    /// \\return     true if the iterator reached the last supported scene format
    pub fn AiSceneFormatIteratorFinished(iter: *const AtSceneFormatIterator) -> bool;
}
extern "C" {
    /// Get an iterator over all supported extensions for this scene format
    ///
    /// \\param format_data Scene format data returned by \\c AiSceneFormatIteratorGetNext
    /// \\return            Iterator
    pub fn AiSceneFormatGetExtensionIterator(
        format_data: *const AtSceneFormatData,
    ) -> *mut AtSceneFormatExtensionIterator;
}
extern "C" {
    /// Destroys scene format extension iterator and releases any allocated memory
    ///
    /// \\param iter  Scene format extension iterator to destroy
    pub fn AiSceneFormatExtensionIteratorDestroy(iter: *mut AtSceneFormatExtensionIterator);
}
extern "C" {
    /// Gets the next supported scene format extension
    ///
    /// \\param iter Scene format extension iterator
    /// \\return     Next supported extension
    pub fn AiSceneFormatExtensionIteratorGetNext(
        iter: *mut AtSceneFormatExtensionIterator,
    ) -> *const c_char;
}
extern "C" {
    /// Check if there are more scene formats extensions to iterate over
    ///
    /// \\param iter Scene format extension iterator
    /// \\return     true if the iterator reached the last supported scene format extension
    pub fn AiSceneFormatExtensionIteratorFinished(
        iter: *const AtSceneFormatExtensionIterator,
    ) -> bool;
}
extern "C" {
    /// Get the name of the scene format
    ///
    /// \\param format_data Scene format data returned by \\c AiSceneFormatIteratorGetNext
    /// \\return            Name of the scene format
    pub fn AiSceneFormatGetName(format_data: *const AtSceneFormatData) -> *const c_char;
}
extern "C" {
    /// Get a description of the scene format
    ///
    /// \\param format_data Scene format data returned by \\c AiSceneFormatIteratorGetNext
    /// \\return            Description of the scene format
    pub fn AiSceneFormatGetDescription(format_data: *const AtSceneFormatData) -> *const c_char;
}
extern "C" {
    /// True if the scene format supports reading from file
    ///
    /// \\param format_data Scene format data returned by \\c AiSceneFormatIteratorGetNext
    /// \\return            true if it can be read from file, false otherwise
    pub fn AiSceneFormatSupportsReading(format_data: *const AtSceneFormatData) -> bool;
}
extern "C" {
    /// True if the scene format supports writing to a file
    ///
    /// \\param format_data Scene format data returned by \\c AiSceneFormatIteratorGetNext
    /// \\return            true if it can be written to a file, false otherwise
    pub fn AiSceneFormatSupportsWriting(format_data: *const AtSceneFormatData) -> bool;
}
extern "C" {
    /// Get metadata for the scene format and its optional parameters
    ///
    /// \\param format_data Scene format data returned by \\c AiSceneFormatIteratorGetNext
    /// \\return            Metadata store with all metadata for the scene format and its optional parameters
    pub fn AiSceneFormatGetMetadataStore(
        format_data: *const AtSceneFormatData,
    ) -> *const AtMetadataStore;
}
