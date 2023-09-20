require "granite/adapter/pg"

Granite::Connections << Granite::Adapter::Pg.new(name: "testdb", url: "postgres://postgres:pass@127.0.0.1/postgres")

class Stock < Granite::Base
  connection testdb
  table stock

  column name : String, primary: true, auto: false
  column vendor : String
  column quantity : Int32
end

Stock.new(name: "Toothbrush", vendor: "Limo", quantity: 1497).save
Stock.new(name: "Comb", vendor: "Takoon", quantity: 210).save
Stock.new(name: "Towel", vendor: "Beana", quantity: 362).save

Stock.where(:quantity, :gteq, 300).select.each do |item|
  # puts item.name # accessing fields also possible
  puts item.to_json
end
