pub fn number_to_words(mut num: i32) -> String {
    fn helper(num: usize) -> String {
        let ones = ["", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"];
        let tens = ["", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];
        let special = ["Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"];

        let tail = num % 100;
        let mut s = if tail >= 20 {
            if tail % 10 == 0 {
                tens[tail / 10].to_string()
            } else {
                format!("{} {}", tens[tail / 10], ones[tail % 10])
            }
        } else if tail >= 10 {
            special[tail - 10].to_string()
        } else {
            ones[tail].to_string()
        };
        if num >= 100 {
            let hundred = num / 100;
            s = if num % 100 == 0 {
                format!("{} Hundred", ones[hundred])
            } else {
                format!("{} Hundred {}", ones[hundred], s)
            };
        }
        s
    }

    if num == 0 {
        "Zero".to_string()
    } else {
        let mut s = "".to_string();
        let units = ["", "Thousand", "Million", "Billion"];
        let mut counter = 0;
        while num > 0 {
            let mut tail = num % 1000;

            if tail > 0 {
                s = if counter == 0 {
                    helper(tail as usize)
                } else {
                    if s.len() == 0 {
                        format!("{} {}", helper(tail as usize), units[counter])
                    } else {
                        format!("{} {} {}", helper(tail as usize), units[counter], s)
                    }
                };
            }

            num /= 1000;
            counter += 1;
        }
        s
    }
}