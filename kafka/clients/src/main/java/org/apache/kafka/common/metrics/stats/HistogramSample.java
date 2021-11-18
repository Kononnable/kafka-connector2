package org.apache.kafka.common.metrics.stats;

import org.apache.kafka.common.metrics.MetricConfig;

public class HistogramSample {

    public final Histogram histogram;

    public HistogramSample(Histogram.BinScheme scheme, long now) {
        this.initialValue = 0.0;
        this.eventCount = 0;
        this.lastWindowMs = now;
        this.value = initialValue;
        histogram = new Histogram(scheme);
    }

    public void reset(long now) {
        this.eventCount = 0;
        this.lastWindowMs = now;
        this.value = initialValue;
        histogram.clear();
    }


    public double initialValue;
    public long eventCount;
    public long lastWindowMs;
    public double value;


    public boolean isComplete(long timeMs, MetricConfig config) {
        return timeMs - lastWindowMs >= config.timeWindowMs() || eventCount >= config.eventWindow();
    }
}
