package org.apache.kafka.common.metrics;

import java.util.Locale;

public enum SensorRecordingLevel {
    INFO(0, "INFO"), DEBUG(1, "DEBUG"), TRACE(2, "TRACE");

    private static final SensorRecordingLevel[] ID_TO_TYPE;
    private static final int MIN_RECORDING_LEVEL_KEY = 0;
    public static final int MAX_RECORDING_LEVEL_KEY;

    static {
        int maxRL = -1;
        for (SensorRecordingLevel level : SensorRecordingLevel.values()) {
            maxRL = Math.max(maxRL, level.id);
        }
        SensorRecordingLevel[] idToName = new SensorRecordingLevel[maxRL + 1];
        for (SensorRecordingLevel level : SensorRecordingLevel.values()) {
            idToName[level.id] = level;
        }
        ID_TO_TYPE = idToName;
        MAX_RECORDING_LEVEL_KEY = maxRL;
    }

    /**
     * an english description of the api--this is for debugging and can change
     */
    public final String name;

    /**
     * the permanent and immutable id of an API--this can't change ever
     */
    public final short id;

    SensorRecordingLevel(int id, String name) {
        this.id = (short) id;
        this.name = name;
    }

    public static SensorRecordingLevel forId(int id) {
        if (id < MIN_RECORDING_LEVEL_KEY || id > MAX_RECORDING_LEVEL_KEY)
            throw new IllegalArgumentException(String.format("Unexpected RecordLevel id `%d`, it should be between `%d` " +
                    "and `%d` (inclusive)", id, MIN_RECORDING_LEVEL_KEY, MAX_RECORDING_LEVEL_KEY));
        return ID_TO_TYPE[id];
    }

    /**
     * Case insensitive lookup by protocol name
     */
    public static SensorRecordingLevel forName(String name) {
        return SensorRecordingLevel.valueOf(name.toUpperCase(Locale.ROOT));
    }

    public native boolean shouldRecord(SensorRecordingLevel value);
//    public boolean shouldRecord(final int configId) {
//        if (configId == INFO.id) {
//            return this.id == INFO.id;
//        } else if (configId == DEBUG.id) {
//            return this.id == INFO.id || this.id == DEBUG.id;
//        } else if (configId == TRACE.id) {
//            return true;
//        } else {
//            throw new IllegalStateException("Did not recognize recording level " + configId);
//        }
//    }
}