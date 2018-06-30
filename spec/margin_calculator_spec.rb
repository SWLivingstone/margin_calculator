require 'rails_helper'

RSpec.describe Pricing::Margins::Calculators do
  let(:margin_values_hash) do
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

  context 'when retail_price is less than 150' do
    it 'calculates cm0' do
      expect(described_class.cm0(margin_values_hash)).to eq(amount: 13.06, percent: 91.39)
    end

    it 'calculates cm1' do
      expect(described_class.cm1(margin_values_hash)).to eq(amount: 12.44, percent: 87.07)
    end

    it 'calculates cm2' do
      expect(described_class.cm2(margin_values_hash)).to eq(amount: 1.84, percent: 12.89)
    end
  end
end
