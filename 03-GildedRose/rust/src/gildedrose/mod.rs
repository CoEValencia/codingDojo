use std::string;
use std::vec;

static AGED_BRIE: &'static str = "Aged Brie";
static MAGIC_HAND: &'static str = "Sulfuras, Hand of Ragnaros";
static BACKSTAGE_PASSES: &'static str = "Backstage passes to a TAFKAL80ETC concert";
const  QUALITY_UNIT: i32 = 1;
const  MAX_QUALITY: i32 = 50;
const  MIN_QUALITY: i32 = 0;
const  MIN_SELL_IN: i32 = 0;

pub struct Item {
    pub name: string::String,
    pub sell_in: i32,
    pub quality: i32
}

impl Item {
    pub fn new(name: String, sell_in: i32, quality: i32) -> Item {
        Item {name: name, sell_in: sell_in, quality: quality}
    }
}

pub struct GildedRose {
    pub items: vec::Vec<Item>
}

fn dec_sell_in(item: &mut Item){
    item.sell_in =  item.sell_in - 1; 
}
fn backstage_passes_handler(item : &mut Item){
    
    dec_sell_in(item);
    let quality : i32  = if item.sell_in < MIN_SELL_IN  {
        0
    } else if item.sell_in <= 5 { 
        item.quality + (QUALITY_UNIT * 3) 
    } else if item.sell_in <= 10 {
        item.quality + (QUALITY_UNIT * 2) 
    } else {
         item.quality + QUALITY_UNIT 
    };
 
    if quality <= MAX_QUALITY {
        item.quality = quality;
    } else {
        item.quality = MAX_QUALITY;
    }
}

fn aged_brie_handler(item : &mut Item){
    
    dec_sell_in(item);
    let quality : i32   = if item.sell_in < MIN_SELL_IN  {
        item.quality + (QUALITY_UNIT * 2) 
    } else {
        item.quality + QUALITY_UNIT 
    };
 
    if quality <= MAX_QUALITY {
        item.quality = quality;
    } else {
        item.quality = MAX_QUALITY;
    }
}

fn no_op_handler(_ : &mut Item) {}

fn generic_handler(item : &mut Item) {
    
    dec_sell_in(item);
    let quality  : i32  = if item.sell_in < MIN_SELL_IN  {
        item.quality - (QUALITY_UNIT * 2)
    } else {
        item.quality - QUALITY_UNIT 
    };
 
    if quality >= MIN_QUALITY {
        item.quality = quality;
    } else {
        item.quality = MIN_QUALITY;
    }
}

fn get_handler(name: &String) -> &'static Fn(&mut Item) -> ()  {

    if name == AGED_BRIE {
        &aged_brie_handler
    } else if name == MAGIC_HAND  {
        &no_op_handler
    } else if name == BACKSTAGE_PASSES  {
        &backstage_passes_handler
    } else {
        &generic_handler
    }
}

impl GildedRose {
    pub fn new(items: vec::Vec<Item>) -> GildedRose {
        GildedRose {items: items}
    }

    pub fn update_quality(&mut self){

        for item in &mut self.items{
           
            let handler = get_handler(&item.name);
            handler(item);
        }
    }
 
    /*pub fn update_quality(&mut self) {
        for item in &mut self.items {
            if item.name != "Aged Brie" && item.name != "Backstage passes to a TAFKAL80ETC concert" {
                if item.quality > 0 {
                    if item.name != "Sulfuras, Hand of Ragnaros" {
                        item.quality = item.quality - 1;
                    }
                }
            } else {
                if item.quality < 50
                {
                    item.quality = item.quality + 1;

                    if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                        if item.sell_in < 11 {
                            if item.quality < 50 {
                                item.quality = item.quality + 1;
                            }
                        }

                        if item.sell_in < 6 {
                            if item.quality < 50 {
                                item.quality = item.quality + 1;
                            }
                        }
                    }
                }
            }

            if item.name != "Sulfuras, Hand of Ragnaros" {
                item.sell_in = item.sell_in - 1;
            }

            if item.sell_in < 0 {
                if item.name != "Aged Brie" {
                    if item.name != "Backstage passes to a TAFKAL80ETC concert" {
                        if item.quality > 0 {
                            if item.name != "Sulfuras, Hand of Ragnaros" {
                                item.quality = item.quality - 1;
                            }
                        }
                    } else {
                        item.quality = item.quality - item.quality;
                    }
                } else {
                    if item.quality < 50 {
                        item.quality = item.quality + 1;
                    }
                }
            }
        }
    }*/
}

#[cfg(test)]
mod test;
