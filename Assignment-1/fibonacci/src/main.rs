fn main(){
    let mut start=0;
    let mut first=0;
    let mut second=0;
    let mut third=1;
    while start<100{
        if start>1{
            first=second;
            second=third;
            third=third+first;
            println!("{}",third);
            start=start+1;
        }
        else {
            println!("{}",start);
            start=start+1;
        }
    }

}
