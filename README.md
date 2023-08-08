# Mileage Budget Calculator

## Description

This project provides users with the ability to define their vehicles and the mileage-based contracts. It then calculates the budget allowance based on mileage for different periods, allowing users to know how far they can drive within specific durations such as a week or a month. The aim is to help users avoid missing their contract limitations and subsequent unnecessary fees.

Additionally, users can now plan and record their trips, providing an estimated mileage for each trip, to further enhance the budget allowance calculation.

## Modules

1. **contract**: Manages contracts related to vehicles.
2. **mileage**: Manages the recording of mileage.
3. **trip**: Manages planned and past trips, providing estimated and actual mileages.
4. **user**: Manages user-specific data, including their vehicles.
5. **vehicle**: Manages details related to individual vehicles.

## Features

- Define vehicles and associated contracts.
- Add mileage records for vehicles.
- Plan and record trips with estimated mileages.
- Calculate the number of kilometers left in the weekly or monthly budget, including considering upcoming trip mileages.
- Provides detailed information about contracts, trips, and their status.

## Future Enhancements

- Integration of real-time tracking and automatic updating of trip mileages based on actual distances covered.

## Getting Started

1. **Clone the Repository**

   ```
   git clone git@github.com:saeedseyfi/drive-plan.git
   ```

2. **Build the Project**

   Navigate to the project directory and use the Rust's package manager, Cargo:

   ```
   cargo build
   ```

3. **Run the Project**

   After building, you can run the project:

   ```
   cargo run
   ```

4. Feel free to expand upon the given modules, or integrate this into a larger system that requires mileage budget calculations.

## Contributions

Contributions, bug reports, and feature requests are welcome! Please open an issue or submit a pull request.

## License

This project is open-source. Ensure you adhere to the licensing terms when using or modifying this software.
