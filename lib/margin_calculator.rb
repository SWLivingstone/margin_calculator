require "helix_runtime"

begin
  require "margin_calculator/native"
rescue LoadError
  warn "Unable to load margin_calculator/native. Please run `rake build`"
end
