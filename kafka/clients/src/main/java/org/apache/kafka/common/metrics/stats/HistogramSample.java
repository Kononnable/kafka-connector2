package org.apache.kafka.common.metrics.stats;

public class HistogramSample extends SampledStat.Sample {

    public final Histogram histogram;

    public HistogramSample(Histogram.BinScheme scheme, long now) {
        super(0.0, now);
        histogram = new Histogram(scheme);
    }

    @Override
    public void reset(long now) {
        super.reset(now);
        histogram.clear();
    }
}
