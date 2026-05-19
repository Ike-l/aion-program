use std::{marker::PhantomData, pin::Pin, sync::Arc, task::{Context, Poll, Waker}};

use hecs::Entity;

use crate::prelude::{FinalisedAccess, Injection, ProgramRegistry};
use parking_lot::Mutex;

pub struct FutureResolve<'a, T> {
    program_registry: &'a Arc<ProgramRegistry>,
    entity: Option<Entity>,
    finalised_accesses: Vec<FinalisedAccess>,

    waker_ready: Arc<Mutex<(Option<Waker>, bool)>>,

    _injection: PhantomData<&'a T>,
}

impl<'a, T> FutureResolve<'a, T> {
    pub fn new(
        program_registry: &'a Arc<ProgramRegistry>,
        entity: Option<Entity>,
        finalised_accesses: Vec<FinalisedAccess>,
        waker_ready: Arc<Mutex<(Option<Waker>, bool)>>, 
    ) -> Self {
        Self {
            program_registry,
            entity,
            finalised_accesses,
            waker_ready,
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
            match self.program_registry.try_resolve::<T>(self.entity, self.finalised_accesses.clone()) {
                Ok(item) => return Poll::Ready(item),
                Err(_) => {
                    waker_ready.1 = false;
                }
            }
        }

        Poll::Pending
    }
}