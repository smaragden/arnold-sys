use ::std::os::raw::{c_char, c_int};

use super::{ai_metadata::AtMetadataStore, ai_universe::AtUniverse};

extern "C" {
    /// \\defgroup ai_dotass ASS File API
    ///
    ///  Loading and writing of .ass (Arnold Scene Source) scene files.
    ///
    ///  This is deprecated in favor of ai_scene.h and its AiScene* methods.
    ///
    ///  \\details
    ///  Arnold has built-in support for writing scene data to a file and later
    ///  reading the file in. Although not required, the extension for these files
    ///  is usually .ass, which stands for <b>A</b>rnold <b>S</b>cene <b>S</b>ource.
    ///  The file format is a straightforward mapping from Arnold `AtNode`'s
    ///  to human-readable ASCII. For example, a sphere node is written as:
    ///  ```c
    ///  sphere          // this is the node class
    ///  {               // any number of param/value pairs enclosed in curly braces
    ///   center 0 0 0   //  parameter \"center\" of type AtVector is set to value (0,0,0)
    ///   radius 2.0     //  parameter \"radius\" of type float is set to value 2.0
    ///  }               // end of node block
    ///  ```
    ///
    /// \\{
    pub fn AiASSWrite(
        universe: *mut AtUniverse,
        filename: *const c_char,
        mask: c_int,
        open_procs: bool,
        binary: bool,
    ) -> c_int;
}
extern "C" {
    pub fn AiASSWriteWithMetadata(
        universe: *mut AtUniverse,
        filename: *const c_char,
        mask: c_int,
        open_procs: bool,
        binary: bool,
        mds: *const AtMetadataStore,
    ) -> c_int;
}
extern "C" {
    pub fn AiASSLoad(universe: *mut AtUniverse, filename: *const c_char, mask: c_int) -> c_int;
}
