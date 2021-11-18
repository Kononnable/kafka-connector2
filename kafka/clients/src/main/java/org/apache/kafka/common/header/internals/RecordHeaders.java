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
package org.apache.kafka.common.header.internals;

import org.apache.kafka.RustLib;
import org.apache.kafka.common.header.Header;
import org.apache.kafka.common.header.Headers;
import org.apache.kafka.common.utils.AbstractIterator;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Iterator;
import java.util.List;
import java.util.Objects;

public class RecordHeaders implements Headers {

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

    public RecordHeaders() {
        this((Iterable<Header>) null);
    }

    public RecordHeaders(Header[] headers) {
        this(headers == null ? null : Arrays.asList(headers));
    }

    public RecordHeaders(Iterable<Header> headers) {
        this.rustConstructor();
        if (headers != null) {
            for (Header header : headers) {
                this.add(header);
            }
        }
    }

    @Override
    public native Headers add(Header header) throws IllegalStateException;
//    public Headers add(Header header) throws IllegalStateException {
//        Objects.requireNonNull(header, "Header cannot be null.");
//        canWrite();
//        headers.add(header);
//        return this;
//    }

    @Override
    public native Headers add(String key, byte[] value) throws IllegalStateException;
//    public Headers add(String key, byte[] value) throws IllegalStateException {
//        return add(new RecordHeader(key, value));
//    }

    @Override
    public native Headers remove(String key) throws IllegalStateException;
//        public Headers remove(String key) throws IllegalStateException {
//        canWrite();
//        checkKey(key);
//        Iterator<Header> iterator = iterator();
//        while (iterator.hasNext()) {
//            if (iterator.next().key().equals(key)) {
//                iterator.remove();
//            }
//        }
//        return this;
//    }

    @Override
    public native Header lastHeader(String key);
//        public Header lastHeader(String key) {
//        checkKey(key);
//        for (int i = headers.size() - 1; i >= 0; i--) {
//            Header header = headers.get(i);
//            if (header.key().equals(key)) {
//                return header;
//            }
//        }
//        return null;
//    }


    @Override
    public native Iterable<Header> headers(final String key);
//    public Iterable<Header> headers(final String key) {
//        checkKey(key);
//        return () -> new FilterByKeyIterator(headers.iterator(), key);
//    }

    @Override
    public native Iterator<Header> iterator();
//    public Iterator<Header> iterator() {
//        return closeAware(headers.iterator());
//    }

    public native Header[] toArray();
//    public Header[] toArray() {
//        return headers.isEmpty() ? Record.EMPTY_HEADERS : headers.toArray(new Header[0]);
//    }

    private void checkKey(String key) {
        if (key == null)
            throw new IllegalArgumentException("key cannot be null.");
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) {
            return true;
        }
        if (o == null || getClass() != o.getClass()) {
            return false;
        }

        RecordHeaders headers1 = (RecordHeaders) o;

        return Arrays.equals(toArray(), headers1.toArray());
    }

    @Override
    public int hashCode() {
        return Arrays.hashCode(toArray());
    }

    @Override
    public String toString() {
        return "RecordHeaders(" +
                "headers = " + Arrays.toString(toArray()) +
                ')';
    }

}
