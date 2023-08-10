use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use super::Cell;
    use std::sync::Arc;

    #[test]
    fn bad() {
        let x = Arc::new(Cell::new(0));

        let y = Arc::clone(&x);
        let z = Arc::clone(&x);

        let t1 = std::thread::spawn(move || {
            for _ in 1..10000 {
                let a = y.get();
                y.set(a + 1);
            }
        });

        let t2 = std::thread::spawn(move || {
            for _ in 1..10000 {
                let a = z.get();
                z.set(a + 1);
            }
        });

        t1.join().unwrap();
        t2.join().unwrap();

        println!("{:#?}", x.get());
    }
}

#[derive(Clone, Copy)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

impl<T> RefCell<T> {
    pub fn new(new_value: T) -> Self {
        Self {
            value: UnsafeCell::new(new_value),
            state: Cell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&mut self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));

                let reference = Ref { refcell: self };
                Some(reference)
            }
            RefState::Shared(count) => {
                self.state.set(RefState::Shared(count + 1));

                let reference = Ref { refcell: self };
                Some(reference)
            }
            RefState::Exclusive => None,
        }
    }
    pub fn borrow_mut(&mut self) -> Option<RefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);

            let reference = RefMut { refcell: self };
            Some(reference)
        } else {
            None
        }
    }
}

struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::Unshared => unreachable!(),
            RefState::Shared(1) => self.refcell.state.set(RefState::Unshared),
            RefState::Shared(n) => self.refcell.state.set(RefState::Shared(n - 1)),
        }
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(_) | RefState::Unshared => unreachable!(),
            RefState::Exclusive => self.refcell.state.set(RefState::Unshared),
        }
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}

pub struct Rc<T> {
    value: *const T,
}
