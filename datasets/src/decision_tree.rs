use linfa_trees::DecisionTree;
use linfa::prelude::*;

pub fn run_decision_tree() {
    let (train, test) = linfa_datasets::iris().split_with_ratio(0.9);

    let model = DecisionTree::params().fit(&train).unwrap();

    let predictions = model.predict(&test);

    println!("{:?}", predictions);
    println!("{:?}", test.targets);
}

//linfa_datasets::iris() is a function call that returns the Iris dataset. 
//The Iris dataset is a well-known dataset commonly used for classification tasks in machine learning.

//split_with_ratio(0.9) is a method call on the dataset. 
//It splits the dataset into two subsets: train and test, 
//where approximately 90% of the data is allocated to the train subset 
//and the remaining 10% is allocated to the test subset. 
//The result of this split is assigned to the variables train and test.

//let model = DecisionTree::params().fit(&train).unwrap();:
//DecisionTree::params() creates a new instance of a decision tree with default parameters.
//.fit(&train) fits (trains) the decision tree model using the train dataset. 
//The fit method returns a result, and .unwrap() is used to extract the trained model from the result. 
//The unwrap function will panic if an error occurs during training.
//let predictions = model.predict(&test);:

//model.predict(&test) applies the trained decision tree model to the test dataset 
//and returns the predictions for the target variable. 
//The predictions are assigned to the predictions variable.
