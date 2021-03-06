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

package org.apache.kafka.connect.runtime.errors;

import org.apache.kafka.connect.sink.SinkRecord;
import org.apache.kafka.connect.storage.Converter;
import org.apache.kafka.connect.storage.HeaderConverter;
import org.easymock.Mock;
import org.junit.Before;
import org.junit.Ignore;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.powermock.core.classloader.annotations.PowerMockIgnore;
import org.powermock.modules.junit4.PowerMockRunner;

import java.util.concurrent.CompletableFuture;

import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;


@RunWith(PowerMockRunner.class)
@Ignore
@PowerMockIgnore("javax.management.*")
public class WorkerErrantRecordReporterTest {

    private WorkerErrantRecordReporter reporter;

    @Mock
    private RetryWithToleranceOperator retryWithToleranceOperator;

    @Mock
    private Converter converter;

    @Mock
    private HeaderConverter headerConverter;

    @Mock
    private SinkRecord record;

    @Before
    public void setup() {
        reporter = new WorkerErrantRecordReporter(
            retryWithToleranceOperator,
            converter,
            converter,
            headerConverter
        );
    }

    @Test
    public void testGetAllFutures() {
        assertTrue(reporter.futures.isEmpty());
        for (int i = 0; i < 4; i++) {
            reporter.futures.add(CompletableFuture.completedFuture(null));
        }
        assertFalse(reporter.futures.isEmpty());
        reporter.awaitAllFutures();
        assertTrue(reporter.futures.isEmpty());
    }
}
