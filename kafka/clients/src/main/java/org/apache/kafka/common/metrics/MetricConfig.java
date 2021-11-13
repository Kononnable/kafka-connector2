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
package org.apache.kafka.common.metrics;

import org.apache.kafka.RustLib;

import java.util.Map;

/**
 * Configuration values for metrics
 */
public class MetricConfig {

    static {
        RustLib.load();
    }

    private long rustPointer;

    public native void rustConstructor();

    public native void rustDeconstructor();

    @Override
    protected void finalize() throws Throwable {
        rustDeconstructor();
        super.finalize();
    }

//    private Quota quota;
//    private int samples;
//    private long eventWindow;
//    private long timeWindowMs;
//    private Map<String, String> tags;
//    private Sensor.RecordingLevel recordingLevel;

    public MetricConfig() {
        rustConstructor();
//        this.quota = null;
//        this.samples = 2;
//        this.eventWindow = Long.MAX_VALUE;
//        this.timeWindowMs = TimeUnit.MILLISECONDS.convert(30, TimeUnit.SECONDS);
//        this.tags = new LinkedHashMap<>();
//        this.recordingLevel = Sensor.RecordingLevel.INFO;
    }

    public native Quota quota();

    //    public Quota quota() {
//        return this.quota;
//    }
    public native MetricConfig quota(Quota quota);

    //    public MetricConfig quota(Quota quota) {
//        this.quota = quota;w
//        return this;
//    }
    public native long eventWindow();

//    public long eventWindow() {
//        return eventWindow;
//    }

    public native MetricConfig eventWindow(long window);
//    public MetricConfig eventWindow(long window) {
//        this.eventWindow = window;
//        return this;
//    }

    public native long timeWindowMs();
//    public long timeWindowMs() {
//        return timeWindowMs;
//    }

    public native MetricConfig timeWindowMs(long window);
//    public MetricConfig timeWindow(long window, TimeUnit unit) {
//        this.timeWindowMs = TimeUnit.MILLISECONDS.convert(window, unit);
//        return this;
//    }

    public native Map<String, String> tags();
//    public Map<String, String> tags() {
//        return this.tags;
//    }

    public native MetricConfig tags(Map<String, String> tags);
//    public MetricConfig tags(Map<String, String> tags) {
//        this.tags = tags;
//        return this;
//    }

    public native int samples();
//    public int samples() {
//        return this.samples;
//    }

    public native MetricConfig samples(int samples);
//    public MetricConfig samples(int samples) {
//        if (samples < 1)
//            throw new IllegalArgumentException("The number of samples must be at least 1.");
//        this.samples = samples;
//        return this;
//    }

    public native SensorRecordingLevel recordLevel();
//    public Sensor.RecordingLevel recordLevel() {
//        return this.recordingLevel;
//    }

    public native MetricConfig recordLevel(SensorRecordingLevel recordingLevel);
//    public MetricConfig recordLevel(Sensor.RecordingLevel recordingLevel) {
//        this.recordingLevel = recordingLevel;
//        return this;
//    }


}
