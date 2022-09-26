use ::std::os::raw::c_char;

use super::{ai_map::AtParamValueMap, ai_string::AtString};

extern "C" {
    #[doc = " Before ADP collects any data, we need to let the user know that Arnold data"]
    #[doc = " is being sent to adsk. This happens one time and then they never need to be"]
    #[doc = " shown this message again."]
    #[doc = ""]
    #[doc = " If this returns true, no need to do anything. If false, then do something like:"]
    #[doc = " \\code"]
    #[doc = "if (!AiADPIntroWasShown())"]
    #[doc = "{"]
    #[doc = "AtString title, introduction, learnMoreText, learnMoreURL, privacyText, privacyURL;"]
    #[doc = "AiADPDialogStringsIntro(title, introduction, learnMoreText, learnMoreURL,"]
    #[doc = "privacyText, privacyURL);"]
    #[doc = ""]
    #[doc = "dialog_title(title);"]
    #[doc = "dialog_text_body(introduction + \" or through the <your plugin> help menu/Autodesk Analytics menu option\"); // tune this for your plugin"]
    #[doc = "dialog_url1(learnMoreText, learnMoreURL); // should be a clickable link"]
    #[doc = "dialog_url2(privacyText, privacyURL);     // should be a clickable link"]
    #[doc = "gui_dialog_box_with_OK_button();"]
    #[doc = ""]
    #[doc = "AiADPSetIntroShown();"]
    #[doc = "}"]
    #[doc = " \\endcode"]
    #[doc = ""]
    #[doc = " \\return true if the user was previously shown the Autodesk Analytics program data involvement agreement"]
    pub fn AiADPIntroWasShown() -> bool;
}
extern "C" {
    #[doc = " Call this after displaying the intro to user so we don't keep displaying intro message."]
    pub fn AiADPSetIntroShown();
}
extern "C" {
    #[doc = " Set to true if user agreed to optin and false if opted out."]
    pub fn AiADPSetOptedIn(wants_optin: bool);
}
extern "C" {
    #[doc = " \\return true if user agreed to opt in and false if opted out."]
    pub fn AiADPIsOptedIn() -> bool;
}
extern "C" {
    #[doc = " Strings and URLs to display to user when displaying ADP related dialog boxes."]
    #[doc = ""]
    #[doc = " There are two dialog boxes:"]
    #[doc = ""]
    #[doc = " INTRO is used when the user has never before been informed about the"]
    #[doc = " Autodesk Analytics program. This will include an introduction_intro string"]
    #[doc = " with instructions for how to change the opt-in/out settings using kick,"]
    #[doc = " since all plugins come with kick. If your plugin has its own way for"]
    #[doc = " changing the optin settings, and it probably should, then instructions for"]
    #[doc = " how to do so should go in the custom_optin_command argument and the"]
    #[doc = " introduction_intro string will then include that as well. For instance:"]
    #[doc = " custom_optin_command=\"in the C4DtoA > Help menu\".  Otherwise,"]
    #[doc = " custom_optin_command can be NULL."]
    #[doc = ""]
    #[doc = " CHANGE is used after the user has been shown the intro dialog and wants to"]
    #[doc = " change opt-in status. Note that this mode has an additional \"checkbox\""]
    #[doc = " string and instead of \"introduction_intro\" should use the"]
    #[doc = " \"introduction_change\" string."]
    #[doc = ""]
    #[doc = " The other strings are the \"title\", \"learnMoreURL\" and associated"]
    #[doc = " \"learnMoreText\" link, and the \"privacyURL\" and associated \"privacyText\" link."]
    pub fn AiADPDialogStrings(
        strings: *mut AtParamValueMap,
        custom_optin_command: *const c_char,
    );
}
extern "C" {
    #[doc = " Provide additional data to the product analytics, such as information about the"]
    #[doc = " client that created the Arnold render session. This has to be called before"]
    #[doc = " data is collected, for instance, before \\ref AiRenderBegin."]
    #[doc = ""]
    #[doc = " This example code sets a plugin name:"]
    #[doc = " \\code"]
    #[doc = " AiADPAddProductMetadata(AI_ADP_PLUGINNAME, AtString(\"HtoA\"));"]
    #[doc = " \\endcode"]
    #[doc = ""]
    #[doc = " @param name   Name for the additional product analytics data entry"]
    #[doc = " @param value  Value for the additional product analytics data entry"]
    pub fn AiADPAddProductMetadata(name: AtString, value: AtString);
}
