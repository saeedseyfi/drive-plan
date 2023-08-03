# Mileage Budget Management System

This system is designed to track and manage a driving contract based on mileage. It allows users to calculate the daily budget, track the mileage used and the mileage left per day.

## Components

- `Contract`: Represents a driving contract that has a start date, end date, starting mileage, and an overall mileage allowance.

- `MileageHistory`: Tracks the history of the mileage records of the vehicle.

- `DrivePlan`: Incorporates a `Contract` and `MileageHistory` to provide an overview and details about the driving plan. It calculates the total mileage used, mileage left, average mileage used per day, and average mileage left per day.

- `Trip`: A structure that can represent planned trips, including start and end dates as well as estimated mileage. (NOTE: as of now, `Trip` functionality has not been fully implemented)

## Usage

Users can create instances of `Contract`, `MileageHistory`, and `DrivePlan`. They can then use various methods provided by these structures to calculate and monitor different aspects of the driving contract, like days passed/left in the contract, average daily mileage usage, and more.

## Future Directions

The system could be extended to include further handling for `Trips`, allowing users to add or remove `Trips`, and incorporating `Trip` data into mileage usage and availability calculations. Additionally, the system could be adjusted to consider different timezones based on user needs.

Overall, this system provides a robust framework for managing mileage-based driving contracts and can be easily expanded to include additional functionality as needed.
