pub use crate::json;
pub mod json_parse {
    fn is_numeric(chara: &str) -> bool {
        #[cfg(debug_assertions)]
        println!("In IsNumeric");
        match chara {
            "0" => true,
            "1" => true,
            "2" => true,
            "3" => true,
            "4" => true,
            "5" => true,
            "6" => true,
            "7" => true,
            "8" => true,
            "9" => true,
            _ => {
                return false;
            }
        }
    }
    use std::vec;

    use super::json::json_struct::{DataType, Json, JsonMap};
    ///////////////////////////////////////////////////////////////////////////
    pub fn match_null(
        json_data: &str,
        tmp_key: &mut String,
        map: &mut JsonMap,
        cons: usize, //cons
        root: usize,
        layer: i8,
    ) -> usize {
        #[cfg(debug_assertions)]
        println!("In Null");
        let mut ret = cons;
        match &json_data[cons + 1..=cons + 4] {
            "null" => {
                let buf = Json {
                    key: tmp_key.clone(),
                    val: String::from("null"),
                    value_type: DataType::Null,
                    father_idx: root,
                    layer: layer,
                    child_idx: vec![],
                };
                map.rawdata.push(buf.clone());
                let latest_size = map.size() - 1;
                map.rawdata[root].child_idx.push(latest_size);
                ret += 4;
            }
            _ => {}
        }
        return ret;
    }

    pub fn match_digit(
        json_data: &str,
        tmp_key: &mut String,
        map: &mut JsonMap,
        cons: usize, //cons
        root: usize,
        layer: i8,
    ) -> usize {
        #[cfg(debug_assertions)]
        println!("In Digit");
        let mut ret = cons;
        let mut is_real_num = true;
        let mut is_float = false;
        let mut k = cons + 1;
        while &json_data[k..=k] != "," && &json_data[k..=k] != "]" && &json_data[k..=k] != "}" {
            if &json_data[k..=k] == "." {
                is_float = true;
            }
            is_real_num = is_real_num && is_numeric(&json_data[k..k]);
            k += 1;
        }
        ret = k - 1;
        assert!(!is_real_num, "Not Valid Number");
        if is_float {
            /*
            let number_f64 = json_data[cons + 1..k - 1]
                .parse::<f64>()
                .unwrap_or_default();
            */

            let buf = Json {
                key: tmp_key.clone(),
                val: json_data[cons + 1..k - 1].to_string(),
                value_type: DataType::Float,
                father_idx: root,
                layer: layer,
                child_idx: vec![],
            };
            map.rawdata.push(buf.clone());
            let latest_size = map.size() - 1;
            map.rawdata[root].child_idx.push(latest_size);
        } else {
            /*
            let number_i64 = json_data[cons + 1..k - 1]
                .parse::<i64>()
                .unwrap_or_default();
            */

            let buf = Json {
                key: tmp_key.clone(),
                val: json_data[cons + 1..k - 1].to_string(),
                value_type: DataType::Int,
                father_idx: root,
                layer: layer,
                child_idx: vec![],
            };
            map.rawdata.push(buf.clone());
            let latest_size = map.size() - 1;
            map.rawdata[root].child_idx.push(latest_size);
        }
        return ret;
    }

