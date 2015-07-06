#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::atomic::fence;
    use core::atomic::AtomicBool;
    use core::atomic::Ordering::{Relaxed, Release, Acquire, AcqRel, SeqCst};

    use std::sync::Arc;

    use std::thread;

    // pub fn fence(order: Ordering) {
    //     unsafe {
    //         match order {
    //             Acquire => intrinsics::atomic_fence_acq(),
    //             Release => intrinsics::atomic_fence_rel(),
    //             AcqRel  => intrinsics::atomic_fence_acqrel(),
    //             SeqCst  => intrinsics::atomic_fence(),
    //             Relaxed => panic!("there is no such thing as a relaxed fence")
    //         }
    //     }
    // }

    type T = usize;

    #[test]
    fn fence_test1() {
	let atomicbool: AtomicBool = AtomicBool::new(false); // M
	let ready: Arc<AtomicBool> = Arc::new(atomicbool);
	let ready_clone: Arc<AtomicBool> = ready.clone();

	let data: Arc<T> = Arc::<T>::new(0);
	let clone: Arc<T> = data.clone();

	thread::spawn(move || {
	    unsafe { *(&*clone as *const T as *mut T) = 42 }; // α
	    fence(Release); // A
	    ready_clone.store(true, Relaxed); // X
	});

	while !ready.load(Relaxed) {} // Y

	fence(Acquire); // B
	assert_eq!(*data, 42); // β
    }

    #[test]
    fn fence_test2() {
	let atomicbool: AtomicBool = AtomicBool::new(false); // M
	let ready: Arc<AtomicBool> = Arc::new(atomicbool);
	let ready_clone: Arc<AtomicBool> = ready.clone();

	let data: Arc<T> = Arc::<T>::new(0);
	let clone: Arc<T> = data.clone();

	thread::spawn(move || {
	    unsafe { *(&*clone as *const T as *mut T) = 42 }; // α
	    fence(AcqRel); // A
	    ready_clone.store(true, Relaxed); // X
	});

	while !ready.load(Relaxed) {} // Y

	fence(AcqRel); // B
	assert_eq!(*data, 42); // β
    }

    #[test]
    fn fence_test3() {
	let atomicbool: AtomicBool = AtomicBool::new(false); // M
	let ready: Arc<AtomicBool> = Arc::new(atomicbool);
	let ready_clone: Arc<AtomicBool> = ready.clone();

	let data: Arc<T> = Arc::<T>::new(0);
	let clone: Arc<T> = data.clone();

	thread::spawn(move || {
	    unsafe { *(&*clone as *const T as *mut T) = 42 }; // α
	    fence(SeqCst); // A
	    ready_clone.store(true, Relaxed); // X
	});

	while !ready.load(Relaxed) {} // Y

	fence(SeqCst); // B
	assert_eq!(*data, 42); // β
    }

    #[test]
    #[should_panic]
    fn fence_test4() {
	fence(Relaxed); // panicked at 'there is no such thing as a relaxed fence'
    }
}
