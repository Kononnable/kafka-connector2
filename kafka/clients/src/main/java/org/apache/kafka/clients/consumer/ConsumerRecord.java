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
package org.apache.kafka.clients.consumer;

import org.apache.kafka.RustLib;
import org.apache.kafka.common.header.Headers;
import org.apache.kafka.common.header.internals.RecordHeaders;
import org.apache.kafka.common.record.RecordBatch;
import org.apache.kafka.common.record.TimestampType;

import java.util.Optional;

/**
 * A key/value pair to be received from Kafka. This also consists of a topic name and
 * a partition number from which the record is being received, an offset that points
 * to the record in a Kafka partition, and a timestamp as marked by the corresponding ProducerRecord.
 */
public class ConsumerRecord<K, V> {

    static {
        RustLib.load();
    }

    public static final long NO_TIMESTAMP = RecordBatch.NO_TIMESTAMP;
    public static final int NULL_SIZE = -1;


    private long rustPointer;

    public native void rustConstructor(
            String topic,
            int partition,
            long offset,
            long timestamp,
            TimestampType timestampType,
            int serializedKeySize,
            int serializedValueSize,
            K key,
            V value,
            Headers headers,
            Optional<Integer> leaderEpoch
    );

    public native void rustDeconstructor();

    @Override
    protected void finalize() throws Throwable {
        rustDeconstructor();
        super.finalize();
    }

    /**
     * Creates a record to be received from a specified topic and partition (provided for
     * compatibility with Kafka 0.9 before the message format supported timestamps and before
     * serialized metadata were exposed).
     *
     * @param topic     The topic this record is received from
     * @param partition The partition of the topic this record is received from
     * @param offset    The offset of this record in the corresponding Kafka partition
     * @param key       The key of the record, if one exists (null is allowed)
     * @param value     The record contents
     */
    public ConsumerRecord(String topic,
                          int partition,
                          long offset,
                          K key,
                          V value) {
        this(topic, partition, offset, NO_TIMESTAMP, TimestampType.NO_TIMESTAMP_TYPE, NULL_SIZE, NULL_SIZE, key, value,
                new RecordHeaders(), Optional.empty());
    }

    /**
     * Creates a record to be received from a specified topic and partition
     *
     * @param topic               The topic this record is received from
     * @param partition           The partition of the topic this record is received from
     * @param offset              The offset of this record in the corresponding Kafka partition
     * @param timestamp           The timestamp of the record.
     * @param timestampType       The timestamp type
     * @param serializedKeySize   The length of the serialized key
     * @param serializedValueSize The length of the serialized value
     * @param key                 The key of the record, if one exists (null is allowed)
     * @param value               The record contents
     * @param headers             The headers of the record
     * @param leaderEpoch         Optional leader epoch of the record (may be empty for legacy record formats)
     */
    public ConsumerRecord(String topic,
                          int partition,
                          long offset,
                          long timestamp,
                          TimestampType timestampType,
                          int serializedKeySize,
                          int serializedValueSize,
                          K key,
                          V value,
                          Headers headers,
                          Optional<Integer> leaderEpoch) {
        if (topic == null)
            throw new IllegalArgumentException("Topic cannot be null");
        if (headers == null)
            throw new IllegalArgumentException("Headers cannot be null");

//        rustConstructor();
        rustConstructor(topic,
                partition,
                offset,
                timestamp,
                timestampType,
                serializedKeySize,
                serializedValueSize,
                key,
                value,
                headers,
                leaderEpoch);
    }

    /**
     * Creates a record to be received from a specified topic and partition (provided for
     * compatibility with Kafka 0.10 before the message format supported headers).
     *
     * @param topic               The topic this record is received from
     * @param partition           The partition of the topic this record is received from
     * @param offset              The offset of this record in the corresponding Kafka partition
     * @param timestamp           The timestamp of the record.
     * @param timestampType       The timestamp type
     * @param serializedKeySize   The length of the serialized key
     * @param serializedValueSize The length of the serialized value
     * @param key                 The key of the record, if one exists (null is allowed)
     * @param value               The record contents
     * @deprecated use one of the constructors without a `checksum` parameter. This constructor will be removed in
     * Apache Kafka 4.0 (deprecated since 3.0).
     */
    @Deprecated
    public ConsumerRecord(String topic,
                          int partition,
                          long offset,
                          long timestamp,
                          TimestampType timestampType,
                          long checksum,
                          int serializedKeySize,
                          int serializedValueSize,
                          K key,
                          V value) {
        this(topic, partition, offset, timestamp, timestampType, serializedKeySize, serializedValueSize,
                key, value, new RecordHeaders(), Optional.empty());
    }

