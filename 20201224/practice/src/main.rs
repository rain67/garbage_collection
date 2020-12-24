// Vec<i32>の扱いに慣れる。
fn main() {
    let mut v1 = vec![1,2,3,4,5,6,7,8,9];
    let mut v2 = (1..10).collect::<Vec<i32>>();
    vec_iter1(&mut v1);
    vec_iter2(&mut v2)
}

fn vec_iter1(v: &mut Vec<i32>){
    let v = v.into_iter()
                .map(|x| x.pow(3))
                .collect::<Vec<i32>>(); //collectの書き方その1
    println!("{:?}",v);            
} 

fn vec_iter2(v: &mut Vec<i32>){
    let mut v: Vec<i32> = v.into_iter() // 変数宣言で型を
                .map(|x| x.pow(2))
                .collect();

    println!("{:?}",&v);
    v.push(4);
    println!("{:?}",&v);     
} 

