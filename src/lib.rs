
use std::marker::PhantomData;
use std::clone::Clone;
use std::ops::Deref;
use std::borrow::Borrow;
use std::ops::DerefMut;

pub enum Memoized<I:'static,O:Clone,Func:Fn(I)->O> {
    UnInitialized(PhantomData<&'static I>,Box<Func>),
    Processed(O),
}
impl<I:'static,O:Clone,Func:Fn(I)->O> Memoized<I,O,Func> {
    pub fn new(lambda: Func) -> Memoized<I,O,Func> {
        Memoized::UnInitialized(PhantomData,Box::new(lambda))
    }
    pub fn fetch(&mut self, data: I) -> O {
        let (flag,val) = match self {
            &mut Memoized::Processed(ref x) => (false,x.clone()),
            &mut Memoized::UnInitialized(_,ref z) => (true,z(data))
        };
        if flag {
            *self = Memoized::Processed(val.clone());
        }
        val
    }
    pub fn get(&self) -> O {
        match self {
            &Memoized::Processed(ref x) => x.clone(),
            _ => panic!("Called get on an uninitialized field")
        }
    }
    pub fn is_initialized(&self) -> bool {
        match self {
            &Memoized::Processed(_) => true,
            _ => false
        }
    }
}
impl<I:'static,O:Clone,Func:Fn(I)->O> Deref for Memoized<I,O,Func> {
    type Target = O;
    fn deref<'b>(&'b self) -> &'b Self::Target {
        match self {
            &Memoized::Processed(ref x) => x,
            _ => panic!("Attempted to derefence uninitalized memoized value")
        }
    }
}
impl<I:'static,O:Clone,Func:Fn(I)->O> DerefMut for Memoized<I,O,Func> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut Self::Target {
        if self.is_initialized() {
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
impl<I:'static,O:Clone,Func:Fn(I)->O> Borrow<O> for Memoized<I,O,Func> {
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
        assert_eq!(dut.is_initialized(), false);
        //initialized the value
        assert_eq!(&dut.fetch(5), "5");
        //check value is initialized
        assert_eq!(dut.is_initialized(), true);
        //check get works
        assert_eq!(&dut.get(), "5");
        //check fetch no longer modifies value
        assert_eq!(&dut.fetch(2000), "5");
        assert_eq!(&dut.get(), "5");
        //check on borrow
        let x: &str = &dut;
        assert_eq!( x, "5");
    }
    #[test]
    #[should_panic]
    fn test_memoized_1() {
        //build lambda function
        let lambda = |x: i32| -> String {
            x.to_string()
        };
        //build object
        let dut = Memoized::new(lambda);
        //value shouldn't be initialized
        assert_eq!(dut.is_initialized(), false);
        //get object
        //THIS WILL PANICE
        assert_eq!(&dut.get(), "5");
    }
    #[test]
    fn test_memoized_2() {

        let lambda = |x:i32| -> String {
            x.to_string()
        };
        let mut dut = Memoized::new(lambda);
        *dut = "5000".to_string();
        assert_eq!(dut.is_initialized(), true);
        assert_eq!(*dut, "5000");
        assert!( eq_str(&dut, "5000"));
    }
    fn eq_str(a: &str, b: &str) -> bool {
        a == b
    }
    struct Testing<I:'static,O:Clone,F: Fn(I)->O> {
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
        assert!( ! dut.data.is_initialized());
        //assign data
        *dut.data = "9000".to_string();
        //now it is
        assert!( dut.data.is_initialized());
        let x: &str = &dut.data;
        assert!( eq_str(&dut.data, "9000"));
    }
}
