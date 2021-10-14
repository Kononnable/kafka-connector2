use jni::{InitArgsBuilder, JNIVersion, JavaVM};
use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref JVM: JavaVM = JavaVM::new(
        InitArgsBuilder::new()
            .version(JNIVersion::V8)
            .option("-Xcheck:jni")
            .option(&format!(
                "-Djava.class.path={}",
                "./kafka/clients/build/classes/java/main"
            ))
            .build()
            .unwrap()
    )
    .unwrap();
}