    /**
     * Creates a record to be received from a specified topic and partition
     *
     * @param topic               The topic this record is received from
     * @param partition           The partition of the topic this record is received from
     * @param offset              The offset of this record in the corresponding Kafka partition
     * @param timestamp           The timestamp of the record.
     * @param timestampType       The timestamp type
     * @param serializedKeySize   The length of the serialized key
     * @param serializedValueSize The length of the serialized value
     * @param key                 The key of the record, if one exists (null is allowed)
     * @param value               The record contents
     * @param headers             The headers of the record.
     * @deprecated use one of the constructors without a `checksum` parameter. This constructor will be removed in
     * Apache Kafka 4.0 (deprecated since 3.0).
     */
    @Deprecated
    public ConsumerRecord(String topic,
                          int partition,
                          long offset,
                          long timestamp,
                          TimestampType timestampType,
                          Long checksum,
                          int serializedKeySize,
                          int serializedValueSize,
                          K key,
                          V value,
                          Headers headers) {
        this(topic, partition, offset, timestamp, timestampType, serializedKeySize, serializedValueSize,
                key, value, headers, Optional.empty());
    }

    /**
     * Creates a record to be received from a specified topic and partition
     *
     * @param topic               The topic this record is received from
     * @param partition           The partition of the topic this record is received from
     * @param offset              The offset of this record in the corresponding Kafka partition
     * @param timestamp           The timestamp of the record.
     * @param timestampType       The timestamp type
     * @param serializedKeySize   The length of the serialized key
     * @param serializedValueSize The length of the serialized value
     * @param key                 The key of the record, if one exists (null is allowed)
     * @param value               The record contents
     * @param headers             The headers of the record
     * @param leaderEpoch         Optional leader epoch of the record (may be empty for legacy record formats)
     * @deprecated use one of the constructors without a `checksum` parameter. This constructor will be removed in
     * Apache Kafka 4.0 (deprecated since 3.0).
     */
    @Deprecated
    public ConsumerRecord(String topic,
                          int partition,
                          long offset,
                          long timestamp,
                          TimestampType timestampType,
                          Long checksum,
                          int serializedKeySize,
                          int serializedValueSize,
                          K key,
                          V value,
                          Headers headers,
                          Optional<Integer> leaderEpoch) {
        this(topic, partition, offset, timestamp, timestampType, serializedKeySize, serializedValueSize, key, value, headers,
                leaderEpoch);
    }

    /**
     * The topic this record is received from (never null)
     */
    public native String topic();

    /**
     * The partition from which this record is received
     */
    public native int partition();

    /**
     * The headers (never null)
     */
    public native Headers headers();

    /**
     * The key (or null if no key is specified)
     */
    public native K key();

    /**
     * The value
     */
    public native V value();

    /**
     * The position of this record in the corresponding Kafka partition.
     */
    public native long offset();

    /**
     * The timestamp of this record
     */
    public native long timestamp();

    /**
     * The timestamp type of this record
     */
    public native TimestampType timestampType();

    /**
     * The size of the serialized, uncompressed key in bytes. If key is null, the returned size
     * is -1.
     */
    public native int serializedKeySize();

    /**
     * The size of the serialized, uncompressed value in bytes. If value is null, the
     * returned size is -1.
     */
    public native int serializedValueSize();

    /**
     * Get the leader epoch for the record if available
     *
     * @return the leader epoch or empty for legacy record formats
     */
    public native Optional<Integer> leaderEpoch();

    @Override
    public String toString() {
        return "ConsumerRecord(topic = " + topic()
                + ", partition = " + partition()
                + ", leaderEpoch = " + leaderEpoch().orElse(null)
                + ", offset = " + offset()
                + ", " + timestampType() + " = " + timestamp()
                + ", serialized key size = " + serializedKeySize()
                + ", serialized value size = " + serializedValueSize()
                + ", headers = " + headers()
                + ", key = " + key()
                + ", value = " + value() + ")";
    }
}
