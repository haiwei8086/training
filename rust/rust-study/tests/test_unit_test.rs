use study::for_test;

#[test]
pub fn it_works(){
    assert_eq!(4, for_test::add_two(2));
}

#[test]
#[ignore]
pub fn it_ignore(){

}

#[test]
#[should_panic]
pub fn it_should_panic(){
    assert_eq!("Hello", "World");
}
