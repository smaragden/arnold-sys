use arnold_rs::ai_array::{AiArrayAllocate, AiArraySetStr};
use arnold_rs::ai_node_entry::NodeTypes;
use arnold_rs::ai_nodes::{AiNodeLink, AiNodeSetArray, AiNodeSetInt, AiNodeSetPtr, AiNodeSetRGB};
use arnold_rs::ai_params::AI_TYPE_STRING;
use arnold_rs::ai_render::{AiRender, AiRenderSession, AtRenderMode};
use arnold_rs::ai_universe::AiUniverseGetOptions;
use arnold_rs::{
    ai_array::AiArray,
    ai_dotass::AiASSWrite,
    ai_msg::{AiMsgSetConsoleFlags, AiMsgSetLogFileName, LogFlags},
    ai_nodes::{AiNode, AiNodeSetFlt, AiNodeSetStr, AiNodeSetVec},
    ai_params::{AI_TYPE_FLOAT, AI_TYPE_UINT},
    ai_render::{AiBegin, AiEnd, AtSessionMode},
};

fn main() {
    // start an Arnold session, log to both a file and the console
    AiBegin(AtSessionMode::AI_SESSION_BATCH);

    AiMsgSetLogFileName("scene1.log");
    AiMsgSetConsoleFlags(None, LogFlags::AI_LOG_ALL);

    // create a sphere geometric primitive
    let sph = AiNode(None, "sphere", Some("mysphere"), None);
    AiNodeSetVec(&sph, "center", 0.0, 4.0, 0.0);
    AiNodeSetFlt(&sph, "radius", 4.0);

    // create a polymesh, with UV coordinates
    let mesh = AiNode(None, "polymesh", Some("mymesh"), None);

    let nsides_array = AiArray::<u32>(1, 1, AI_TYPE_UINT, &[4]);
    AiNodeSetArray(&mesh, "nsides", &nsides_array);
    let vlist_array = AiArray::<f32>(
        12,
        1,
        AI_TYPE_FLOAT,
        &[
            -10.0, 0.0, 10.0, 10.0, 0.0, 10.0, -10.0, 0.0, -10.0, 10.0, 0.0, -10.0,
        ],
    );
    AiNodeSetArray(&mesh, "vlist", &vlist_array);
    let vidxs_array = AiArray::<u32>(4, 1, AI_TYPE_UINT, &[0, 1, 3, 2]);
    AiNodeSetArray(&mesh, "vidxs", &vidxs_array);
    let uvlist_array = AiArray::<f32>(
        8,
        1,
        AI_TYPE_FLOAT,
        &[0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0],
    );
    AiNodeSetArray(&mesh, "uvlist", &uvlist_array);
    let uvidxs_array = AiArray::<u32>(4, 1, AI_TYPE_UINT, &[0, 1, 2, 3]);
    AiNodeSetArray(&mesh, "uvidxs", &uvidxs_array);

    // create a red standard surface shader
    let shader1 = AiNode(None, "standard_surface", Some("myshader1"), None);
    AiNodeSetRGB(&shader1, "base_color", 0.1, 0.2, 0.9);
    AiNodeSetFlt(&shader1, "specular", 0.05);

    // create a textured standard surface shader
    let shader2 = AiNode(None, "standard_surface", Some("myshader2"), None);
    AiNodeSetRGB(&shader2, "base_color", 1.0, 0.0, 0.0);

    // create an image shader for texture mapping
    let image = AiNode(None, "image", Some("myimage"), None);
    AiNodeSetStr(&image, "filename", "examples/textures/leaves.jpg");
    AiNodeSetFlt(&image, "sscale", 4.0);
    AiNodeSetFlt(&image, "tscale", 4.0);
    // link the output of the image shader to the color input of the surface shader
    AiNodeLink(&image, "base_color", Some(&shader2));

    // assign the shaders to the geometric objects
    AiNodeSetPtr(&sph, "shader", &shader1);
    AiNodeSetPtr(&mesh, "shader", &shader2);

    // create a perspective camera
    let camera = AiNode(None, "persp_camera", Some("mycamera"), None);
    // position the camera (alternatively you can set 'matrix')
    AiNodeSetVec(&camera, "position", 0.0, 10.0, 35.0);
    AiNodeSetVec(&camera, "look_at", 0.0, 3.0, 0.0);
    AiNodeSetFlt(&camera, "fov", 45.0);

    // create a point light source
    let light = AiNode(None, "point_light", Some("mylight"), None);
    AiNodeSetStr(&light, "name", "mylight");

    // position the light (alternatively use 'matrix')
    AiNodeSetVec(&light, "position", 15.0, 30.0, 15.0);
    AiNodeSetFlt(&light, "intensity", 4500.0); // alternatively, use 'exposure'
    AiNodeSetFlt(&light, "radius", 4.0); // for soft shadows

    // get the global options node and set some options
    let options = AiUniverseGetOptions(None);
    AiNodeSetInt(&options, "AA_samples", 8);
    AiNodeSetInt(&options, "xres", 480);
    AiNodeSetInt(&options, "yres", 360);
    AiNodeSetInt(&options, "GI_diffuse_depth", 4);
    // set the active camera (optional, since there is only one camera)
    AiNodeSetPtr(&options, "camera", &camera);

    // create an output driver node
    let driver = AiNode(None, "driver_png", Some("mydriver"), None);
    AiNodeSetStr(&driver, "filename", "scene1.png");

    // create a gaussian filter node
    let _filter = AiNode(None, "gaussian_filter", Some("myfilter"), None);

    // assign the driver and filter to the main (beauty) AOV,
    // which is called "RGBA" and is of type RGBA
    let outputs_array = AiArrayAllocate(1, 1, AI_TYPE_STRING);
    AiArraySetStr(&outputs_array, 0, "RGBA RGBA myfilter mydriver");
    AiNodeSetArray(&options, "outputs", &outputs_array);

    if true {
        // finally, render the image!
        let session = AiRenderSession(None, AtSessionMode::AI_SESSION_BATCH);
        AiRender(&session, AtRenderMode::AI_RENDER_MODE_CAMERA);
    } else {
        // ... or you can write out an .ass file instead
        AiASSWrite(None, "scene1.ass", NodeTypes::AI_NODE_ALL, false, false)
            .expect("Can write ass file");
    }
    // Arnold session shutdown
    AiEnd();
}
