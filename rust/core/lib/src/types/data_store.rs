use std::any::Any;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};

type StoreMap = HashMap<String, Box<dyn Any>>;

pub struct DataStore<T: Send + Sync> {
    init: std::sync::Once,
    data: UnsafeCell<Option<StoreMap>>,
    spin_lock: AtomicUsize,
    _type: PhantomData<*mut T>,
}

impl<T: Send + Sync> DataStore<T> {
    #[inline(always)]
    fn _spin_lock(&self) {
        while self
            .spin_lock
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Relaxed)
            .is_err()
        {
            std::thread::yield_now();
        }
    }

    #[inline(always)]
    fn _unlock_spin(&self) {
        assert!(
            self.spin_lock
                .compare_exchange(1, 0, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
        );
    }

    fn _ensure_data_initialized(&self) {
        if !self.init.is_completed() {
            self.init.call_once(|| {
                let data: StoreMap = HashMap::new();
                unsafe {
                    *self.data.get() = Some(data);
                }
            });
        }
    }

    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    fn _get_mutable_data(&self) -> &mut StoreMap {
        self._ensure_data_initialized();
        unsafe { &mut *(*self.data.get()).as_mut().unwrap() }
    }

    fn _set<D: 'static>(&self, key: String, state: D) -> bool {
        self._spin_lock();

        let map = self._get_mutable_data();
        let already_set = map.contains_key(&key);
        if !already_set {
            map.insert(key, Box::new(state) as Box<dyn Any>);
        }

        self._unlock_spin();
        !already_set
    }

    fn _get<D: 'static>(&self, key: &str) -> Option<&D> {
        self._ensure_data_initialized();

        self._spin_lock();
        let map = self._get_mutable_data();
        let value = map.get(key).and_then(|b| b.downcast_ref::<D>());
        self._unlock_spin();
        value
    }
}

impl<T: Send + Sync> DataStore<T> {
    pub fn new() -> Self {
        DataStore {
            init: std::sync::Once::new(),
            data: UnsafeCell::new(None),
            spin_lock: AtomicUsize::new(0),
            _type: PhantomData,
        }
    }

    pub fn set<D: 'static>(&self, key: &str, state: D) -> bool {
        self._set(key.to_owned(), state)
    }

    pub fn get<D: 'static>(&self, key: &str) -> Option<&D> {
        self._get(key)
    }
}
