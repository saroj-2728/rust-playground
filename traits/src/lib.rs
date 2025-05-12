#![allow(unused)]
use std::fmt::{Debug, Display};

pub trait Summary {
    // fn summarize(&self) -> String; // This way we need to define method for each type that implements this trait
    // But if we use a default implementation instead
    fn summarize(&self) -> String {
        format!("(Read more...), by {}", self.summarize_author())
    }

    // then we only need to use a block like
    // impl Summary for NewsArticle {} to implement this on the type NewsArticle
    // if method is again written as below, the default one will be overwritten

    // Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation.

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub location: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // The default summarize method will be applied to the NewsArticle type
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

// Traits as Parameters
pub fn notify(item: &impl Summary) {
    // The parameter accepts any type that implements the Summary trait
    println!("Breaking news! \n{}", item.summarize());
}

// Trait Bound Syntax
// The above fn is short hand form of the below fn, both forms are important
pub fn notify1<T: Summary>(item: &T) {
    println!("Breaking news! \n{}", item.summarize());
}

pub fn notify2(item1: &impl Summary, item2: &impl Summary) {
    // This form allows both parameters to be of different types that both implement Summary
    println!("{} and {}", item1.summarize(), item2.summarize());
}

pub fn notify3<T: Summary>(item1: &T, item2: &T) {
    // This form forces both parameters to be of the same type that implement Summary
    // Eg. Both NewsArticle or Both Tweet
    println!("{} and {}", item1.summarize(), item2.summarize());
}

// Multiple Trait Bounds with + Syntax
pub fn notify_more(item: &(impl Summary + Display)) {
    // Do sth here
}

// Also valid for with Trait Bounds
pub fn notify_more1<T: Summary + Display>(item: &T) {
    // Do sth here
}

// Trait Bounds with where Claused
// Using too many trait bounds makes fn unreadable, so use where clause
// Instead of this
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// Do this
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    34
}

// Using Trait Bounds to Conditionally Implement Methods
pub struct Point2D<T> {
    x: T,
    y: T,
}

impl<T> Point2D<T> { // A method for any type
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Point2D<T> { 
    fn cmp_display(&self) { // A method for a type that implements Display and PartialOrd traits
        if self.x >= self.y {
            println!("x co-ordinate is greater, x = {}", self.x);
        }
        else {
            println!("y co-ordinate is greater, y = {}", self.y);
        }
    }
}