#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32), // todo!("Move `mileage: u32` field into `age` field - a tuple with two fields: an `Age` enum, u32");
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
enum Age { New, Used }

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Create a tuple for the car quality with the Age ("New" or "Used") and mileage
// Return a tuple with the arrow `->` syntax
fn car_quality (miles: u32) -> (Age, u32) {
    let quality: (Age, u32);
    //todo!("Add conditional expression: If car has accumulated miles, return tuple for Used car with current mileage");

    if miles > 0 {
        quality = (Age::Used, miles);
    }
    else {
        quality = (Age::New, miles);
    }

    //todo!("Return tuple for New car with zero miles");

    // Declare and initialize the return tuple value
    // For a new car, set the miles to 0
    //let quality: (Age, u32) = (Age::New, miles);//todo!("Set the `Age` value to \"New\", set the mileage using the `miles` input argument");

    // Return the completed tuple to the caller
    //todo!("Return the tuple");
    return quality;
}

// Build a new "Car" using the values of four input arguments
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has a hard top roof)
// - miles (u32)
// Call the car_quality(miles) function to get the car age
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {

    // Show details about car order
    // - Check if order is for Used or New car, then check the roof type
    // - Print details for New or Used car based on roof type
    //todo!("Add conditional expression: If car is Used age, then check roof type");
    if car_quality(miles).0 == Age::Used {
        if roof == true {
        //todo!("Add conditional expression: If roof is a hard top, print details");
            // Call the `println!` macro to show the car order details
            println!("Prepare a used car: {:?}, {}, Hard top, {} miles\n", motor, color, miles);
        }
        else {
            println!("Prepare a used car: {:?}, {}, Convertible, {} miles\n", motor, color, miles);
        }
    }
    else {
        if roof == true {
        //todo!("Add conditional expression: If roof is a hard top, print details");
            // Call the `println!` macro to show the car order details
            println!("Build a new car: {:?}, {}, Hard top, {} miles\n", motor, color, miles);
        }
        else {
            println!("Build a new car: {:?}, {}, Convertible, {} miles\n", motor, color, miles);
        }
    }

    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments
    // - "age" field calls "car_quality" function with "miles" input argument
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality (miles), // todo!("Replace `mileage: miles` with `age` tuple field, call `car_quality()` with `miles` as input argument");
    }
}

fn main() {
    // Car order #1: New, Manual, Hard top
    car_factory(String::from("Orange"), Transmission::Manual, true, 0);

    // Car order #2: Used, Semi-automatic, Convertible
    car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);

    // Car order #3: Used, Automatic, Hard top
    car_factory(String::from("White"), Transmission::Automatic, true, 3000);
}

/*
fn main() {
    // Create car color array
    let colors = ["Blue", "Green", "Red", "Silver"];//todo!("Set the enum values: 0 = Blue, 1 = Green, 2 = Red, 3 = Silver");

    // Declare the car type and initial values
    let mut car: Car;// todo!("Create `car` as a `Car` struct");
    let mut engine = Transmission::Manual; //= todo!("Declare `engine` as a `Transmission` enum, initialize to `Manual`");

// Order 3 cars, one car for each type of transmission

    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #2: Used, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #3: Used, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
}
*/
