use std::marker::PhantomData;
use crate::components::{Component, ComponentStorage};


pub trait QueryFilter: 'static {
    type Item<'a>;
    unsafe fn filter_fetch<'a>(storage: *const ComponentStorage, entity_id: usize) -> Option<Self::Item<'a>>;
}

pub trait QueryFilterMut: 'static {
    type Item<'a>;
    unsafe fn filter_fetch_mut<'a>(storage: *mut ComponentStorage, entity_id: usize) -> Option<Self::Item<'a>>;
}

pub struct Query<'a, F: QueryFilter> {
    storage: *const ComponentStorage,
    _marker: PhantomData<&'a F>,
}

pub struct QueryMut<'a, F: QueryFilterMut> {
    storage: *mut ComponentStorage,
    _marker: PhantomData<&'a mut F>,
}

impl<'a, F: QueryFilter> Query<'a, F> {
    pub fn new(storage: &'a ComponentStorage) -> Self {
        Query {
            storage: storage as *const ComponentStorage,
            _marker: PhantomData,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, F::Item<'a>)> + '_ {
        let max_entity_id = unsafe { (*self.storage).max_entity_id() };
        (0..=max_entity_id).filter_map(|entity_id| {
            unsafe {
                F::filter_fetch(self.storage, entity_id).map(|item| (entity_id, item))
            }
        })
    }
}

impl<'a, F: QueryFilterMut> QueryMut<'a, F> {
    pub fn new(storage: &'a mut ComponentStorage) -> Self {
        QueryMut {
            storage: storage as *mut ComponentStorage,
            _marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, F::Item<'a>)> + '_ {
        let max_entity_id = unsafe { (*self.storage).max_entity_id() };
        (0..=max_entity_id).filter_map(|entity_id| {
            unsafe {
                F::filter_fetch_mut(self.storage, entity_id).map(|item| (entity_id, item))
            }
        })
    }
}

/*impl<A: Component, B: Component> QueryFilter for (A, B) {
    type Item<'a> = (&'a A, &'a B);

    unsafe fn filter_fetch<'a>(storage: *const ComponentStorage, entity_id: usize) -> Option<Self::Item<'a>> {
        Some((
            (*storage).get::<A>(entity_id)?,
            (*storage).get::<B>(entity_id)?
        ))
    }
}

impl<A: Component, B: Component> QueryFilterMut for (A, B) {
    type Item<'a> = (&'a mut A, &'a mut B);

    unsafe fn filter_fetch_mut<'a>(storage: *mut ComponentStorage, entity_id: usize) -> Option<Self::Item<'a>> {
        Some((
            (*storage).get_mut::<A>(entity_id)?,
            (*storage).get_mut::<B>(entity_id)?
        ))
    }
}
*/
// macro to generate QueryFilter and QueryFilterMut impls for tuples of components
macro_rules! impl_query_filter {
    ($($comp:ident),*) => {
        impl<$($comp: Component),*> QueryFilter for ($($comp,)*) {
            type Item<'a> = ($(&'a $comp,)*);

            unsafe fn filter_fetch<'a>(storage: *const ComponentStorage, entity_id: usize) -> Option<Self::Item<'a>> {
                Some((
                    $($comp::filter_fetch(storage, entity_id)?,)*
                ))
            }
        }

        impl<$($comp: Component),*> QueryFilterMut for ($($comp,)*) {
            type Item<'a> = ($(&'a mut $comp,)*);

            unsafe fn filter_fetch_mut<'a>(storage: *mut ComponentStorage, entity_id: usize) -> Option<Self::Item<'a>> {
                Some((
                    $($comp::filter_fetch_mut(storage, entity_id)?,)*
                ))
            }
        }
    };
}

// Query::<(A)>::iter()
impl_query_filter!(A);
// Query::<(A, B)>::iter()
impl_query_filter!(A, B);
// And so on...
impl_query_filter!(A, B, C);
impl_query_filter!(A, B, C, D);
impl_query_filter!(A, B, C, D, E);
impl_query_filter!(A, B, C, D, E, F);
impl_query_filter!(A, B, C, D, E, F, G);
impl_query_filter!(A, B, C, D, E, F, G, H);
impl_query_filter!(A, B, C, D, E, F, G, H, I);
impl_query_filter!(A, B, C, D, E, F, G, H, I, J);
impl_query_filter!(A, B, C, D, E, F, G, H, I, J, K);
impl_query_filter!(A, B, C, D, E, F, G, H, I, J, K, L);

// for non tuple (single) components
// EX:
// Query::<Position>::iter()
// Or   
// Query::<Velocity>::iter()

impl<T: Component> QueryFilter for T {
    type Item<'a> = &'a T;

    unsafe fn filter_fetch<'a>(storage: *const ComponentStorage, entity_id: usize) -> Option<Self::Item<'a>> {
        (*storage).get::<T>(entity_id)
    }
}

impl<T: Component> QueryFilterMut for T {
    type Item<'a> = &'a mut T;

    unsafe fn filter_fetch_mut<'a>(storage: *mut ComponentStorage, entity_id: usize) -> Option<Self::Item<'a>> {
        (*storage).get_mut::<T>(entity_id)
    }
}
