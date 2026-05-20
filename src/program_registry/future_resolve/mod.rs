use std::{marker::PhantomData, pin::Pin, sync::Arc, task::{Context, Poll, Waker}};

use hecs::Entity;

use crate::prelude::{FinalisedAccess, Injection, ProgramId, ProgramRegistry, ResourceId};
use parking_lot::Mutex;

pub struct FutureResolve<'a, T> {
    program_registry: &'a Arc<ProgramRegistry>,
    entity: Option<Entity>,
    finalised_accesses: Vec<FinalisedAccess>,

    waker_ready: Arc<Mutex<(Option<Waker>, bool)>>,

    cached_keys: Vec<(ProgramId, ResourceId)>,
    _injection: PhantomData<&'a T>,
}

impl<'a, T> FutureResolve<'a, T> {
    pub fn new(
        program_registry: &'a Arc<ProgramRegistry>,
        entity: Option<Entity>,
        finalised_accesses: Vec<FinalisedAccess>,
        cached_keys: Vec<(ProgramId, ResourceId)>,
        waker_ready: Arc<Mutex<(Option<Waker>, bool)>>, 
    ) -> Self {
        Self {
            program_registry,
            entity,
            finalised_accesses,
            waker_ready,
            cached_keys,
            _injection: PhantomData::default(),
        }
    }
}

impl<'a, T: Injection> Future for FutureResolve<'a, T> {
    type Output = T::Item<'a>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut waker_ready = self.waker_ready.lock();
        waker_ready.0 = Some(cx.waker().clone());

        if waker_ready.1 {
            let derived_results = self.finalised_accesses.iter().map(|finalised_access| finalised_access.derive(self.program_registry)).collect::<Vec<_>>();
            match T::resolve_access(self.entity, Arc::clone(&self.program_registry), derived_results) {
                Ok(item) => return Poll::Ready(item),
                Err(_) => {
                    waker_ready.1 = false;
                }
            }
        }

        Poll::Pending
    }
}

impl<'a, T> Drop for FutureResolve<'a, T> {
    fn drop(&mut self) {
        let mut future_resources = self.program_registry.future_resources.lock();

        for key in self.cached_keys.iter() {
            if let Some(waiters) = future_resources.get_mut(key) {
                waiters.retain(|w| !Arc::ptr_eq(w, &self.waker_ready));
            }
        }
    }
}