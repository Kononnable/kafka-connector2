use jni::AttachGuard;

use crate::utils::JVM;

pub struct Consumer {
    jvm_guard: AttachGuard<'static>,
}

impl Consumer {
    pub fn new() -> Consumer {
        Consumer {
            jvm_guard: JVM.attach_current_thread().unwrap(),
        }
    }
    pub fn assignment() {
        todo!();
    }
    pub fn subscription() {
        todo!();
    }
    pub fn subscribe() {
        todo!();
    }
    pub fn unsubscribe() {
        todo!();
    }
    pub fn assign() {
        todo!();
    }
    pub fn pool() {
        todo!();
    }
    pub fn update_assignment_metadata_if_needed() {
        todo!();
    }
    pub fn commit_sync() {
        todo!();
    }
    pub fn commit_async() {
        todo!();
    }
    pub fn seek() {
        todo!();
    }
    pub fn seek_to_beginning() {
        todo!();
    }
    pub fn seek_to_end() {
        todo!();
    }
    pub fn position() {
        todo!();
    }
    pub fn committed() {
        todo!();
    }
    pub fn metrics() {
        todo!();
    }
    pub fn partitions_for() {
        todo!();
    }
    pub fn list_topics() {
        todo!();
    }
    pub fn pause() {
        todo!();
    }
    pub fn resume() {
        todo!();
    }
    pub fn paused() {
        todo!();
    }
    pub fn offsets_for_times() {
        todo!();
    }
    pub fn beginning_offsets() {
        todo!();
    }
    pub fn end_offsets() {
        todo!();
    }
    pub fn current_lag() {
        todo!();
    }
    pub fn group_metadata() {
        todo!();
    }
    pub fn enforce_rebalance() {
        todo!();
    }
    pub fn close() {
        todo!();
    }
    pub fn wakeup() {
        todo!();
    }
}