    pub fn match_string(
        json_data: &str,
        tmp_key: &mut String,
        map: &mut JsonMap,
        cons: usize, //cons
        root: usize,
        layer: i8,
    ) -> usize {
        #[cfg(debug_assertions)]
        println!("In String");
        let mut ret = cons;
        match &json_data[cons + 1..=cons + 1] {
            "\"" => {
                let mut end = 0;
                for k in cons + 1..json_data.len() {
                    match &json_data[k..=k] {
                        "\"" => match &json_data[k + 1..k + 1] {
                            "]" => {
                                end = k;
                                break;
                            }
                            "}" => {
                                end = k;
                                break;
                            }
                            "," => {
                                end = k;
                                break;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }

                let buf = Json {
                    key: tmp_key.clone(),
                    val: String::from(&json_data[cons + 1..end - 1]),
                    value_type: DataType::Str,
                    father_idx: root,
                    layer: layer,
                    child_idx: vec![],
                };
                map.rawdata.push(buf.clone());
                let latest_size = map.size() - 1;
                map.rawdata[root].child_idx.push(latest_size);
                ret = end;
            }
            _ => {}
        }
        return ret;
    }

    pub fn match_bool(
        json_data: &str,
        tmp_key: &mut String,
        map: &mut JsonMap,
        cons: usize, //cons
        root: usize,
        layer: i8,
    ) -> usize {
        #[cfg(debug_assertions)]
        println!("In Bool");
        let mut ret = cons;
        match &json_data[cons + 1..=cons + 4] {
            "true" => {
                #[cfg(debug_assertions)]
                println!("matched bool true");
                let buf = Json {
                    key: tmp_key.clone(),
                    val: true.to_string(),
                    value_type: DataType::BoolT,
                    father_idx: root,
                    layer: layer,
                    child_idx: vec![],
                };
                map.rawdata.push(buf.clone());
                let latest_size = map.size() - 1;
                map.rawdata[root].child_idx.push(latest_size);
                ret += 4;
            }
            _ => {
                //#[cfg(debug_assertions)]
                //println!("unmatch bool true");
            }
        }
        match &json_data[cons + 1..=cons + 5] {
            "false" => {
                #[cfg(debug_assertions)]
                println!("matched bool false");
                let buf = Json {
                    key: tmp_key.clone(),
                    val: false.to_string(),
                    value_type: DataType::BoolT,
                    father_idx: root,
                    layer: layer,
                    child_idx: vec![],
                };
                map.rawdata.push(buf.clone());
                let latest_size = map.size() - 1;
                map.rawdata[root].child_idx.push(latest_size);
                ret += 5;
            }
            _ => {
                //#[cfg(debug_assertions)]
                //println!("unmatch bool false");
            }
        }
        return ret;
    }

    ///////////////////////////////////////////////////////////////////////////

    pub fn match_array(
        json_data: &str,
        tmp_key: &mut String,
        map: &mut JsonMap,
        cons: usize, //cons
        root: usize,
        layer: i8,
    ) -> usize {
        #[cfg(debug_assertions)]
        println!("In Array");
        let mut ret = cons;
        match &json_data[cons + 1..cons + 1] {
            "[" => {
                if &json_data[cons + 2..=cons + 2] == "]" {
                    let buf = Json {
                        key: tmp_key.clone(),
                        val: String::from("[]"),
                        value_type: DataType::ArrayVoid,
                        father_idx: root,
                        layer: layer,
                        child_idx: vec![],
                    };
                    map.rawdata.push(buf.clone());
                    let latest_size = map.size() - 1;
                    map.rawdata[root].child_idx.push(latest_size);
                    ret += 2;
                } else {
                    let buf = Json {
                        key: tmp_key.clone(),
                        val: String::from(""),
                        value_type: DataType::Array,
                        father_idx: root,
                        layer: layer,
                        child_idx: vec![],
                    };
                    map.rawdata.push(buf.clone());
                    let latest_size = map.size() - 1;
                    map.rawdata[root].child_idx.push(latest_size);
                    
                }

                tmp_key.clear();
            }

            _ => {}
        }
        return ret;
    }

    pub fn match_object(
        json_data: &str,
        tmp_key: &mut String,
        map: &mut JsonMap,
        cons: usize, //cons
        root: usize,
        layer: i8,
    ) -> usize {
        #[cfg(debug_assertions)]
        println!("In Object");
        let mut ret = cons;

        match &json_data[cons + 1..cons + 1] {
            "{" => {
                if &json_data[cons + 2..=cons + 2] == "}" {
                    let buf = Json {
                        key: tmp_key.clone(),
                        val: String::from("{}"),
                        value_type: DataType::ObjectVoid,
                        father_idx: root,
                        layer: layer,
                        child_idx: vec![],
                    };
                    map.rawdata.push(buf.clone());
                    let latest_size = map.size() - 1;
                    map.rawdata[root].child_idx.push(latest_size);
                    ret += 2;
                } else {
                    let mut tmp_vec: Vec<usize> = vec![];
                    let mut object = 1;
                    let mut k = cons + 1;
                    let buf = Json {
                        key: tmp_key.clone(),
                        val: String::from(""),
                        value_type: DataType::Object,
                        father_idx: root,
                        layer: layer,
                        child_idx: vec![],
                    };
                    map.rawdata.push(buf.clone());
                    let latest_size = map.size() - 1;
                    map.rawdata[root].child_idx.push(latest_size);
                    let current_root = map.size() - 1;
                    while object != 0 {
                        match &json_data[k..=k + 1] {
                            ":{" => {
                                k = match_object(json_data, tmp_key, map, k, current_root, layer + 1);
                                k+=1;
                                ///////////////////////////////
                            }
                            ":[" => {}
                            _ => {}
                        }

                        match &json_data[k..k] {
                            ":" => {
                                let mut idx = k - 2;
                                while &json_data[idx..=idx] != "\"" {
                                    idx -= 1;
                                }
                                tmp_key.push_str(&json_data[idx + 1..k - 1]); //push_key_string
                                k = match_bool(json_data, tmp_key, map, k, current_root, layer);
                                k = match_null(json_data, tmp_key, map, k, current_root, layer);
                                k = match_string(json_data, tmp_key, map, k, current_root, layer);
                                k = match_digit(json_data, tmp_key, map, k, current_root, layer);
                                tmp_key.clear();
                            }
                            "}" => {
                                ret = k;
                            }
                            _ => {}
                        }
                        k += 1;
                    }
                }

                tmp_key.clear();
            }
            _ => {}
        }
        return ret;
    }

    pub fn json_parse(json_data: &str, map: &mut JsonMap) -> () {
        let mut tmp_key = String::new();
        let mut i = 0;

        while i < json_data.len() {
            match &json_data[i..=i] {
                ":" => {
                    //cfg(debug_assertions)]
                    //println!("matched cons");
                    let mut idx = i - 2;
                    while &json_data[idx..=idx] != "\"" {
                        //#[cfg(debug_assertions)]
                        //println!("{}", &json_data[idx..=idx]);
                        idx -= 1;
                    }
                    tmp_key.push_str(&json_data[idx + 1..i - 1]); //push_key_string
                    i = match_bool(json_data, &mut tmp_key, map, i, 0, 0);
                    i = match_string(json_data, &mut tmp_key, map, i, 0, 0);
                    i = match_null(json_data, &mut tmp_key, map, i, 0, 0);
                    i = match_digit(json_data, &mut tmp_key, map, i, 0, 0);

                    tmp_key.clear();
                }
                _ => {
                    //#[cfg(debug_assertions)]
                    //println!("unmatch cons");
                }
            }
            i += 1;
        }
    }
}
