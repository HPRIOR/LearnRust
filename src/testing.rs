// #[cfg(test)]                    // marks a test module
// #[test]                         // marks a test function
// assert!()                       // macro will fail on false
// panics                          //   will cause fails
// assert_eq! assert_ne!           // equal and not equal assertions (order does not matter)
//                                 // uses == and != under the hood, upon failure the macros
//                                 // will print their argument - values being compard
//                                 // must implement PartialEq and Debug traits
//                                 // this is often as easy as adding the #[derive(..)] annotation
//
// The assert macros can have can be passed two extra arguments, a string containing {} placeholder
// and a result to place into the {}. This is used for more descriptive error messages
// #[should_panic]                 // asserts that a give test should panic
// #[should_panic(expected = "add descriptive message")]

