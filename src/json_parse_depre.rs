pub use crate::json;
pub mod json_parse_depre {
    #![allow(dead_code)]

    use super::json;
    #[warn(unused_assignments)]
    type Valtype = super::json::json_struct::Valtype;
    type Json = super::json::json_struct::Json;

    fn pair_list_expect(
        data: &str,
        map: &mut super::json::json_struct::JsonMap,
        current_root_idx: usize,
        beginpos: usize,
        current_layer: i8,
    ) -> usize {
        let mut tmp_key = String::new();
        let mut tmp_size = 0;
        let mut j = 0;
        let mut k = 0;
        let mut i = 0;
        let mut t = 0;
        let mut layer = 0;
        let mut is_float = false;
        i = beginpos + 1;
        while data.chars().nth(i).unwrap_or_default() != '}' {
            if data.chars().nth(i).unwrap_or_default() == ':' {
                //buf.layer = current_layer;

                t = i;
                while data.chars().nth(t).unwrap_or_default() != ','
                    && data.chars().nth(t).unwrap_or_default() != '{'
                {
                    t -= 1;
                }

                k = t;
                tmp_key.clear();
                while data.chars().nth(k).unwrap_or_default() != ':' {
                    if data.chars().nth(k).unwrap_or_default() != '{'
                        && data.chars().nth(k).unwrap_or_default() != '['
                        && data.chars().nth(k).unwrap_or_default() != ','
                        && data.chars().nth(k).unwrap_or_default() != '"'
                    {
                        tmp_key.push(data.chars().nth(k).unwrap_or_default());
                    }
                    k += 1;
                }

                if data.chars().nth(i + 1).unwrap_or_default() == 'n'
                    && data.chars().nth(i + 2).unwrap_or_default() == 'u'
                    && data.chars().nth(i + 3).unwrap_or_default() == 'l'
                    && data.chars().nth(i + 4).unwrap_or_default() == 'l'
                {
                    //buf.content = "null";
                    map.rawdata.push(Json {
                        key: tmp_key.clone(),
                        val: Valtype::Str(String::from("null")),
                        layer: current_layer,
                        father_idx: current_root_idx,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[current_root_idx].child_idx.push(tmp_size - 1);
                }
                if data.chars().nth(i + 1).unwrap_or_default() == 't'
                    && data.chars().nth(i + 2).unwrap_or_default() == 'r'
                    && data.chars().nth(i + 3).unwrap_or_default() == 'u'
                    && data.chars().nth(i + 4).unwrap_or_default() == 'e'
                {
                    //bool true
                    //buf.content = true;

                    map.rawdata.push(Json {
                        key: tmp_key.clone(),
                        val: Valtype::BoolT(true),
                        layer: current_layer,
                        father_idx: current_root_idx,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[current_root_idx].child_idx.push(tmp_size - 1);
                }
                if data.chars().nth(i + 1).unwrap_or_default() == 'f'
                    && data.chars().nth(i + 2).unwrap_or_default() == 'a'
                    && data.chars().nth(i + 3).unwrap_or_default() == 'l'
                    && data.chars().nth(i + 4).unwrap_or_default() == 's'
                    && data.chars().nth(i + 5).unwrap_or_default() == 'e'
                {
                    //bool false
                    //buf.content = false;
                    map.rawdata.push(Json {
                        key: tmp_key.clone(),
                        val: Valtype::BoolT(false),
                        layer: current_layer,
                        father_idx: current_root_idx,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[current_root_idx].child_idx.push(tmp_size - 1);
                }
                if data.chars().nth(i + 1).unwrap_or_default() == '"' {
                    //str
                    j = i + 2;

                    while (data.chars().nth(j + 1).unwrap_or_default() != ','
                        || data.chars().nth(j).unwrap_or_default() != '"')
                        && (data.chars().nth(j + 1).unwrap_or_default() != '}'
                            || data.chars().nth(j).unwrap_or_default() != '"')
                    {
                        j += 1;
                    }
                    //-------buf.content.push_back('"');

                    map.rawdata.push(Json {
                        key: tmp_key.clone(),
                        val: Valtype::Str(data[i + 2..j].to_string()),
                        layer: current_layer,
                        father_idx: current_root_idx,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[current_root_idx].child_idx.push(tmp_size - 1);

                    //std::cout << data[j] << "  ssssssss\n";
                    i = j - 1;
                    j = 0;
                }
                if data.chars().nth(i + 1).unwrap_or_default().is_numeric()
                    || (data.chars().nth(i + 1).unwrap_or_default() == '-'
                        && data.chars().nth(i + 2).unwrap_or_default().is_numeric())
                {
                    //digit_all
                    j = i + 1;
                    while data.chars().nth(j).unwrap_or_default() != ','
                        && data.chars().nth(j).unwrap_or_default() != '}'
                    {
                        if data.chars().nth(j).unwrap_or_default() == '.' {
                            is_float = true;
                        }
                        j += 1;
                    }
                    if is_float == true {
                        map.rawdata.push(Json {
                            key: tmp_key.clone(),
                            val: Valtype::Float(
                                data[i + 2..j]
                                    .to_string()
                                    .parse::<f64>()
                                    .unwrap_or_default(),
                            ),
                            layer: current_layer,
                            father_idx: current_root_idx,
                            child_idx: vec![],
                        });
                    } else if is_float == false {
                        map.rawdata.push(Json {
                            key: tmp_key.clone(),
                            val: Valtype::Number(
                                data[i + 2..j]
                                    .to_string()
                                    .parse::<i64>()
                                    .unwrap_or_default(),
                            ),
                            layer: current_layer,
                            father_idx: current_root_idx,
                            child_idx: vec![],
                        });
                    };
                    is_float = false;

                    tmp_size = map.size();
                    map[current_root_idx].child_idx.push(tmp_size - 1);
                    i = j - 1;
                    j = 0;
                }
                //////////////////////////////////////////////////////////////////
                if data.chars().nth(i + 1).unwrap_or_default() == '['
                    && data.chars().nth(i + 2).unwrap_or_default() == ']'
                {
                    //void list

                    map.rawdata.push(Json {
                        key: tmp_key.clone(),
                        val: Valtype::ArrayVoid(String::from("[]")),
                        layer: current_layer,
                        father_idx: current_root_idx,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[current_root_idx].child_idx.push(tmp_size - 1);

                    i = i + 2;
                }
                if data.chars().nth(i + 1).unwrap_or_default() == '{'
                    && data.chars().nth(i + 2).unwrap_or_default() == '}'
                {
                    //pair_list_VOID {}   Object accelerate parsing

                    //buf.content = "{}";
                    map.rawdata.push(Json {
                        key: tmp_key.clone(),
                        val: Valtype::ObjectVoid(String::from("{}")),
                        layer: current_layer,
                        father_idx: current_root_idx,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[current_root_idx].child_idx.push(tmp_size - 1);

                    i = i + 2;
                }
                ///////////////////////
                if data.chars().nth(i + 1).unwrap_or_default() == '[' {
                    //Dimension List array
                    map.rawdata.push(Json {
                        key: tmp_key.clone(),
                        val: Valtype::Array(()),
                        layer: current_layer,
                        father_idx: current_root_idx,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[current_root_idx].child_idx.push(tmp_size - 1);
                    j = dimension_array_expect(data, map, map.size() - 1, i + 1, current_layer + 1);
                    i = j;
                }
                /////////////////////////////////////////////////////////////////

                if data.chars().nth(i + 1).unwrap_or_default() == '{' {
                    //pair_list {}   Object
                    map.rawdata.push(Json {
                        key: tmp_key.clone(),
                        val: Valtype::Object(()),
                        layer: current_layer,
                        father_idx: current_root_idx,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[current_root_idx].child_idx.push(tmp_size - 1);
                    j = pair_list_expect(data, map, map.size() - 1, i + 1, current_layer + 1);
                    i = j; //if it is j-1,it will result in catching last nesting "}",and idx will be wrong
                    j = 0;
                }
            }
            i += 1;
        }
        return i; //letter Should Be LayerE,However,When There was a nest,It will change to next "}" or "]"
    }

    fn dimension_array_expect(
        data: &str,
        map: &mut super::json::json_struct::JsonMap,
        current_root_idx: usize,
        beginpos: usize,
        current_layer: i8,
    ) -> usize {
        //std::cout << "IN_FUNC\n";

        let mut tmp_size = 0;
        let mut is_float = false;
        let mut j: usize = 0;
        let mut i: usize = 0;
        i = beginpos;
        while data.chars().nth(i).unwrap_or_default() != ']' {
            if data.chars().nth(i).unwrap_or_default() == '['
                || data.chars().nth(i).unwrap_or_default() == ','
            {
                //buf.layer = current_layer;
                if data.chars().nth(i + 1).unwrap_or_default() == 'n'
                    && data.chars().nth(i + 2).unwrap_or_default() == 'u'
                    && data.chars().nth(i + 3).unwrap_or_default() == 'l'
                    && data.chars().nth(i + 4).unwrap_or_default() == 'l'
                {
                    map.rawdata.push(Json {
                        key: String::from(""),
                        val: Valtype::Null(String::from("null")),
                        layer: current_layer,
                        father_idx: current_root_idx,
                        child_idx: vec![],
                    });

                    tmp_size = map.size();
                    map[current_root_idx].child_idx.push(tmp_size - 1);

                    i = i + 4;
                }
                if data.chars().nth(i + 1).unwrap_or_default() == 't'
                    && data.chars().nth(i + 2).unwrap_or_default() == 'r'
                    && data.chars().nth(i + 3).unwrap_or_default() == 'u'
                    && data.chars().nth(i + 4).unwrap_or_default() == 'e'
                {
                    //bool true
                    //buf.content = true;

                    map.rawdata.push(Json {
                        key: String::from(""),
                        val: Valtype::BoolT(true),
                        layer: current_layer,
                        father_idx: current_root_idx,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[current_root_idx].child_idx.push(tmp_size - 1);

                    i = i + 4;
                }
                if data.chars().nth(i + 1).unwrap_or_default() == 'f'
                    && data.chars().nth(i + 2).unwrap_or_default() == 'a'
                    && data.chars().nth(i + 3).unwrap_or_default() == 'l'
                    && data.chars().nth(i + 4).unwrap_or_default() == 's'
                    && data.chars().nth(i + 5).unwrap_or_default() == 'e'
                {
                    //bool false

                    map.rawdata.push(Json {
                        key: String::from(""),
                        val: Valtype::BoolT(false),
                        layer: current_layer,
                        father_idx: current_root_idx,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[current_root_idx].child_idx.push(tmp_size - 1);

                    i = i + 5;
                }
                if data.chars().nth(i + 1).unwrap_or_default() == '"' {
                    //str
                    j = i + 2;
                    while (data.chars().nth(j + 1).unwrap_or_default() != ','
                        || data.chars().nth(j).unwrap_or_default() != '"')
                        && (data.chars().nth(j + 1).unwrap_or_default() != '}'
                            || data.chars().nth(j).unwrap_or_default() != '"')
                    {
                        j += 1;
                    }
                    map.rawdata.push(Json {
                        key: String::from(""),
                        val: Valtype::Str(data[i + 2..j].to_string()),
                        layer: current_layer,
                        father_idx: current_root_idx,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[current_root_idx].child_idx.push(tmp_size - 1);

                    i = j - 1;
                    j = 0;
                }
                if data.chars().nth(i + 1).unwrap_or_default().is_numeric()
                    || (data.chars().nth(i + 1).unwrap_or_default() == '-'
                        && data.chars().nth(i + 2).unwrap_or_default().is_numeric())
                {
                    //digit_all
                    j = i + 1;
                    while data.chars().nth(j).unwrap_or_default() != ','
                        && data.chars().nth(j).unwrap_or_default() != '}'
                    {
                        if data.chars().nth(j).unwrap_or_default() == '.' {
                            is_float = true;
                        }
                        j += 1;
                    }
                    if is_float == true {
                        map.rawdata.push(Json {
                            key: String::from(""),
                            val: Valtype::Float(
                                data[i + 2..j]
                                    .to_string()
                                    .parse::<f64>()
                                    .unwrap_or_default(),
                            ),
                            layer: current_layer,
                            father_idx: current_root_idx,
                            child_idx: vec![],
                        });
                    }
                    if is_float == false {
                        map.rawdata.push(Json {
                            key: String::from(""),
                            val: Valtype::Number(
                                data[i + 2..j]
                                    .to_string()
                                    .parse::<i64>()
                                    .unwrap_or_default(),
                            ),
                            layer: current_layer,
                            father_idx: current_root_idx,
                            child_idx: vec![],
                        });
                    }
                    is_float = false;

                    tmp_size = map.size();
                    map[current_root_idx].child_idx.push(tmp_size - 1);

                    i = j - 1;
                    j = 0;
                }

                /////////////////////////////////////////////////////////////////////////////////
                if data.chars().nth(i + 1).unwrap_or_default() == '['
                    && data.chars().nth(i + 2).unwrap_or_default() == ']'
                {
                    //void list

                    map.rawdata.push(Json {
                        key: String::from(""),
                        val: Valtype::ArrayVoid(String::from("[]")),
                        layer: current_layer,
                        father_idx: current_root_idx,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[current_root_idx].child_idx.push(tmp_size - 1);

                    //i = i + 2;
                    i = i + 2;
                    //std::cout << data[i - 1] << "("  << data[i] << ")" << "   Void\n";
                }

                if data.chars().nth(i + 1).unwrap_or_default() == '[' {
                    //Dimension List

                    map.rawdata.push(Json {
                        key: String::from(""),
                        val: Valtype::Array(()),
                        layer: current_layer,
                        father_idx: current_root_idx,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[current_root_idx].child_idx.push(tmp_size - 1);
                    j = dimension_array_expect(data, map, map.size() - 1, i + 1, current_layer + 1);
                    i = j;
                }

                /////////////////////////////////////////////////////////////////////////////

                if data.chars().nth(i + 1).unwrap_or_default() == '{'
                    && data.chars().nth(i + 2).unwrap_or_default() == '}'
                {
                    //pair_list_VOID {}   Object

                    map.rawdata.push(Json {
                        key: String::from(""),
                        val: Valtype::ObjectVoid(String::from("{}")),
                        layer: current_layer,
                        father_idx: current_root_idx,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[current_root_idx].child_idx.push(tmp_size - 1);

                    i = i + 2;
                }

                if data.chars().nth(i + 1).unwrap_or_default() == '{' {
                    //pair_list {}   Object///////////////////

                    map.rawdata.push(Json {
                        key: String::from(""),
                        val: Valtype::Object(()),
                        layer: current_layer,
                        father_idx: current_root_idx,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[current_root_idx].child_idx.push(tmp_size - 1);
                    j = pair_list_expect(data, map, map.size() - 1, i + 1, current_layer + 1);

                    i = j - 1; //LayerE
                    j = 0;
                }
            }
            i += 1;
        }

        return i;
    }

    pub fn json_parse(map: &mut super::json::json_struct::JsonMap, data: &str) -> usize {
        let mut tmp_key = String::from("");
        let mut tmp_size = 0;
        let mut is_float = false;
        let mut j: usize = 0;
        let mut k: usize = 0;
        let mut t: usize = 0;
        let mut i: usize = 0;
        println!("StartFunc");
        while i <= (data.len() - 1) {
            //#pragma Debug
            //std::cout << data[i];
            //println!("Loopin");
            if data.chars().nth(i).unwrap_or_default() == ':' {
                //println!("ConS");
                t = i;
                while data.chars().nth(t).unwrap_or_default() != ','
                    && data.chars().nth(t).unwrap_or_default() != '{'
                {
                    t -= 1;
                }
                k = t + 1;

                while data.chars().nth(k).unwrap_or_default() != ':' {
                    if data.chars().nth(k).unwrap_or_default() != '"' {
                        tmp_key.push(data.chars().nth(k).unwrap_or_default());
                    }
                    k += 1
                }

                if data[i + 1..i + 4] == *"null" {
                    map.rawdata.push(Json {
                        key: tmp_key.clone(),
                        val: Valtype::Null(String::from("null")),
                        layer: 1,
                        father_idx: 0,
                        child_idx: vec![],
                    });
                    print!("Loopin\n");
                    tmp_size = map.size();
                    map[0].child_idx.push(tmp_size - 1);
                }
                /*
                data.chars().nth(i + 1).unwrap_or_default() == 't'
                    && data.chars().nth(i + 2).unwrap_or_default() == 'r'
                    && data.chars().nth(i + 3).unwrap_or_default() == 'u'
                    && data.chars().nth(i + 4).unwrap_or_default() == 'e' */
                if data[i + 1..i + 4] == *"true" {
                    //bool true

                    map.rawdata.push(Json {
                        key: tmp_key.clone(),
                        val: Valtype::BoolT(true),
                        layer: 1,
                        father_idx: 0,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[0].child_idx.push(tmp_size - 1);
                }
                if data.chars().nth(i + 1).unwrap_or_default() == 'f'
                    && data.chars().nth(i + 2).unwrap_or_default() == 'a'
                    && data.chars().nth(i + 3).unwrap_or_default() == 'l'
                    && data.chars().nth(i + 4).unwrap_or_default() == 's'
                    && data.chars().nth(i + 5).unwrap_or_default() == 'e'
                {
                    //bool false

                    map.rawdata.push(Json {
                        key: tmp_key.clone(),
                        val: Valtype::BoolT(false),
                        layer: 1,
                        father_idx: 0,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[0].child_idx.push(tmp_size - 1);
                }
                if data.chars().nth(i + 1).unwrap_or_default() == '"' {
                    //str
                    j = i + 2;

                    while (data.chars().nth(j + 1).unwrap_or_default() != ','
                        || data.chars().nth(j).unwrap_or_default() != '"')
                        && (data.chars().nth(j + 1).unwrap_or_default() != '}'
                            || data.chars().nth(j).unwrap_or_default() != '"')
                    {
                        j += 1;
                    }

                    map.rawdata.push(Json {
                        key: tmp_key.clone(),
                        val: Valtype::Str(data[i + 2..j].to_string()),
                        layer: 1,
                        father_idx: 0,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[0].child_idx.push(tmp_size - 1);

                    //std::cout << data[j] << "  ssssssss\n";
                    i = j - 1;
                    j = 0;
                }
                if data.chars().nth(i + 1).unwrap_or_default().is_numeric()
                    || (data.chars().nth(i + 1).unwrap_or_default() == '-'
                        && data.chars().nth(i + 2).unwrap_or_default().is_numeric())
                {
                    //digit_all
                    j = i + 1;
                    while data.chars().nth(j).unwrap_or_default() != ','
                        && data.chars().nth(j).unwrap_or_default() != '}'
                    {
                        if data.chars().nth(j).unwrap_or_default() == '.' {
                            is_float = true;
                        }
                        j += 1;
                    }
                    if is_float == true {
                        map.rawdata.push(Json {
                            key: tmp_key.clone(),
                            val: Valtype::Float(
                                data[i + 2..j]
                                    .to_string()
                                    .parse::<f64>()
                                    .unwrap_or_default(),
                            ),
                            layer: 1,
                            father_idx: 0,
                            child_idx: vec![],
                        });
                    }
                    if is_float == false {
                        map.rawdata.push(Json {
                            key: tmp_key.clone(),
                            val: Valtype::Number(
                                data[i + 2..j]
                                    .to_string()
                                    .parse::<i64>()
                                    .unwrap_or_default(),
                            ),
                            layer: 1,
                            father_idx: 0,
                            child_idx: vec![],
                        });
                    }
                    is_float = false;

                    tmp_size = map.size();
                    map[0].child_idx.push(tmp_size - 1);

                    i = j - 1;
                    j = 0;
                }
                /////////////////////////////////////////////////////////////////////////////////
                if data.chars().nth(i + 1).unwrap_or_default() == '['
                    && data.chars().nth(i + 2).unwrap_or_default() == ']'
                {
                    //void list
                    println!("Void List");
                    map.rawdata.push(Json {
                        key: tmp_key.clone(),
                        val: Valtype::ArrayVoid(String::from("[]")),
                        layer: 1,
                        father_idx: 0,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[0].child_idx.push(tmp_size - 1);
                    i = i + 2;
                }
                if data.chars().nth(i + 1).unwrap_or_default() == '{'
                    && data.chars().nth(i + 2).unwrap_or_default() == '}'
                {
                    //void list
                    map.rawdata.push(Json {
                        key: tmp_key.clone(),
                        val: Valtype::ObjectVoid(String::from("{}")),
                        layer: 1,
                        father_idx: 0,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[0].child_idx.push(tmp_size - 1);
                    i = i + 2;
                }

                if data.chars().nth(i + 1).unwrap_or_default() == '[' {
                    //Dimension List

                    map.rawdata.push(Json {
                        key: tmp_key.clone(),
                        val: Valtype::Array(()),
                        layer: 1,
                        father_idx: 0,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[0].child_idx.push(tmp_size - 1);
                    j = dimension_array_expect(data, map, map.size() - 1, i + 1, 1);
                    //std::cout << "Dimen:" << data[i + 1] << "\n";

                    i = j;
                }

                /////////////////////////////////////////////////////////////////////////////
                if data.chars().nth(i + 1).unwrap_or_default() == '{' {
                    //pair_list {}   Object

                    map.rawdata.push(Json {
                        key: tmp_key.clone(),
                        val: Valtype::Object(()),
                        layer: 1,
                        father_idx: 0,
                        child_idx: vec![],
                    });
                    tmp_size = map.size();
                    map[0].child_idx.push(tmp_size - 1);
                    j = pair_list_expect(data, map, map.size() - 1, i + 1, 1);

                    i = j - 1;
                    //std::cout << "                                       " << data[i] << "\n";
                    j = 0;
                }
            }
            i += 1;
        }
        return 1;
    }
}
