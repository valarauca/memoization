//Copyright 2016 William Cody Laeder
//
//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
//limitations under the License.

//!Memoization offers a simple generic enum that allows for variables and 
//!structure fields to become memoized. This crate only has 1 instance of unsafe
//!code within itself.
//!
//!The goal of this crate is to enable complex/timely operations that only
//!need to be performed once to only be done once. And then to be accessed via 
//!a structure field, and borrowed like a normal structure field. Not though
//!weird function or memory allocation patterns.
//!
//!When constructed Memoized will be as large as the constructed field.
//!The act of writing to, or processing/initializing the field does not cost
//!allocations (unless the initalizer lambda does). The minimum size is 2 
//!pointers.
//!
//!The deref, derefmut, and borrow fields are overloaded. So as a structure
//!field the contained data can be written to, and borrowed as if it was the 
//!normal field it constructs. derefmut does contain unsafe code, to allow the
//!user to write to the uninitailized field. This is mearly to cut down on
//!allocations performed. 
//!
//!I believe this should be able to be ported to `core`, as none of its code
//!necessarily needs standard. This maybe a future change.

use std::marker::PhantomData;
use std::ops::Deref;
use std::borrow::Borrow;
use std::ops::DerefMut;

///Magical Enum
///
///The generic enum that allows for memoization to happeni.
///
///      use memoization::Memoized;
///
///      struct Example<I:'static, O: Clone, F: Fn(I) -> O> {
///           data: Memoized<I,O,F>
///      }
///
///      fn eq_str(a: &str, b: &str) -> bool {
///            a == b
///      }
///
///      let lambda = |x: i32 | -> String {
///            x.to_string()
///      };
///      let mut dut = Example {
///            data: Memoized::new(lambda)
///      };
///      //field is memoized but it can still be written too.
///      *dut.data = "9001".to_string();
///      //field can be borrowed as its output data type.
///      assert!( eq_str( &dut.data, "9001") );
///
///
pub enum Memoized<I:'static,O,Func:Fn(I)->O> {
    UnInitialized(PhantomData<&'static I>,Box<Func>),
    Processed(O),
}
impl<I:'static,O,Func:Fn(I)->O> Memoized<I,O,Func> {
    
    ///Build a new memoized field. The user will pass a lambda function
    ///that will initialize the field.
    ///
    ///     use memoization::Memoized;
    ///     
    ///     let lambda = | a: (i32,i64,&str) | -> String {
    ///           format!("Line {:?} DateCode {:?} Log \"{}\"",a.0,a.1,a.2)
    ///     };
    ///     
    ///     let memoized = Memoized::new(lambda);
    ///     
    pub fn new(lambda: Func) -> Memoized<I,O,Func> {
        Memoized::UnInitialized(PhantomData,Box::new(lambda))
    }
    ///This will convert an UnInitialized value to a Processed value. When
    ///called on a Processed value this function will PANIC.
    ///
    ///     use memoization::Memoized;
    ///
    ///     let tup: (i32,i64,&'static str) = (20,-15,"Hello World!");
    ///     let lambda = | a: (i32,i64,&str) | -> String {
    ///           format!("Line {:?} DateCode {:?} Log \"{}\"",a.0,a.1,a.2)
    ///     };
    ///     let mut memoized = Memoized::new(lambda);
    ///     //process the data
    ///     memoized.process(tup);
    ///     //borrowing the processed, as it's processed data type
    ///     let x: &str = &memoized;
    ///     assert_eq!( x, "Line 20 DateCode -15 Log \"Hello World!\"");
    ///
    pub fn process(&mut self, data: I) {
        let val = match self {
            &mut Memoized::Processed(_) => panic!("Already processed"),
            &mut Memoized::UnInitialized(_,ref z) => z(data)
        };
        *self = Memoized::Processed(val);
    }
    ///Informs user if a field has been Processed.
    ///
    ///     use memoization::Memoized;
    ///
    ///     let tup: (i32,i64,&'static str) = (20,-15,"Hello World!");
    ///     let lambda = | a: (i32,i64,&str) | -> String {
    ///           format!("Line {:?} DateCode {:?} Log \"{}\"",a.0,a.1,a.2)
    ///     };
    ///     let mut memoized = Memoized::new(lambda);
    ///     //data is not initalized/processed
    ///     assert!( ! memoized.processed() );
    ///     //process the data
    ///     memoized.process(tup);
    ///     //data is now initalized
    ///     assert!( memoized.processed() );
    ///
    pub fn processed(&self) -> bool {
        match self {
            &Memoized::Processed(_) => true,
            _ => false
        }
    }
}
impl<I:'static,O,Func:Fn(I)->O> Deref for Memoized<I,O,Func> {
    type Target = O;
    fn deref<'b>(&'b self) -> &'b Self::Target {
        match self {
            &Memoized::Processed(ref x) => x,
            _ => panic!("Attempted to derefence uninitalized memoized value")
        }
    }
}
impl<I:'static,O,Func:Fn(I)->O> DerefMut for Memoized<I,O,Func> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut Self::Target {
        if self.processed() {
            match self {
                &mut Memoized::Processed(ref mut x) => return x,
                _ => unreachable!()
            };
        } else {
            *self = Memoized::Processed(unsafe{std::mem::zeroed()});
            match self {
                &mut Memoized::Processed(ref mut x) => return x,
                _ => unreachable!()
            };
        }
    }
}
impl<I:'static,O,Func:Fn(I)->O> Borrow<O> for Memoized<I,O,Func> {
    fn borrow<'b>(&'b self) -> &'b O {
        match self {
            &Memoized::Processed(ref x) => x,
            _ => panic!("Attempted to borrow uninitalized memoized value")
        }
    }
}

mod test {
    use super::Memoized;
    #[test]
    fn test_memoized_0() {
        //build lambda function
        let lambda = |x: i32| -> String {
            x.to_string()
        };
        //build object
        let mut dut = Memoized::new(lambda);
        //value shouldn't be initialized
        assert_eq!(dut.processed(), false);
        //initialized the value
        dut.process(5);
        //check value is initialized
        assert_eq!(dut.processed(), true);
        //check on borrow
        let x: &str = &dut;
        assert_eq!( x, "5");
    }
    #[test]
    fn test_memoized_2() {

        let lambda = |x:i32| -> String {
            x.to_string()
        };
        let mut dut = Memoized::new(lambda);
        *dut = "5000".to_string();
        assert_eq!(dut.processed(), true);
        assert_eq!(*dut, "5000");
        assert!( eq_str(&dut, "5000"));
    }
    fn eq_str(a: &str, b: &str) -> bool {
        a == b
    }
    struct Testing<I:'static,O,F: Fn(I)->O> {
        data: Memoized<I,O,F>
    }
    #[test]
    fn test_testing_pattern() {
        let lambda = |x: i32| -> String {
            x.to_string()
        };
        let mut dut = Testing {
            data: Memoized::new(lambda)
        };
        //data doesn't exist yet
        assert!( ! dut.data.processed());
        //assign data
        *dut.data = "9000".to_string();
        //now it is
        assert!( dut.data.processed());
        assert!( eq_str(&dut.data, "9000"));
    }
}
