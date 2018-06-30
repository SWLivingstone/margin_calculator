#![recursion_limit = "1024"]
#[macro_use]
extern crate helix;
impl Copy for Cm0Values {}
impl Copy for Cm1Values {}
impl Copy for Cm2Values {}

fn calculate_relative_margin(absolute: f64, net_retail: f64) -> f64 {
    (absolute / net_retail) * 100.00
}

fn calculate_cm0(values: Cm0Values) -> MarginCalculation {
    let absolute = values.shipping_revenue + values.net_retail - values.wholesale_price;
    let relative = calculate_relative_margin(absolute, values.net_retail);
    MarginCalculation::new(absolute, relative)
}

fn calculate_cm1(values: Cm1Values) -> MarginCalculation {
    let cm0values = values.cm0values;
    let cm0 = calculate_cm0(cm0values);

    let absolute = cm0.absolute - (values.return_rate * (values.return_shipping + values.return_fulfillment)) -
        (values.cancellation_rate * values.depreciation * cm0values.net_retail);

    let relative = calculate_relative_margin(absolute, cm0values.net_retail);
    MarginCalculation::new(absolute, relative)
}

fn calculate_cm2(values: Cm2Values) -> MarginCalculation {
    let cm1values = values.cm1values;
    let cm0values = cm1values.cm0values;
    let cm1 = calculate_cm1(cm1values);

    let logistic_payment_and_reclamation_costs =
        values.inbound_shipping + values.packaging + values.fulfillment + values.outbound_shipping +
        ((values.payment_cost * (cm0values.shipping_revenue + values.retail_price)) + (values.refunds * values.retail_price));

    let absolute = cm1.absolute - logistic_payment_and_reclamation_costs;
    let relative = calculate_relative_margin(absolute, cm0values.net_retail);
    MarginCalculation::new(absolute, relative)
}

fn calculate_lowest_possible_price(values: Cm2Values, target_margin: f64, margin_category: String) -> f64 {
    let mut upper_limit = values.retail_price * 2.0;
    let mut lower_limit = 0.0;
    let tax_rate = values.retail_price / values.cm1values.cm0values.net_retail;

    if calculate_cost_margin_with(values, &margin_category, lower_limit, tax_rate) > target_margin {
        upper_limit = -1.0;
    }
    while calculate_cost_margin_with(values, &margin_category, upper_limit, tax_rate) < target_margin {
        upper_limit *= 2.0;
    }
    let mut mid = calculate_mid(upper_limit, lower_limit);

    while mid != upper_limit || mid != lower_limit {
        let cost_margin = calculate_cost_margin_with(values, &margin_category, mid, tax_rate);
        if cost_margin > target_margin {
            upper_limit = mid;
            mid = calculate_mid(upper_limit, lower_limit);
        } else if cost_margin < target_margin {
            lower_limit = mid;
            mid = calculate_mid(upper_limit, lower_limit);
        } else {
            return mid
        }
    }
    mid
}

fn calculate_cost_margin_with(values: Cm2Values, margin_category: &String, retail_price: f64, tax_rate: f64) -> f64 {
    let mut vals = values;
    vals.retail_price = retail_price;
    vals.cm1values.cm0values.net_retail = retail_price / tax_rate;
    let margin = match margin_category.as_ref() {
        "cm0" => { calculate_cm0(vals.cm1values.cm0values).relative }
        "cm1" => { calculate_cm1(vals.cm1values).relative }
        _ => { calculate_cm2(vals).relative }
    };
    (margin * 100.0).round() / 100.0
}

fn calculate_mid(upper_limit: f64, lower_limit: f64) -> f64 {
    (upper_limit / 2.0) + (lower_limit / 2.0)
}

ruby! {

    class Calculator {
        def cm0(values: Cm0Values) -> MarginCalculation {
            calculate_cm0(values)
        }
        def cm1(values: Cm1Values) -> MarginCalculation {
            calculate_cm1(values)
        }
        def cm2(values: Cm2Values) -> MarginCalculation {
            calculate_cm2(values)
        }
        def lowest_possible_price(values: Cm2Values, target_margin: f64, margin_category: String) -> f64 {
            calculate_lowest_possible_price(values, target_margin, margin_category)
        }
    }

    class MarginCalculation {
        struct {
            absolute: f64,
            relative: f64
        }
        def initialize(helix, absolute: f64, relative: f64) {
            MarginCalculation { helix, absolute, relative }
        }
        def absolute(&self) -> f64 {
            self.absolute
        }
        def relative(&self) -> f64 {
            self.relative
        }
    }

    class Cm0Values {
        struct {
            shipping_revenue: f64,
            net_retail: f64,
            wholesale_price: f64
        }

        def initialize(helix, shipping_revenue: f64, net_retail: f64, wholesale_price: f64) {
            Cm0Values { helix, shipping_revenue, net_retail, wholesale_price }
        }
    }

    class Cm1Values {
        struct {
            cm0values: Cm0Values,
            return_rate: f64,
            return_shipping: f64,
            return_fulfillment: f64,
            cancellation_rate: f64,
            depreciation: f64
        }
        def initialize(helix, cm0values: Cm0Values, return_rate: f64, return_shipping: f64,
            return_fulfillment: f64, cancellation_rate: f64, depreciation: f64) {
                Cm1Values { helix, cm0values, return_rate, return_shipping, return_fulfillment,
                    cancellation_rate, depreciation }
            }
    }

    class Cm2Values {
        struct {
            cm1values: Cm1Values,
            outbound_shipping: f64,
            inbound_shipping: f64,
            packaging: f64,
            fulfillment: f64,
            payment_cost: f64,
            refunds: f64,
            retail_price: f64
        }
        def initialize(helix, cm1values: Cm1Values, outbound_shipping: f64, inbound_shipping: f64,
            packaging: f64, fulfillment: f64, payment_cost: f64, refunds: f64, retail_price: f64 ) {
                Cm2Values { helix, cm1values, outbound_shipping, inbound_shipping,
                    refunds, packaging, fulfillment, payment_cost, retail_price }
            }
    }
}
