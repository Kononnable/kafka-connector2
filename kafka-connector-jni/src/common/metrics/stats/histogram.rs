use jni::{
    objects::{JObject, JValue},
    sys::{jarray, jdouble, jint},
    JNIEnv,
};

use crate::{java_stored_object::FromJObject, java_struct_standard_impl};

#[derive(Debug, Clone)]
pub struct Histogram {
    bin_scheme: BinScheme,
    hist: Vec<f32>,
    count: f64,
}

impl Histogram {
    pub fn new(bin_scheme: BinScheme) -> Self {
        let bins = match bin_scheme {
            BinScheme::Constant { bins, .. } | BinScheme::Linear { bins, .. } => bins,
        };
        let hist = (0..bins).into_iter().map(|_| 0_f32).collect();
        Self {
            bin_scheme,
            count: 0.0,
            hist,
        }
    }
    pub fn record(&mut self, value: f64) {
        let bin = self.bin_scheme.to_bin(value);
        *self.hist.get_mut(bin).unwrap() += 1.0;
        self.count += 1.0;
    }
    pub fn counts(&self) -> &[f32] {
        self.hist.as_ref()
    }
    pub fn clear(&mut self) {
        self.hist.iter_mut().for_each(|x| *x = 0.0);
        self.count = 0.0;
    }

    pub fn value(&self, quantile: f64) -> f64 {
        if self.count == 0.0 {
            return f64::NAN;
        }
        if quantile > 1.0 {
            return f64::INFINITY;
        }
        if quantile < 0.0 {
            return f64::NEG_INFINITY;
        }
        let mut sum = 0_f32;
        let min_count = (quantile * self.count) as f32;
        for (i, val) in self.hist.iter().enumerate() {
            sum += val;
            if sum > min_count {
                return self.bin_scheme.from_bin(i);
            }
        }
        self.bin_scheme.from_bin(self.hist.len() - 1)
    }
}

#[derive(Debug, Clone)]
pub enum BinScheme {
    Constant {
        bins: usize,
        min: f64,
        max: f64,
        bucket_width: f64,
        max_bin_number: usize,
    },
    Linear {
        bins: usize,
        max: f64,
        scale: f64,
    },
}
impl BinScheme {
    pub fn new_constant(bins: usize, min: f64, max: f64) -> BinScheme {
        debug_assert!(bins > 1);
        BinScheme::Constant {
            bins,
            max,
            min,
            bucket_width: (max - min) / bins as f64,
            max_bin_number: bins - 1,
        }
    }
    pub fn new_linear(bins: usize, max: f64) -> BinScheme {
        debug_assert!(bins > 1);
        let scale = max / (bins * (bins - 1) / 2) as f64;
        BinScheme::Linear { bins, max, scale }
    }
    fn to_bin(&self, x: f64) -> usize {
        match *self {
            BinScheme::Constant {
                min,
                bucket_width,
                max_bin_number,
                ..
            } => {
                let bin_number = ((x - min) / bucket_width) as usize;
                usize::min(bin_number, max_bin_number)
            }
            BinScheme::Linear { bins, max, scale } => {
                debug_assert!(x >= 0.0);
                if x > max {
                    bins - 1
                } else {
                    (-0.5 + 0.5 * f64::sqrt(1.0 + 8.0 * x / scale)) as usize
                }
            }
        }
    }

    fn from_bin(&self, b: usize) -> f64 {
        match *self {
            BinScheme::Constant {
                max_bin_number,
                min,
                bucket_width,
                ..
            } => {
                if b > max_bin_number {
                    f64::INFINITY
                } else {
                    min + b as f64 * bucket_width
                }
            }
            BinScheme::Linear { bins, scale, .. } => {
                if b > bins - 1 {
                    f64::INFINITY
                } else {
                    scale * ((b * (b + 1)) / 2) as f64
                }
            }
        }
    }
}

java_struct_standard_impl!(Histogram, "org/apache/kafka/common/metrics/stats/Histogram");
from_jobject!(BinScheme, "");

