use std::{u64};

pub struct EngineKey {
    pub key: char,
    pub index_char: usize,
    pub index_line: usize,
}

pub struct EngineParts {
    pub key: EngineKey,
    pub value: u64,
}

pub struct Engine {
    from_map: Vec<String>,
    pub v_map_len: usize,
    pub h_map_len: usize,
    pub parts: Vec<EngineParts>,
}

fn get_line(engine: &Engine, index_line: i64) -> Option<Vec<char>> {
    if index_line >= 0 && (index_line as usize) < engine.v_map_len {
        let line = engine.from_map[index_line as usize].chars().collect();
        Some(line)
    } else {
        None
    }
}

fn discord_engine_key_part(engine: &Engine, index_line: usize, index_char: usize) -> Option<EngineKey> {
    let above_line_option = get_line(engine, (index_line as i64) - 1);
    let below_line_option = get_line(engine, (index_line as i64) + 1);
    let line: Vec<char> = engine.from_map[index_line].chars().collect();
    if above_line_option.is_some() {
        let above_line = above_line_option.unwrap();
        // Top left case
        if index_char > 0 && above_line[index_char - 1].is_numeric() == false && above_line[index_char - 1] != '.' {
            return Some(EngineKey{
                index_line: index_line - 1,
                index_char: index_char - 1,
                key: above_line[index_char - 1],
            });
        }
        // Top right case
        if index_char < engine.h_map_len - 1 && above_line[index_char + 1].is_numeric() == false && above_line[index_char + 1] != '.' {
            return Some(EngineKey{
                index_line: index_line - 1,
                index_char: index_char + 1,
                key: above_line[index_char + 1],
            });
        }
        // Top Case
        if above_line[index_char].is_numeric() == false && above_line[index_char] != '.' {
            return Some(EngineKey{
                index_line: index_line - 1,
                index_char: index_char,
                key: above_line[index_char],
            });
        }
    }
    if below_line_option.is_some() { 
        let below_line = below_line_option.unwrap();
        // Bottom left case
        if index_char > 0 && below_line[index_char - 1].is_numeric() == false && below_line[index_char - 1] != '.' {
            return Some(EngineKey{
                index_line: index_line + 1,
                index_char: index_char - 1,
                key: below_line[index_char - 1],
            });
        }
        // Bottom right case
        if index_char < engine.h_map_len - 1 && below_line[index_char + 1].is_numeric() == false && below_line[index_char + 1] != '.' {
            return Some(EngineKey{
                index_line: index_line + 1,
                index_char: index_char + 1,
                key: below_line[index_char + 1],
            });
        }
        // Bottom Case
        if below_line[index_char].is_numeric() == false && below_line[index_char] != '.' {
            return Some(EngineKey{
                index_line: index_line + 1,
                index_char: index_char,
                key: below_line[index_char],
            });
        }
    }
    // Left case
    if index_char > 0 && line[index_char - 1].is_numeric() == false && line[index_char - 1] != '.' {
        return Some(EngineKey{
            index_line: index_line,
            index_char: index_char - 1,
            key: line[index_char - 1],
        });
    }
    // Right case
    if index_char < engine.h_map_len - 1 && line[index_char + 1].is_numeric() == false && line[index_char + 1] != '.' {
        return Some(EngineKey{
            index_line: index_line,
            index_char: index_char + 1,
            key: line[index_char + 1],
        });
    }
    // Next potential number
    if index_char < engine.h_map_len - 1 && line[index_char + 1].is_numeric() {
        return discord_engine_key_part(engine, index_line, (index_char.clone()) + 1)
    }
    None
}

fn discord_engine_value_part(engine: &Engine, index_line: usize, mut index_char: usize) -> String {
    let mut splitted_value = vec![];
    let line = get_line(engine, index_line as i64).unwrap();
    while line[index_char].is_numeric() && index_char < engine.h_map_len {
        splitted_value.push(line[index_char]);
        if index_char + 1 >= engine.h_map_len {
            break;
        }
        // Funny that compilo understand that index_char is an index for line
        index_char += 1;
    }
    return splitted_value.iter().collect()
}

fn discord_part(engine: &Engine, index_line: usize, index_char: usize) -> Option<EngineParts> {
    let engine_part_key_optionnal = discord_engine_key_part(engine, index_line, index_char);
    if engine_part_key_optionnal.is_none() {
        // println!("No key found for {}, {}", index_line, index_char);
        return None
    }
    let engine_part_key = engine_part_key_optionnal.unwrap(); 
    let engine_part_value = discord_engine_value_part(engine, index_line.clone(), index_char.clone());
    if index_line < 10 {
        println!("Key position: {} {}", engine_part_key.index_line, engine_part_key.index_char);
    }
    Some(EngineParts {
        key: engine_part_key,
        value: engine_part_value.parse().unwrap(),
    })
}

pub fn parse(map: Vec<String>) -> Engine {
    let mut engine = Engine{
        from_map: map.clone(),
        v_map_len: map.len(),
        h_map_len: map[0].len(),
        parts: vec![],
    };
    for (index_line, x) in map.iter().enumerate() {
        let chars: Vec<char> = x.chars().collect();
        let mut index_char = 0;
        while index_char <  x.len() {
            let c = chars[index_char];
            if c.is_numeric() {
               let engine_part = discord_part(&engine, index_line.clone(), index_char.clone());
               match engine_part {
                    Some(part) => {
                        index_char += part.value.to_string().len();
                        engine.parts.push(part) 
                    }
                    None => println!("Found invalid key engine at position {}:{}", index_line, index_char.clone())
               }
            }
            index_char += 1;
        }
    }

    return engine;
}