require 'margin_calculator'

RSpec.describe Calculator do
  let(:values) do
    {
      free_shipping_threshold: 150,
      shipping_revenue: 6.77,
      net_retail: 14.29,
      wholesale_price: 8.00,
      net_recommended_retail_price: 19.00,
      return_rate: 0.094,
      return_shipping: 5.05,
      return_fulfillment: 1.16,
      cancellation_rate: 0.0242,
      depreciation: 0.1,
      inbound_shipping: 0.36,
      packaging: 0.68,
      fulfillment: 3.82,
      outbound_shipping: 5.05,
      payment_cost: 0.0195,
      refunds: 0.0133,
      retail_price: 17.00
    }
  end

  let(:negative_cm0values) do
    Cm0Values.new(
      values[:shipping_revenue],
      values[:net_retail],
      1.0
    )
  end

  let(:cm0values) do
    Cm0Values.new(
      values[:shipping_revenue],
      values[:net_retail],
      values[:wholesale_price]
    )
  end

  let(:cm1values) do
    Cm1Values.new(
      cm0values,
      values[:return_rate],
      values[:return_shipping],
      values[:return_fulfillment],
      values[:cancellation_rate],
      values[:depreciation]
    )
  end

  let(:negative_cm1values) do
    Cm1Values.new(
      negative_cm0values,
      values[:return_rate],
      values[:return_shipping],
      values[:return_fulfillment],
      values[:cancellation_rate],
      values[:depreciation]
    )
  end

  let(:cm2values) do
    Cm2Values.new(
      cm1values,
      values[:outbound_shipping],
      values[:inbound_shipping],
      values[:packaging],
      values[:fulfillment],
      values[:payment_cost],
      values[:refunds],
      values[:retail_price]
    )
  end

  let(:negative_cm2values) do
    Cm2Values.new(
      negative_cm1values,
      values[:outbound_shipping],
      values[:inbound_shipping],
      values[:packaging],
      values[:fulfillment],
      values[:payment_cost],
      values[:refunds],
      values[:retail_price]
    )
  end

  context 'when retail_price is less than 150' do
    it 'calculates cm0' do
      margin_calculation = described_class.cm0(cm0values)
      expect(margin_calculation.relative.round(2)).to eq(91.39)
      expect(margin_calculation.absolute.round(2)).to eq(13.06)

    end

    it 'calculates cm1' do
      margin_calculation = described_class.cm1(cm1values)
      expect(margin_calculation.relative.round(2)).to eq(87.07)
      expect(margin_calculation.absolute.round(2)).to eq(12.44)
    end

    it 'calculates cm2' do
      margin_calculation = described_class.cm2(cm2values)
      expect(margin_calculation.relative.round(2)).to eq(12.89)
      expect(margin_calculation.absolute.round(2)).to eq(1.84)
    end

    context '#lowest_possible_price' do
      it 'calculates the lowest possible price' do
        price = described_class.lowest_possible_price(cm2values, 12.00, 'cm2')
        expect(price.round(2)).to eq(16.82)
      end
      it 'is able to return negative values' do
        price = described_class.lowest_possible_price(negative_cm2values, 45.00, 'cm0')
        expect(price.round(2)).to eq(-12.48)
      end
      it 'is really fast!' do
        start = Time.now
        2000.times do
          cm0values = Cm0Values.new(
            values[:shipping_revenue],
            values[:net_retail],
            values[:wholesale_price]
          )
          cm1values = Cm1Values.new(
            cm0values,
            values[:return_rate],
            values[:return_shipping],
            values[:return_fulfillment],
            values[:cancellation_rate],
            values[:depreciation]
          )
          vals = Cm2Values.new(
            cm1values,
            values[:outbound_shipping],
            values[:inbound_shipping],
            values[:packaging],
            values[:fulfillment],
            values[:payment_cost],
            values[:refunds],
            values[:retail_price]
          )
          price = described_class.lowest_possible_price(vals, 12.00, 'cm2')
          expect(price.round(2)).to eq(16.82)
        end
        finish = Time.now
        puts finish - start
      end
    end
  end
end
