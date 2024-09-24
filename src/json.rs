pub mod json_struct {

    #![allow(dead_code)]

    use std::{
        any::{type_name, type_name_of_val, Any, TypeId},
        fmt::write,
    };

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /*
    #[derive(Clone, Debug)]
    pub enum Valtype {
        Str(String),
        BoolT(bool),
        Number(i64),
        Float(f64),
        Null(String),
        ObjectVoid(String),
        ArrayVoid(String),
        Array(()),
        Object(()),
    }
    */
    #[derive(Clone, Debug)]
    pub enum DataType {
        Str = 1,
        Int = 2,
        Float = 3,
        Null = 4,
        ObjectVoid = 5,
        ArrayVoid = 6,
        Array = 7,
        Object = 8,
        BoolT = 9,
        Unset = 0,
    }
    #[derive(Debug, Clone)] //用Derive宏可以快速为类或者enum添加对应接口
    pub struct Json {
        pub key: String,
        pub val: String,
        pub value_type: DataType,
        pub layer: i8,
        pub father_idx: usize,
        pub child_idx: Vec<usize>,
    }
    impl std::fmt::Display for Json {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "key:{}    val:{:?}", self.key, self.val)
        }
    }
    impl Json {
        fn from(&mut self, key_v: String, val_v: String) -> &Json {
            self.key = key_v;
            self.val = val_v;
            return self;
        }
        pub fn get_value<T: Sized + 'static>(&self) -> TypeId::of<T> {}
        pub fn clear(&mut self) {
            self.key = String::from("");
            self.val = String::from("");
            self.value_type = DataType::Unset;
            self.layer = 0;
            self.father_idx = 0;
            self.child_idx.clear();
        }
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    #[derive(Clone, Debug)]
    pub struct JsonMap {
        pub rawdata: Vec<Json>,
    }

    #[repr(i64)] //make it a i64 type enum,so we can use binary comparison
    pub enum SearchMethod {
        Bfs = 2,
        Dfs = 3,
        Directly = 1,
    }

    impl std::ops::Index<usize> for JsonMap {
        type Output = Json;
        fn index(&self, index: usize) -> &Self::Output {
            return &self.rawdata[index];
        }
    }

    impl std::ops::IndexMut<usize> for JsonMap {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            return &mut self.rawdata[index];
        }
    }

    impl JsonMap {
        pub fn emplace_back(&mut self, obj: &Json) {
            self.rawdata.push(obj.clone());
        }
        pub fn size(&self) -> usize {
            self.rawdata.len()
        }
        fn func_bfs(&self, layer: i8, key_name: &String, currentroot: usize) -> usize {
            //广度搜索
            //initial c_layer must be 0
            //because 0 is a general father node,so we dont have to do more job than this
            let mut result: usize = 0;
            for i in 0..self.rawdata[currentroot].child_idx.len() {
                if self.rawdata[self.rawdata[currentroot].child_idx[i]].key == *key_name
                    && self.rawdata[self.rawdata[currentroot].child_idx[i]].layer == layer
                {
                    //here,we have all the children of current_root checked
                    result = i;
                    break;
                } else {
                    result = self.func_bfs(layer, key_name, self.rawdata[currentroot].child_idx[i]);
                } //here checking for children of children
            }
            return result;
        }
        fn func_dfs(&self, layer: i8, key_name: &String, currentroot: usize) -> usize {
            //current_root initial value must be 0
            let mut result: usize = 0;
            let mut current_opt_idx = currentroot;
            let mut current_father_idx: usize = 0;
            let mut child = 0;
            while result == 0 {
                while !self.rawdata[current_opt_idx].child_idx.is_empty() {
                    current_father_idx = current_opt_idx;
                    current_opt_idx = self.rawdata[current_opt_idx].child_idx[child];
                    if self.rawdata[current_opt_idx].key == *key_name
                        && self.rawdata[current_opt_idx].layer == layer
                    {
                        result = current_opt_idx;
                        break;
                    }
                }
                child += 1;
                if self.rawdata[current_opt_idx].child_idx.is_empty() {
                    current_opt_idx = current_father_idx;
                    current_father_idx = self.rawdata[current_opt_idx].father_idx;
                }
            }
            return result;
        }
        pub fn get_idx(&self, method: SearchMethod, key_name: String, layer: i8) -> usize {
            //externel method,get index of specific name struct
            let mut result: usize = 0;
            match method {
                SearchMethod::Bfs => {
                    result = self.func_bfs(layer, &key_name, 0);
                }
                SearchMethod::Dfs => {}
                SearchMethod::Directly => {
                    for idx in 0..self.rawdata.len() {
                        if self.rawdata[idx].key == key_name && self.rawdata[idx].layer == layer {
                            result = idx;
                        }
                    }
                }
            }
            return result;
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        fn add_at_back() -> () {}
        pub fn new() -> JsonMap {
            let root = Json {
                key: String::from(""),
                val: String::from(""),
                value_type: DataType::Unset,
                layer: 0,
                father_idx: 0,
                child_idx: vec![],
            };

            return JsonMap {
                rawdata: vec![root],
            };
        }
        fn add_as_child<T: Sized + 'static + ToString>(
            &mut self,
            key_add: String,
            val_add: T,
            layer_add: i8,
            key_father: String,
            layer_father: i8,
        ) -> () {
            let index = self.get_idx(SearchMethod::Directly, key_father, layer_father);
            let child_listlen = self.rawdata[index].child_idx.len();
            let string_val = val_add.to_string();
            let tmp = Json {
                key: key_add,
                val: string_val.clone(),
                value_type: match type_name::<T>() {
                    "bool" => DataType::BoolT,
                    "f64" => DataType::Float,
                    "i64" => DataType::Int,
                    "String" => match &string_val[..string_val.len()] {
                        "[]" => DataType::ArrayVoid,
                        "{}" => DataType::ObjectVoid,
                        "null" => DataType::Null,
                        _ => match &string_val[0..0] {
                            "\"" => DataType::ArrayVoid,
                            _ => DataType::Unset,
                        },
                    },
                    _ => DataType::Unset,
                },
                layer: layer_add,
                father_idx: index,
                child_idx: vec![],
            };
            self.rawdata[index]
                .child_idx
                .push(index + child_listlen + 1);
            self.rawdata.insert(index + child_listlen + 1, tmp.clone());
        }
    }
}
