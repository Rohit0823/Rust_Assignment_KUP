struct Student {
    name:String,
    roll_no:i32,
    score_of_each_subject:i32,
    department:String,
    school:String,
}


fn new(){
    let student =Student {
        name:String::from("Rohitverma"),
        roll_no:1710154,
        score_of_each_subject:87,
        department:String::from("ComputerScience"),
        school:String::from("inderprasthaengineeringcollege"),
    };


}
struct Score {
    hindi:f32,
    english:f32,
    maths:f32,
    science:f32,
}


fn get_avg(avg: &Score) -> f32{
    let avg: f32 = (avg.hindi + avg.english + avg.maths + avg.science) / 4.0;
    avg
}

fn pass_student(marks: &Score) -> [f32; 4]{
    let mut arr: [f32; 4] = [marks.hindi, marks.english, marks.maths, marks.science];

    let difference1: f32;
    let difference2: f32;
    let difference3: f32;
    let difference4: f32;
    if arr[0] < 35.0 {
        difference1 = 35.0 - arr[0];
        arr[0] = arr[0] + difference1;
    }
    if arr[1] < 35.0 {
        difference2 = 35.0 - arr[1];
        arr[1] = arr[1] + difference2;
    }
    if arr[2] < 35.0 {
        difference3 = 35.0 - arr[2];
        arr[2] = arr[2] + difference3;
    }
    if arr[3] < 35.0 {
        difference4 = 35.0 - arr[3];
        arr[3] = arr[3] + difference4;
    }
    return arr;
}



fn main() {
    let mut studentscore = Score{
        hindi: 32.0,
        english: 62.0,
        maths: 49.0,
        science: 56.0,
    };


    let arr1 = [studentscore.hindi, studentscore.english, studentscore.maths, studentscore.science];
    println!("score {:?}", arr1);

    println!("Avg {}", get_avg(&studentscore));

    let array1: [f32; 4] = pass_student(&studentscore);
    println!("New score {:?}", array1);

    studentscore = Score {
        hindi: array1[0],
        english: array1[1],
        maths: array1[2],
        science: array1[3],
    };

    println!("New Avg {}", get_avg(&studentscore));
}