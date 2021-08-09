/// Module for computing the rating of the Worker and Customer
use frame_support::sp_runtime::FixedPointNumber;

#[derive(Clone)]
pub enum RatingType {
    Worker(WorkerRating),
    Customer(CustomerRating),
}

#[derive(Clone)]
pub struct WorkerRating {
    // Task division number
    pub milestone: i32,
    // Amount staked bt the worker per milestone
    pub percentage_of_bidding_amt: f32,
    // rating on scale of 1-5(0.2 - 1)
    pub customer_provided_rating: f32,
}

impl Default for WorkerRating {
    fn default() -> Self {
        Self {
            milestone: 1,
            percentage_of_bidding_amt: 0.1,
            customer_provided_rating: 1.0,
        }
    }
}

#[derive(Clone)]
pub struct CustomerRating {
    // rating provided by worker for the customer
    pub worker_provided_rating: f32,
    // Amount staked bt the worker per milestone
    pub worker_bidding_amount_percentage: f32,
    // rating on scale of 1-5(0.2 - 1)
    pub worker_current_rating: f32,
}

impl Default for CustomerRating {
    fn default() -> Self {
        Self {
            worker_provided_rating: 1.0,
            worker_bidding_amount_percentage: 0.1,
            worker_current_rating: 1.0,
        }
    }
}

#[derive(Clone)]
pub struct Rating {
    pub rating_type: RatingType,
    pub rating: f32,
}

impl Rating {
    pub fn new(rating_type: RatingType) -> Self {
        let rating;
        match rating_type.clone() {
            RatingType::Worker(worker_rating) => {
                rating = Rating::compute_rating(
                    worker_rating.milestone as f32,
                    worker_rating.percentage_of_bidding_amt,
                    worker_rating.customer_provided_rating,
                );
            }
            RatingType::Customer(customer_rating) => {
                rating = Rating::compute_rating(
                    customer_rating.worker_provided_rating,
                    customer_rating.worker_bidding_amount_percentage,
                    customer_rating.worker_current_rating,
                );
            }
        }
        Self {
            rating_type,
            rating,
        }
    }

    fn compute_rating(a: f32, b: f32, c: f32) -> f32 {
        let mut res;

        let calc_rating = a * b * c;

        res = Rating::round_to_f32(calc_rating, 2);

        if calc_rating <= 0.1 {
            res = 1.00;
        }
        res
    }

    // returns a number rounded to the spefied
    // number of decimal places
    fn round_to_f32(num: f32, precision: u32) -> f32 {
        let base: i32 = 10;
        let factor = base.pow(precision) as f32;
        let step_one = num * factor;
        step_one / factor
    }
}

// Trial Code for Substrate Callable methods
// let worker_rating = Rating::new(RatingType::Worker(WorkerRating {
//     milestone: 1,
//     percentage_of_bidding_amt: 0.8,
//     customer_provided_rating: 3.0,
// }));

// let res1 = Rating::new(RatingType::Worker(WorkerRating {
//     milestone: 1,
//     percentage_of_bidding_amt: 0.898769,
//     ..Default::default()
// }));

// debug::info!("-------------1---------------");
// debug::info!("Worker Rating: {:#?}", worker_rating.rating);
// debug::info!("Worker Rating: {:#?}", res1.rating);
