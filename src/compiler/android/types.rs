use jni::{
    objects::{JClass, JObject},
    sys::jobject,
    JNIEnv,
};

pub type Unit = ();

pub trait JNIValue<'local> {
    fn to_jni_value(&self) -> jni::objects::JObject<'local>;
}

#[derive(Debug)]
pub struct Composable {
    name: String,
    arguments: Vec<(String, String)>,
}

impl Composable {
    pub fn new(name: &str, arguments: Vec<(String, String)>) -> Self {
        Self {
            name: name.to_string(),
            arguments,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn arguments(&self) -> &Vec<(String, String)> {
        &self.arguments
    }
}

pub fn call_composable_function(
    env: JNIEnv,
    class: JClass,
    composable_name: &str,
    args: Vec<jni::objects::JObject>,
) -> jobject {
    // Convert composable name to JNI string
    let composable_name_jni = env
        .new_string(composable_name)
        .expect("Couldn't create Java string");

    // Convert args to JNI strings
    let mut jni_args = Vec::new();
    for (arg_name, arg_value) in args {
        let arg_name_jni = env
            .new_string(*arg_name)
            .expect("Couldn't create Java string");
        let arg_value_jni = env
            .new_string(*arg_value)
            .expect("Couldn't create Java string");
        jni_args.push(arg_name_jni.into());
        jni_args.push(arg_value_jni.into());
    }

    // Find the Java class
    let class_name = "com/example/YourJavaClass"; // Change this to your Java class name
    let java_class = env
        .find_class(class_name)
        .expect("Couldn't find Java class");

    // Get the method ID of the JNI method corresponding to the composable function
    let method_signature = get_method_signature(env, class, composable_name, args.len());
    let method_id = env
        .get_static_method_id(java_class, composable_name, &method_signature)
        .expect("Couldn't find JNI method");

    // Call the JNI method
    let result = env
        .call_static_method(
            java_class,
            method_id,
            method_signature,
            &[
                JObject::from(composable_name_jni),
                JObject::from(
                    env.new_object_array(args.len() as i32, "java/lang/String", Some(jni_args))
                        .unwrap(),
                ),
            ],
        )
        .expect("JNI method call failed");

    result.into()
}

fn get_method_signature(
    env: JNIEnv,
    class: JClass,
    method_name: &str,
    args_count: usize,
) -> String {
    let mut signature = format!("(");
    for _ in 0..args_count {
        signature.push_str("Ljava/lang/String;");
    }
    signature.push_str(")Ljava/lang/Object;");
    signature
}
