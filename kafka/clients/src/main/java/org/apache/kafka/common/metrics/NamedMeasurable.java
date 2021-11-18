package org.apache.kafka.common.metrics;

import org.apache.kafka.RustLib;
import org.apache.kafka.common.MetricName;

public class NamedMeasurable {


    static {
        RustLib.load();
    }

    private long rustPointer;

    public native void rustConstructor(MetricName name, Measurable stat);

    public native void rustDestructor();

    @Override
    protected void finalize() throws Throwable {
        rustDestructor();
        super.finalize();
    }

//
//    private final MetricName name;
//    private final Measurable stat;

    public NamedMeasurable(MetricName name, Measurable stat) {
        rustConstructor(name, stat);
    }

    public native MetricName name();
//    public MetricName name() {
//        return name;
//    }

    public native Measurable stat();
//    public Measurable stat() {
//        return stat;
//    }

}