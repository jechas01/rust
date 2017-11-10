// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cmp::Ordering;

use parking_lot;
use owning_ref::OwningRef;

pub use std::sync::Arc as Rcd;

pub type ReadGuardRef<'a, T, U = T> = OwningRef<ReadGuard<'a, T>, U>;

pub use parking_lot::RwLockReadGuard as ReadGuard;

pub fn assert_sync<T: ?Sized + Sync>() {}

#[derive(Debug)]
pub struct RwLock<T>(parking_lot::RwLock<T>);

const ERROR_CHECKING: bool = true;

impl<T> RwLock<T> {
    #[inline(always)]
    pub fn new(inner: T) -> Self {
        RwLock(parking_lot::RwLock::new(inner))
    }

    #[inline(always)]
    pub fn borrow(&self) -> parking_lot::RwLockReadGuard<T> {
        if ERROR_CHECKING {
            self.0.try_read().expect("lock was already held")
        } else {
            self.0.read()
        }
    }

    #[inline(always)]
    pub fn borrow_mut(&self) -> parking_lot::RwLockWriteGuard<T> {
        if ERROR_CHECKING {
            self.0.try_write().expect("lock was already held")
        } else {
            self.0.write()
        }
    }
}

#[derive(Debug)]
pub struct LockCell<T>(Lock<T>);

impl<T> LockCell<T> {
    #[inline(always)]
    pub fn new(inner: T) -> Self {
        LockCell(Lock::new(inner))
    }

    #[inline(always)]
    pub fn set(&self, new_inner: T) {
        *self.0.lock() = new_inner;
    }

    #[inline(always)]
    pub fn get(&self) -> T where T: Copy {
        *self.0.lock()
    }

    #[inline(always)]
    pub fn set_mut(&mut self, new_inner: T) {
        *self.0.get_mut() = new_inner;
    }

    #[inline(always)]
    pub fn get_mut(&mut self) -> T where T: Copy {
        *self.0.get_mut()
    }

    #[inline(always)]
    pub fn get_raw(&self) -> T where T: Copy {
        *self.0.lock()
    }
}

impl<T> LockCell<Option<T>> {
    #[inline(always)]
    pub fn take(&self) -> Option<T> {
        self.0.lock().take()
    }
}

impl<T:Default> Default for LockCell<T> {
    /// Creates a `LockCell<T>`, with the `Default` value for T.
    #[inline]
    fn default() -> LockCell<T> {
        LockCell::new(Default::default())
    }
}

impl<T:PartialEq + Copy> PartialEq for LockCell<T> {
    #[inline]
    fn eq(&self, other: &LockCell<T>) -> bool {
        self.get() == other.get()
    }
}

impl<T:Eq + Copy> Eq for LockCell<T> {}

impl<T:PartialOrd + Copy> PartialOrd for LockCell<T> {
    #[inline]
    fn partial_cmp(&self, other: &LockCell<T>) -> Option<Ordering> {
        self.get().partial_cmp(&other.get())
    }

    #[inline]
    fn lt(&self, other: &LockCell<T>) -> bool {
        self.get() < other.get()
    }

    #[inline]
    fn le(&self, other: &LockCell<T>) -> bool {
        self.get() <= other.get()
    }

    #[inline]
    fn gt(&self, other: &LockCell<T>) -> bool {
        self.get() > other.get()
    }

    #[inline]
    fn ge(&self, other: &LockCell<T>) -> bool {
        self.get() >= other.get()
    }
}

impl<T:Ord + Copy> Ord for LockCell<T> {
    #[inline]
    fn cmp(&self, other: &LockCell<T>) -> Ordering {
        self.get().cmp(&other.get())
    }
}

pub use parking_lot::MutexGuard as LockCellGuard;

#[derive(Debug)]
pub struct Lock<T>(parking_lot::Mutex<T>);

pub use parking_lot::MutexGuard as LockGuard;

impl<T> Lock<T> {
    #[inline(always)]
    pub fn new(inner: T) -> Self {
        Lock(parking_lot::Mutex::new(inner))
    }

    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }

    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut()
    }

    #[inline(always)]
    pub fn lock(&self) -> parking_lot::MutexGuard<T> {
        if ERROR_CHECKING {
            self.0.try_lock().expect("lock was already held")
        } else {
            self.0.lock()
        }
    }

    #[inline(always)]
    pub fn borrow(&self) -> parking_lot::MutexGuard<T> {
        self.lock()
    }

    #[inline(always)]
    pub fn borrow_mut(&self) -> parking_lot::MutexGuard<T> {
        self.lock()
    }
}

impl<T: Clone> Clone for Lock<T> {
    #[inline]
    fn clone(&self) -> Self {
        Lock::new(self.borrow().clone())
    }
}