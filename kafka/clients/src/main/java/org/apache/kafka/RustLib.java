package org.apache.kafka;


import java.io.File;

public class RustLib {

    static {
        File lib = new File("../../target/debug/libkafka_connector_jni.so");
        if (!lib.exists()) {
            lib = new File("../../../target/debug/libkafka_connector_jni.so");
        }
        System.load(lib.getAbsolutePath());
    }
    public static void load(){
        // static code should be run the first time class is used
    }
}
