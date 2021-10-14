use jni::AttachGuard;

use crate::utils::JVM;

pub struct Producer {
    jvm_guard: AttachGuard<'static>,
}

impl Producer {
    pub fn new() -> Producer {
        Producer {
            jvm_guard: JVM.attach_current_thread().unwrap(),
        }
    }

    pub fn init_transactions() {
        todo!();
    }
    pub fn begin_transaction() {
        todo!();
    }
    pub fn send_offsets_to_transaction() {
        todo!();
    }
    pub fn commit_transaction() {
        todo!();
    }
    pub fn abort_transaction() {
        todo!();
    }
    pub fn send() {
        todo!();
    }
    pub fn flush() {
        todo!();
    }
    pub fn partitions_for() {
        todo!();
    }
    pub fn metrics() {
        todo!();
    }
    pub fn close() {
        todo!();
    }
}
