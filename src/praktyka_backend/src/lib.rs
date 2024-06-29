use std::{borrow::{Borrow, BorrowMut}, cell::RefCell};


thread_local! {
    static TYTULY: RefCell<Vec<String>> = RefCell::default();
    static TRESCI: RefCell<Vec<String>> = RefCell::default();
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::update]
fn nowyTytul(tytul: String) {
    TYTULY.with(|tytuly| {
        tytuly.borrow_mut().push(tytul)
    });
}

#[ic_cdk::update]
fn noweTresci(tresc: String) {
    TRESCI.with(|tresci|{
        tresci.borrow_mut().push(tresc)
    });      
}

#[ic_cdk::query]
fn odczytajTytuly() -> Vec<String> {
    TYTULY.with(|tytuly|{
        tytuly.borrow().clone()
    })
}