
use std::fs::File;
use csv;
#[derive(Debug)]
struct TaxBracket {
    wage_ceil :  f32,
    tax : f32,
}


fn calculate_tax(yearly_wage : f32) -> f32 {

    // get tax brackets from the csv
    // this function should take several 
    let brackets_file = File::open("src/tax_brackets.csv").unwrap();
    let mut reader = csv::Reader::from_reader(brackets_file);
    let mut brackets : Vec<TaxBracket> = Vec::new();

    for line in reader.records() {
        let fields = line.unwrap();
        let wage_ceil: f32 = fields[0].parse().unwrap();
        let tax: f32 = fields[1].parse().unwrap();

        let tax_bracket = TaxBracket { wage_ceil, tax };
        brackets.push(tax_bracket);        
    }

    // Tax calculation itself
    let mut tax : f32 = 0.0;
    let mut prev_ceil:f32 = 0.0;
    for bracket in brackets {
        if yearly_wage < bracket.wage_ceil {
            tax += (yearly_wage - prev_ceil) * (bracket.tax / 100.0);
            break;
        }
        else {
            tax += (bracket.wage_ceil - prev_ceil) * (bracket.tax / 100.0);
            prev_ceil = bracket.wage_ceil;
        }

    }


    return tax 
}



fn main() {
    println!("--- Welcome to rusty_finances");
    let gross_wage: f32 = 30000.0;
    let tax = calculate_tax(gross_wage);

    println!("The tax is {}%, or, {} eur. Net wage is: {}eur",tax/gross_wage * 100.0,tax,gross_wage - tax);
}
