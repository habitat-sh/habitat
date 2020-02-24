use owning_ref::{OwningRef,
                 OwningRefMut,
                 StableAddress};
use parking_lot::{MutexGuard,
                  RwLockReadGuard,
                  RwLockWriteGuard};
use std::ops::{Deref,
               DerefMut};

// A wrapper around `MutexGuard` needed to implement the `StableAddress` trait to use `OwningRef`
pub struct StableMutexGuard<'a, T: ?Sized>(MutexGuard<'a, T>);

unsafe impl<'a, T: ?Sized> StableAddress for StableMutexGuard<'a, T> {}

impl<T: ?Sized> Deref for StableMutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T { &*self.0 }
}

impl<T: ?Sized> DerefMut for StableMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T { &mut *self.0 }
}

impl<'a, T> From<MutexGuard<'a, T>> for StableMutexGuard<'a, T> {
    fn from(guard: MutexGuard<'a, T>) -> StableMutexGuard<'a, T> { Self(guard) }
}

// A wrapper around `RwLockReadGuard` needed to implement the `StableAddress` trait to use
// `OwningRef`
pub struct StableRwLockReadGuard<'a, T: ?Sized>(RwLockReadGuard<'a, T>);

unsafe impl<'a, T: ?Sized> StableAddress for StableRwLockReadGuard<'a, T> {}

impl<T: ?Sized> Deref for StableRwLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T { &*self.0 }
}

impl<'a, T> From<RwLockReadGuard<'a, T>> for StableRwLockReadGuard<'a, T> {
    fn from(guard: RwLockReadGuard<'a, T>) -> StableRwLockReadGuard<'a, T> { Self(guard) }
}

// A wrapper around `RwLockWriteGuard` needed to implement the `StableAddress` trait to use
// `OwningRef`
pub struct StableRwLockWriteGuard<'a, T: ?Sized>(RwLockWriteGuard<'a, T>);

unsafe impl<'a, T: ?Sized> StableAddress for StableRwLockWriteGuard<'a, T> {}

impl<T: ?Sized> Deref for StableRwLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T { &*self.0 }
}

impl<T: ?Sized> DerefMut for StableRwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T { &mut *self.0 }
}

impl<'a, T> From<RwLockWriteGuard<'a, T>> for StableRwLockWriteGuard<'a, T> {
    fn from(guard: RwLockWriteGuard<'a, T>) -> StableRwLockWriteGuard<'a, T> { Self(guard) }
}

/// Typedef of a owning reference that uses a `MutexGuard` as the owner.
pub type MutexGuardRef<'a, T, U = T> = OwningRef<StableMutexGuard<'a, T>, U>;

/// Typedef of a mutable owning reference that uses a `MutexGuard` as the owner.
pub type MutexGuardRefMut<'a, T, U = T> = OwningRefMut<StableMutexGuard<'a, T>, U>;

/// Typedef of a owning reference that uses a `RwLockReadGuard` as the owner.
pub type RwLockReadGuardRef<'a, T, U = T> = OwningRef<StableRwLockReadGuard<'a, T>, U>;

/// Typedef of a owning reference that uses a `RwLockWriteGuard` as the owner.
pub type RwLockWriteGuardRef<'a, T, U = T> = OwningRef<StableRwLockWriteGuard<'a, T>, U>;

/// Typedef of a mutable owning reference that uses a `RwLockWriteGuard` as the owner.
pub type RwLockWriteGuardRefMut<'a, T, U = T> = OwningRefMut<StableRwLockWriteGuard<'a, T>, U>;

#[test]
fn raii_locks() {
    use parking_lot::{Mutex,
                      RwLock};
    {
        let a = Mutex::new(1);
        let a = {
            let a = MutexGuardRef::new(a.lock().into());
            assert_eq!(*a, 1);
            a
        };
        assert_eq!(*a, 1);
        drop(a);
    }
    {
        let a = Mutex::new(1);
        let a = {
            let mut a = MutexGuardRefMut::new(a.lock().into());
            assert_eq!(*a, 1);
            *a = 2;
            a
        };
        assert_eq!(*a, 2);
        drop(a);
    }
    {
        let a = RwLock::new(1);
        let a = {
            let a = RwLockReadGuardRef::new(a.read().into());
            assert_eq!(*a, 1);
            a
        };
        assert_eq!(*a, 1);
        drop(a);
    }
    {
        let a = RwLock::new(1);
        let a = {
            let a = RwLockWriteGuardRef::new(a.write().into());
            assert_eq!(*a, 1);
            let mut a = RwLockWriteGuardRefMut::new(a.into_owner());
            assert_eq!(*a, 1);
            *a = 2;
            a
        };
        assert_eq!(*a, 2);
        drop(a);
    }
}
