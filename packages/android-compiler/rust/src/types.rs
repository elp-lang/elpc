use jni::JavaVM;

#[derive(Debug)]
pub struct KotlinCallback {
    pub args: Vec<&str>,
    pub return_type: &str,
}

impl KotlinCallback {
    pub fn new(args: Vec<&str>, return_type: &str) -> Self {
        Self { args, return_type }
    }
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

pub fn get_jvm(env: JNIEnv<'local>) -> JavaVM {
    let jvm = env.get_java_vm().expect("Can't get JVM");
    let jvm_ptr = jvm.get_native_interface().expect("Can't get JVM pointer");
    let jvm = unsafe {
        JNIEnv::from_raw(env, jvm_ptr);
    };

    jvm
}
