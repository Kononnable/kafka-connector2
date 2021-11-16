/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License. You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package org.apache.kafka.common.metrics.stats;

import java.util.ArrayList;
import java.util.List;

import org.apache.kafka.common.metrics.CompoundStat;
import org.apache.kafka.common.metrics.MeasurableStat;
import org.apache.kafka.common.metrics.MetricConfig;
import org.apache.kafka.common.metrics.NamedMeasurable;
import org.apache.kafka.common.metrics.stats.Histogram.BinScheme;
import org.apache.kafka.common.metrics.stats.Histogram.ConstantBinScheme;
import org.apache.kafka.common.metrics.stats.Histogram.LinearBinScheme;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * A compound stat that reports one or more percentiles
 */
public class Percentiles implements CompoundStat, MeasurableStat {

    private int current = 0;
    protected List<HistogramSample> samples;

    @Override
    public void record(MetricConfig config, double value, long timeMs) {
        HistogramSample sample = current(timeMs);
        if (sample.isComplete(timeMs, config))
            sample = advance(config, timeMs);
        update(sample, config, value, timeMs);
        sample.eventCount += 1;
    }

    public HistogramSample current(long timeMs) {
        if (samples.size() == 0)
            this.samples.add(newSample(timeMs));
        return this.samples.get(this.current);
    }

    private HistogramSample advance(MetricConfig config, long timeMs) {
        this.current = (this.current + 1) % config.samples();
        if (this.current >= samples.size()) {
            HistogramSample sample = newSample(timeMs);
            this.samples.add(sample);
            return sample;
        } else {
            HistogramSample sample = current(timeMs);
            sample.reset(timeMs);
            return sample;
        }
    }

    /* Timeout any windows that have expired in the absence of any events */
    protected void purgeObsoleteSamples(MetricConfig config, long now) {
        long expireAge = config.samples() * config.timeWindowMs();
        for (SampledStat.Sample sample : samples) {
            if (now - sample.lastWindowMs >= expireAge)
                sample.reset(now);
        }
    }

    public double measure(MetricConfig config, long now) {
        purgeObsoleteSamples(config, now);
        return combine(config, now);
    }


    private final Logger log = LoggerFactory.getLogger(Percentiles.class);

    public enum BucketSizing {
        CONSTANT, LINEAR
    }

    private final int buckets;
    private final Percentile[] percentiles;
    private final BinScheme binScheme;
    private final double min;
    private final double max;

    public Percentiles(int sizeInBytes, double max, BucketSizing bucketing, Percentile... percentiles) {
        this(sizeInBytes, 0.0, max, bucketing, percentiles);
    }

    public Percentiles(int sizeInBytes, double min, double max, BucketSizing bucketing, Percentile... percentiles) {
        this.samples = new ArrayList<>(2);
        this.percentiles = percentiles;
        this.buckets = sizeInBytes / 4;
        this.min = min;
        this.max = max;
        if (bucketing == BucketSizing.CONSTANT) {
            this.binScheme = new ConstantBinScheme(buckets, min, max);
        } else if (bucketing == BucketSizing.LINEAR) {
            if (min != 0.0d)
                throw new IllegalArgumentException("Linear bucket sizing requires min to be 0.0.");
            this.binScheme = new LinearBinScheme(buckets, max);
        } else {
            throw new IllegalArgumentException("Unknown bucket type: " + bucketing);
        }
    }

    @Override
    public List<NamedMeasurable> stats() {
        List<NamedMeasurable> ms = new ArrayList<>(this.percentiles.length);
        for (Percentile percentile : this.percentiles) {
            final double pct = percentile.percentile();
            ms.add(new NamedMeasurable(
                    percentile.name(),
                    (config, now) -> value(config, now, pct / 100.0))
            );
        }
        return ms;
    }

    public double value(MetricConfig config, long now, double quantile) {
        purgeObsoleteSamples(config, now);
        float count = 0.0f;
        for (HistogramSample sample : this.samples)
            count += sample.eventCount;
        if (count == 0.0f)
            return Double.NaN;
        float sum = 0.0f;
        float quant = (float) quantile;
        for (int b = 0; b < buckets; b++) {
            for (HistogramSample sample : this.samples) {
                float[] hist = sample.histogram.counts();
                sum += hist[b];
                if (sum / count > quant)
                    return binScheme.fromBin(b);
            }
        }
        return Double.POSITIVE_INFINITY;
    }

    public double combine(MetricConfig config, long now) {
        return value(config, now, 0.5);
    }

    protected HistogramSample newSample(long timeMs) {
        return new HistogramSample(this.binScheme, timeMs);
    }

    protected void update(HistogramSample sample, MetricConfig config, double value, long timeMs) {
        final double boundedValue;
        if (value > max) {
            log.debug("Received value {} which is greater than max recordable value {}, will be pinned to the max value",
                    value, max);
            boundedValue = max;
        } else if (value < min) {
            log.debug("Received value {} which is less than min recordable value {}, will be pinned to the min value",
                    value, min);
            boundedValue = min;
        } else {
            boundedValue = value;
        }

        sample.histogram.record(boundedValue);
    }

}