/*
 * Class:     org_apache_kafka_common_metrics_stats_Histogram
 * Method:    rustConstructor
 * Signature: (Lorg/apache/kafka/common/metrics/stats/Histogram/BinScheme;)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Histogram_rustConstructor(
    env: JNIEnv,
    obj: JObject,
    bin_scheme: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let bin_scheme = BinScheme::from_jobject(env, bin_scheme)?;
        let scheme = bin_scheme.obj.clone().unwrap().as_ref().clone();
        let histogram = Box::new(Histogram::new(scheme));
        let ptr = Box::into_raw(histogram);
        env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Histogram
 * Method:    rustDestructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Histogram_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _obj = unsafe { Box::from_raw(ptr as *mut Histogram) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Histogram
 * Method:    record
 * Signature: (D)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Histogram_record(
    env: JNIEnv,
    obj: JObject,
    val: jdouble,
) {
    let result = || -> jni::errors::Result<_> {
        let mut histogram = Histogram::from_jobject(env, obj)?;
        histogram.modify(|histogram| histogram.record(val));
        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Histogram
 * Method:    value
 * Signature: (D)D
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Histogram_value(
    env: JNIEnv,
    obj: JObject,
    val: jdouble,
) {
    let result = || -> jni::errors::Result<_> {
        let mut histogram = Histogram::from_jobject(env, obj)?;
        histogram.modify(|histogram| histogram.value(val));
        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

// /*
//  * Class:     org_apache_kafka_common_metrics_stats_Histogram
//  * Method:    counts
//  * Signature: ()[F
//  */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Histogram_counts(
    env: JNIEnv,
    obj: JObject,
) -> jarray {
    let result = || -> jni::errors::Result<_> {
        let histogram = Histogram::from_jobject(env, obj)?;
        let counts = histogram.counts();
        let arr = env.new_float_array(counts.len() as i32)?;
        env.set_float_array_region(arr, 0, counts)?;
        Ok(arr)
    }();
    match result {
        Ok(val) => val,
        Err(jni::errors::Error::JavaException) => JObject::null().into_inner(),
        _ => panic!("{:?}", result),
    }
}
/*
 * Class:     org_apache_kafka_common_metrics_stats_Histogram
 * Method:    clear
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Histogram_clear(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let mut histogram = Histogram::from_jobject(env, obj)?;
        histogram.modify(|histogram| histogram.clear());
        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Histogram_ConstantBinScheme
 * Method:    rustConstructor
 * Signature: (IDD)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Histogram_00024ConstantBinScheme_rustConstructor(
    env: JNIEnv,
    obj: JObject,
    bins: jint,
    min: jdouble,
    max: jdouble,
) {
    let result = || -> jni::errors::Result<_> {
        let bin_scheme = Box::new(BinScheme::new_constant(bins as usize, min, max));
        let ptr = Box::into_raw(bin_scheme);
        env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Histogram_ConstantBinScheme
 * Method:    rustDestructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Histogram_00024ConstantBinScheme_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _obj = unsafe { Box::from_raw(ptr as *mut BinScheme) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Histogram_ConstantBinScheme
 * Method:    bins
 * Signature: ()I
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Histogram_00024ConstantBinScheme_bins(
    env: JNIEnv,
    obj: JObject,
) -> jint {
    java_bin_scheme_bins(env, obj)
}

fn java_bin_scheme_bins(env: JNIEnv, obj: JObject) -> i32 {
    let result = || -> jni::errors::Result<_> {
        let mut bin_scheme = BinScheme::from_jobject(env, obj)?;
        let bins = bin_scheme.modify(|bin_scheme| match bin_scheme {
            BinScheme::Constant { bins, .. } => *bins,
            BinScheme::Linear { bins, .. } => *bins,
        });
        Ok(bins)
    }();
    match result {
        Ok(val) => val as i32,
        Err(jni::errors::Error::JavaException) => 0,
        _ => panic!("{:?}", result),
    }
}
/*
 * Class:     org_apache_kafka_common_metrics_stats_Histogram_ConstantBinScheme
 * Method:    fromBin
 * Signature: (I)D
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Histogram_00024ConstantBinScheme_fromBin(
    env: JNIEnv,
    obj: JObject,
    b: jint,
) -> jdouble {
    java_bin_scheme_from_bin(env, obj, b)
}

fn java_bin_scheme_from_bin(env: JNIEnv, obj: JObject, b: jint) -> f64 {
    let result = || -> jni::errors::Result<_> {
        let bin_scheme = BinScheme::from_jobject(env, obj)?;
        let ret = bin_scheme.from_bin(b as usize);
        Ok(ret)
    }();
    match result {
        Ok(val) => val as jdouble,
        Err(jni::errors::Error::JavaException) => 0.0,
        _ => panic!("{:?}", result),
    }
}
/*
 * Class:     org_apache_kafka_common_metrics_stats_Histogram_ConstantBinScheme
 * Method:    toBin
 * Signature: (D)I
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Histogram_00024ConstantBinScheme_toBin(
    env: JNIEnv,
    obj: JObject,
    x: jdouble,
) -> jint {
    java_bin_scheme_to_bin(env, obj, x)
}

fn java_bin_scheme_to_bin(env: JNIEnv, obj: JObject, x: jdouble) -> i32 {
    let result = || -> jni::errors::Result<_> {
        let bin_scheme = BinScheme::from_jobject(env, obj)?;
        let ret = bin_scheme.to_bin(x);
        Ok(ret)
    }();
    match result {
        Ok(val) => val as jint,
        Err(jni::errors::Error::JavaException) => 0,
        _ => panic!("{:?}", result),
    }
}
/*
 * Class:     org_apache_kafka_common_metrics_stats_Histogram_LinearBinScheme
 * Method:    rustConstructor
 * Signature: (ID)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Histogram_00024LinearBinScheme_rustConstructor(
    env: JNIEnv,
    obj: JObject,
    bins: jint,
    max: jdouble,
) {
    let result = || -> jni::errors::Result<_> {
        let bin_scheme = Box::new(BinScheme::new_linear(bins as usize, max));
        let ptr = Box::into_raw(bin_scheme);
        env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
* Class:     org_apache_kafka_common_metrics_stats_Histogram_LinearBinScheme
* Method:    rustDestructor
* Signature: ()V
*/
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Histogram_00024LinearBinScheme_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _obj = unsafe { Box::from_raw(ptr as *mut BinScheme) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
* Class:     org_apache_kafka_common_metrics_stats_Histogram_LinearBinScheme
* Method:    bins
* Signature: ()I
*/
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Histogram_00024LinearBinScheme_bins(
    env: JNIEnv,
    obj: JObject,
) -> jint {
    java_bin_scheme_bins(env, obj)
}

/*
* Class:     org_apache_kafka_common_metrics_stats_Histogram_LinearBinScheme
* Method:    fromBin
* Signature: (I)D
*/
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Histogram_00024LinearBinScheme_fromBin(
    env: JNIEnv,
    obj: JObject,
    b: jint,
) -> jdouble {
    java_bin_scheme_from_bin(env, obj, b)
}

/*
* Class:     org_apache_kafka_common_metrics_stats_Histogram_LinearBinScheme
* Method:    toBin
* Signature: (D)I
*/
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Histogram_00024LinearBinScheme_toBin(
    env: JNIEnv,
    obj: JObject,
    x: jdouble,
) -> jint {
    java_bin_scheme_to_bin(env, obj, x)
}
