use std::io::{self, Write};

//create a struct 
struct Subject {
    title: String,
    grade: f64,
    units: u8,
    weight: f64,
}

//constructor for Subject
impl Subject {
    fn new(title: &str, grade: f64, units: u8) -> Subject {
        Subject {
            title: title.to_string(),
            units,
            grade,
            weight: grade * units as f64,
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().to_string()
}

fn calculate_average(subjects: &Vec<Subject>) -> f64 {
    let mut total_weight: f64 = 0.0;
    let mut total_units = 0;
    
    for subj in subjects {
        total_weight =  total_weight + subj.weight;
        total_units = total_units + subj.units;
    }
   
    return total_weight / total_units as f64;

}
fn main() {
    //create a vector of Subjects struct 
    let mut subjects: Vec<Subject> = Vec::new();

    let mut count = 1;
    //put in a loop
    loop {
        println!("type 'done' to finish");
        //get user input for the grade and units
        println!("Subject {}", count);
        let grade = get_input("Enter grade: ");
        if grade == "done" {
            break;
        }

        let grade: f64 = grade.parse().expect("not valid");

        let units = get_input("Enter units: ");
        let units: u8 = units.parse().expect("not valid");

        let title = format!("Subject {}", count);
        count = count + 1;
        
        subjects.push(Subject::new(&title, grade, units));

    }

    //print the vector of struct
    println!("Subjects");

    for subject in &subjects {
        println!("{}: Grade:{}, Units:{}, Weight:{}", subject.title, subject.grade, subject.units, subject.weight);
    }
    
    //calculate the average of the weight
    let average = calculate_average(&subjects);
    println!("GWA: {:2}", average);
}
