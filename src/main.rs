mod test1;
mod test2;
mod test3;
// use test1::test111::test111_fn;
use crate::test1::test111::test111_fn;
// use test2::test22::test222::test222_fn;
use test2::test222_fn;
use test3::test33::test333::test333_fn;
include!("test4/test44.rs");
/*fn main() {
    //语句；或者表达式
    //=拷贝或所有权转移运算符
    // println!("hello {:X}",110);
    /*test111_fn();
    test222_fn();
    test333_fn();
    test444();*/
    /*for i in 1..10 {
        for j in 1..10{
            println!("{}*{}={}",i,j,i*j);
        }
    }*/
    enum MyOption<T>{
        MyNone,
        MySome(T)
    }
    use MyOption::MyNone;
    use MyOption::MySome;
    let a:MyOption<i32>=MyNone;
    let xx=match a {
        MyNone=>{
            println!("XXX");
            12
        }
        MySome(i)=>{
            println!("holle {}",i);
            13
        }
    };
    println!("{}",xx);


}*/
//  生命周期
/*fn main() {
//生命周期
    let mut s:&String=&"xxx".to_string();
    {
        let mut a = "abc".to_string();
        const b: &'static str = "def";//'static
        s=plus(&mut a,&b);
        println!("{}",s);
    }
// let mut s=plus(&mut a);

}
//正常生命周期'a
//全局生命周期（数据段）'static
/*
函数的每一个参数，都有各自独立的生命周期！
可以用生命周期限定符，来缩小原本默认较长的生命周期！
函数有一个参数的时候：返回值和参数生命周期默认一致
如果函数有两个以上参数：返回值必须指定和某个参数共用生命周期
所有参数的生命周期，都会兼容那个最短的
方法的返回值默认和self（this）指针生命周期一样
*/
fn plus<'a>(str1:&'a mut String,str2:&'a str)->&'a String{
    str1.push_str(str2);
    str1
}
// fn plus(str1:&mut String)->&String{
// str1.push_str("def");
// str1
// }*/

//泛型特化
#[feature(specialization)]
use std::fmt::Display;
fn main() {
    struct Person{

    }
    trait XXX{}
    impl XXX for &str{}
    trait Log{//泛型特化
    fn debug(&self,s:&str);
    }
    impl<T> Log for T{
        default fn debug(&self, s: &str) {
            println!("TTTTTTT {}",s);
        }
    }
    impl<U:XXX+Display> Log for U{
        fn debug(&self, s: &str) {
            println!("dddddd{}{}",self,s);
        }
    }
    impl Log for i32{
        fn debug(&self, s: &str) {
            println!("111111 {}",s);
        }
    }

    1.debug("aaa");
    "adfasdf".debug("aaa");
    let p=Person{};
    p.debug("aaa");
}
//match>=switch
// val a=9
//1)
// a match{
// case 1=>{
// println(1)
// }
// case 2=>{
// println(2)
// }
// case _=>{
// println(3)
// }}
//2)
// var b=a match {
// case 1=>{
// 1
// }
// case 2=>{12}
// case _=>{
// println("xxxx")
// "aaa"
// }
// }
// println(b)
// def b(i:Int)=i match { //选择函数--->只能处理一部分数据（偏函数）
// case 1=>{
// 199
// }
//// case 2=>{12}
//// case _=>{
//// println("xxxx")
//// "aaa"
//// }
// }
// println(b(a))
//3)
// var b=a match{
// case i =>"aaaa"+i
// }
// println(b)
// var b=a match{
// case i if i%2!=0 =>"aaaa"+i
// case _=>"bbb"
// }
// println(b)
// val a=(2,"aaa",99)
// var b=a match{
// case (id,name,age) =>"id="+id+",name="+name+",age="+age
// }
// println(b)
var a:Any="abc"

var a:Any="abc"
a match{
case i:Int=>println("int")
case i:String=>println("String")
case _=>println("other")
}