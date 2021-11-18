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
 * An upper or lower bound for metrics
 */
public final class Quota {

    static {
        RustLib.load();
    }

    private long rustPointer;

    public native void rustConstructor(double bound, boolean upper);

    public native void rustDestructor();

    @Override
    protected void finalize() throws Throwable {
        rustDestructor();
        super.finalize();
    }


    public Quota(double bound, boolean upper) {
        rustConstructor(bound, upper);
    }

    public static Quota upperBound(double upperBound) {
        return new Quota(upperBound, true);
    }

    public static Quota lowerBound(double lowerBound) {
        return new Quota(lowerBound, false);
    }

    public native boolean isUpperBound();
//    public boolean isUpperBound() {
//        return this.upper;
//    }

    public native double bound();
//    public double bound() {
//        return this.bound;
//    }

    public native boolean acceptable(double value);
//    public boolean acceptable(double value) {
//        return (upper && value <= bound) || (!upper && value >= bound);
//    }

    @Override
    public int hashCode() {
        final int prime = 31;
        int result = 1;
        result = prime * result + (int) this.bound();
        result = prime * result + (this.isUpperBound() ? 1 : 0);
        return result;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (!(obj instanceof Quota))
            return false;
        Quota that = (Quota) obj;
        return (that.bound() == this.bound()) && (that.isUpperBound() == this.isUpperBound());
    }

    @Override
    public String toString() {
        return (isUpperBound() ? "upper=" : "lower=") + bound();
    }
}
