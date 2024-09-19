# Vinmonopolet
This is a Rust-based tool designed to scrape and extract product information from [vinmonopolet.no](https://www.vinmonopolet.no/). This tool allows you to compare the alcohol content relative to price, helping you find the most cost-effective alcohol products available.

## Features
* Scrapes product data from Vinmonopolet.
* Calculates the price per centiliter of alcohol for each product.
* Outputs the data in a csv file for importing in tools like excel or Google sheets.

## Installation and usage
### Prerequisites
You will need to install [Rust](https://www.rust-lang.org/tools/install) to run this project.

### Clone the repository
Using git you can clone the repository by running this command:
```bash
git clone https://github.com/Crusage/vinmonopolet
```
Navigate into the project directory:
```bash
cd vinmonopolet
```
### Running the project
Run the scraper using:
```bash
cargo run
```
The scraping process may take 10-20 minutes to complete, depending on the number of products on Vinmonopolet.

Once completed, the extracted data will be saved in a CSV file named "products_data.csv". This file can be imported into excel or Google sheets for further analysis, such as sorting the products.

## Columns
Here are the columns present in the csv, in order, and a product example:

|link|name|price|vol|alc|alc vol|price per alc vol|
|-|-|-|-|-|-|-|
|https://www.vinmonopolet.no/p/12000701|#bio Bardolino Chiaretto Rosé|199.90kr|75.00cl|0.12%|9.38cl|21.31kr|

## To-do
- [ ] Add UI.
  - [ ] Import and export functionality.
  - [ ] View the csv directly without needing to import it to excel or Google sheets.
  - [ ] Add more columns and ability to sort by other things like categories, or minimum and maximum alcohol percentage.
- [ ] Improve code documentation.
- [ ] Make into a library. (maybe)
- [ ] Add binary release.
- [ ] Add proxy support. (maybe)
- [ ] Optimize more with smarter threading.
## Contributing
If you find a bug or have a feature request, please open an issue or submit a pull request.

## License
This project is licensed under the [MIT License](LICENSE). Feel free to use, modify, and distribute this project as needed.