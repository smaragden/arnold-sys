use ::std::os::raw::c_char;

use super::{ai_map::AtParamValueMap, ai_string::AtString};

extern "C" {
    /// Before ADP collects any data, we need to let the user know that Arnold data
    /// is being sent to adsk. This happens one time and then they never need to be
    /// shown this message again.
    ///
    /// If this returns true, no need to do anything. If false, then do something like:
    /// ```c
    ///if (!AiADPIntroWasShown())
    ///{
    ///AtString title, introduction, learnMoreText, learnMoreURL, privacyText, privacyURL;
    ///AiADPDialogStringsIntro(title, introduction, learnMoreText, learnMoreURL,
    ///privacyText, privacyURL);
    ///
    ///dialog_title(title);
    ///dialog_text_body(introduction + \" or through the <your plugin> help menu/Autodesk Analytics menu option\"); // tune this for your plugin
    ///dialog_url1(learnMoreText, learnMoreURL); // should be a clickable link
    ///dialog_url2(privacyText, privacyURL);     // should be a clickable link
    ///gui_dialog_box_with_OK_button();
    ///
    ///AiADPSetIntroShown();
    ///}
    /// ```
    ///
    /// \\return true if the user was previously shown the Autodesk Analytics program data involvement agreement
    pub fn AiADPIntroWasShown() -> bool;

    /// Call this after displaying the intro to user so we don't keep displaying intro message.
    pub fn AiADPSetIntroShown();

    /// Set to true if user agreed to optin and false if opted out.
    pub fn AiADPSetOptedIn(wants_optin: bool);

    /// \\return true if user agreed to opt in and false if opted out.
    pub fn AiADPIsOptedIn() -> bool;

    /// Strings and URLs to display to user when displaying ADP related dialog boxes.
    ///
    /// There are two dialog boxes:
    ///
    /// INTRO is used when the user has never before been informed about the
    /// Autodesk Analytics program. This will include an introduction_intro string
    /// with instructions for how to change the opt-in/out settings using kick,
    /// since all plugins come with kick. If your plugin has its own way for
    /// changing the optin settings, and it probably should, then instructions for
    /// how to do so should go in the custom_optin_command argument and the
    /// introduction_intro string will then include that as well. For instance:
    /// custom_optin_command=\"in the C4DtoA > Help menu\".  Otherwise,
    /// custom_optin_command can be NULL.
    ///
    /// CHANGE is used after the user has been shown the intro dialog and wants to
    /// change opt-in status. Note that this mode has an additional \"checkbox\"
    /// string and instead of \"introduction_intro\" should use the
    /// \"introduction_change\" string.
    ///
    /// The other strings are the \"title\", \"learnMoreURL\" and associated
    /// \"learnMoreText\" link, and the \"privacyURL\" and associated \"privacyText\" link.
    pub fn AiADPDialogStrings(strings: *mut AtParamValueMap, custom_optin_command: *const c_char);

    /// Provide additional data to the product analytics, such as information about the
    /// client that created the Arnold render session. This has to be called before
    /// data is collected, for instance, before `AiRenderBegin`.
    ///
    /// This example code sets a plugin name:
    /// ```c
    /// AiADPAddProductMetadata(AI_ADP_PLUGINNAME, AtString(\"HtoA\"));
    /// ```
    ///
    /// @param name   Name for the additional product analytics data entry
    /// @param value  Value for the additional product analytics data entry
    pub fn AiADPAddProductMetadata(name: AtString, value: AtString);
}
