package org.apache.kafka;


import java.io.File;
import java.util.Iterator;

public class RustLib {

    static {
//        String version = "debug";
        String version = "release";
        File lib = new File("../../target/" + version + "/libkafka_connector_jni.so");
        if (!lib.exists()) {
            lib = new File("../../../target/" + version + "/libkafka_connector_jni.so");
        }
        System.load(lib.getAbsolutePath());
    }

    public static void load() {
        // static code should be run the first time class is used
    }

    public static <T> Iterable<T> iteratorToIterable(Iterator<T> iterator) {
        return () -> iterator;
    }
}
