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

import org.apache.kafka.RustLib;
import org.apache.kafka.common.metrics.MeasurableStat;
import org.apache.kafka.common.metrics.MetricConfig;

/**
 * An instantaneous value.
 */
public class Value implements MeasurableStat {
    static {
        RustLib.load();
    }

    private long rustPointer;

    public native void rustConstructor();

    public native void rustDestructor();

    @Override
    protected void finalize() throws Throwable {
        rustDestructor();
        super.finalize();
    }

    public Value() {
        rustConstructor();
    }

//    private double value = 0;

    @Override
    public native double measure(MetricConfig config, long now);
//    public double measure(MetricConfig config, long now) {
//        return value;
//    }

    @Override
    public native void record(MetricConfig config, double value, long timeMs);
//    public void record(MetricConfig config, double value, long timeMs) {
//        this.value = value;
//    }
}
