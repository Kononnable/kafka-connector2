package org.apache.kafka.common.metrics.stats;

import org.apache.kafka.RustLib;
import org.apache.kafka.common.metrics.MetricConfig;

public class Sample {

    static {
        RustLib.load();
    }

    private long rustPointer;

    public native void rustConstructor(double initialValue, long now);

    public native void rustDestructor();

    @Override
    protected void finalize() throws Throwable {
        rustDestructor();
        super.finalize();
    }

//        public double initialValue;
//        public long eventCount;
//        public long lastWindowMs;
//        public double value;

    public Sample(double initialValue, long now) {
        rustConstructor(initialValue, now);
    }

    public native long eventCount();

    public native Sample eventCount(long value);

    public native long lastWindowMs();

    public native double value();


    public native void reset(long now);
//        public void reset(long now) {
//            this.eventCount = 0;
//            this.lastWindowMs = now;
//            this.value = initialValue;
//        }

    public native boolean isComplete(long timeMs, MetricConfig config);
//        public boolean isComplete(long timeMs, MetricConfig config) {
//            return timeMs - lastWindowMs >= config.timeWindowMs() || eventCount >= config.eventWindow();
//        }

//        @Override
//        public native String toString();
//        public String toString() {
//            return "Sample(" +
//                    "value=" + value +
//                    ", eventCount=" + eventCount +
//                    ", lastWindowMs=" + lastWindowMs +
//                    ", initialValue=" + initialValue +
//                    ')';
//        }
}