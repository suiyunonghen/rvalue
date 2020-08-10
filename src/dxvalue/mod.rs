pub mod json;

use std::fmt::{Debug, Formatter, Result, Display};
use std::cell;
use std::cell::{RefCell, Ref};
use std::str::FromStr;
use std::any::TypeId;
use std::borrow::Borrow;
use std::ops::Div;

#[derive(Debug)]
pub struct Key_Value{
    Key:    String,
    Value:  cell::RefCell<DxValue>
}

impl Key_Value{
    pub fn new_value(key: &str,v: DxValue) -> Self{
        Key_Value{
            Key:    key.to_string(),
            Value:  cell::RefCell::new(v),
        }
    }

    pub fn new_int(key: &str,v: isize) -> Self{
        Key_Value::new_value(key,DxValue::Int(v))
    }

    pub fn new_int32(key: &str,v: i32) -> Self{
        Key_Value::new_value(key,DxValue::Int32(v))
    }

    pub fn new_int64(key: &str,v: i64) -> Self{
        Key_Value::new_value(key,DxValue::Int64(v))
    }

    pub fn new_string(key: &str,v: &str) -> Self{
        Key_Value::new_value(key,DxValue::String(v.to_string()))
    }

    pub fn new_bool(key: &str,v: bool) -> Self{
        Key_Value::new_value(key,DxValue::Boolean(v))
    }

    pub fn new_Float(key: &str,v: f32) -> Self{
        Key_Value::new_value(key,DxValue::Float(v))
    }

    pub fn new_Double(key: &str,v: f64) -> Self{
        Key_Value::new_value(key,DxValue::Double(v))
    }

}

impl Display for Key_Value{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f,"\"{}\": {}",self.Key,self.Value.borrow())
    }
}


#[derive(Debug)]
pub enum  DxValue{
    Object(cell::RefCell<Vec<Key_Value>>),
    Array(cell::RefCell<Vec<DxValue>>),
    String(String),
    Int(isize),
    Int32(i32),
    Int64(i64),
    Float(f32),
    Double(f64),
    Boolean(bool),
    None
}
impl Display for DxValue{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            DxValue::String(T)=> write!(f,"\"{}\"",T),
            DxValue::Int(T) => write!(f,"{}",T),
            DxValue::Int32(T)=> write!(f,"{}",T),
            DxValue::Int64(T)=> write!(f,"{}",T),
            DxValue::Float(T)=> write!(f,"{:.2}",T),
            DxValue::Double(T)=> write!(f,"{:.2}",T),
            DxValue::Boolean(T)=> write!(f,"{}",T),
            DxValue::Object(T) =>{
                write!(f,"{}","{")?; //默认不格式化
                let vecObj = &(*T.borrow());
                let mut isfirst = true;
                for obj in vecObj{
                    if isfirst{
                        write!(f,"{}",obj)?;
                        isfirst = false;
                    }else{
                        write!(f,",{}",obj)?;
                    }
                }
                write!(f,"{}","}")
            },
            DxValue::Array(T) =>{
                write!(f,"[")?;
                let vecArr = &(*T.borrow());
                let mut isfirst = true;
                for v in vecArr{
                    if isfirst{
                        write!(f,"{}",v)?;
                        isfirst = false;
                    }else{
                        write!(f,",{}",v)?;
                    }
                }
                write!(f,"]")
            },
            DxValue::None => write!(f,"Null"),
        }
    }
}

impl DxValue {
    /// 创建一个KV结构
    /// # Examples
    /// ```
    /// use rust_value::dxvalue::{Key_Value,DxValue};
    /// let objMap = DxValue::newObject();
    /// objMap.setKeyString("name","不得闲");
    /// ```
    pub fn newObject() -> DxValue{
        DxValue::Object(RefCell::new(Vec::new()))
    }

    pub fn newArray() -> DxValue{
        DxValue::Array(RefCell::new(Vec::new()))
    }

    pub fn as_String(&self) -> String{
        format!("{}",self)
    }

    /*pub fn iter(&self) -> Iter {
        Iter{curindex:0,ptr: Rc::new(self)}
    }*/

    pub fn as_int(&self)->isize{
        match self {
            DxValue::Int(T)=> return *T,
            DxValue::Int32(T)=>return *T as isize,
            DxValue::String(T)=> return (*T).parse::<isize>().unwrap_or(0),
            DxValue::Boolean(T)=> {
                if *T{ return 1;}
            },
            DxValue::Double(T) => return *T as isize,
            DxValue::Float(T) => return *T as isize,
            _=>{}
        }
        0
    }

    pub fn as_value<T: FromStr>(&self,defValue: T) -> T{
        //判定一下T是什么类型
        format!("{}",self).parse::<T>().unwrap_or(defValue)
    }

    pub fn as_bool(&self)-> bool{
        match self {
            DxValue::Boolean(t)=> *t,
            DxValue::Int64(t) => *t>0,
            DxValue::Int(t) => *t>0,
            DxValue::Int32(t) => *t>0,
            DxValue::Float(t) => *t>0.0,
            DxValue::Double(t) => *t>0.0,
            DxValue::String(t)=>{
                t.to_lowercase().eq("true")
            }
            _=>false,
        }
    }

    pub fn as_float(&self) -> f32{
        match self {
            DxValue::Int(T)=> return *T as f32,
            DxValue::Int32(T)=>return *T as f32,
            DxValue::String(T)=> return (*T).parse::<f32>().unwrap_or(0.0),
            DxValue::Boolean(T)=> {
                if *T{ return 1.0;}
            },
            DxValue::Double(T) => return *T as f32,
            DxValue::Float(T) => return *T,
            _=>{}
        }
        0.0
    }

    pub fn as_double(&self) -> f64{
        match self {
            DxValue::Int(T)=> return *T as f64,
            DxValue::Int32(T)=>return *T as f64,
            DxValue::String(T)=> return (*T).parse::<f64>().unwrap_or(0 as f64),
            DxValue::Boolean(T)=> {
                if *T{ return 1 as f64;}
            },
            DxValue::Double(T) => return *T,
            DxValue::Float(T) => return *T as f64,
            _=>{}
        }
        0 as f64
    }

    pub fn setString(&mut self,value: &str){
        match self {
            DxValue::String(T)=>{
                *T = value.to_string();
            },
            _=>{
                *self = DxValue::String(value.to_string())
            }
        }
    }

    pub fn setInt(&mut self,value: isize){
        match self {
            DxValue::Int(T)=>{
                *T = value
            },
            DxValue::Int32(T) =>{
                *T = value as i32
            },
            _=>{
                *self = DxValue::Int(value)
            }
        }
    }

    pub fn setBool(&mut self,value: bool){
        match self {
            DxValue::Boolean(T)=>{
                *T = value
            },
            _=>{
                *self = DxValue::Boolean(value)
            }
        }
    }

    pub fn setKeyString(&mut self,name: &str,value: &str){
        match self {
            DxValue::Object(T) =>{
               /* let mut temp = T.borrow_mut();
                let mut index = 0;
                while index < temp.len(){
                    if let Some(obj) = temp.get_mut(index){
                        if obj.Key == name{
                            let mut v = obj.Value.borrow_mut();
                            *v = DxValue::String(value.to_string());
                            return
                        }
                    }
                    index += 1;
                }
                temp.push(Key_Value::new_value(name,DxValue::String(value.to_string())));*/
                let vecobj = T.get_mut();
                for obj in vecobj{
                    if obj.Key == name{
                        let mut v = obj.Value.borrow_mut();
                        *v = DxValue::String(value.to_string());
                        return
                    }
                }
                T.borrow_mut().push(Key_Value::new_value(name,DxValue::String(value.to_string())));
            },
            DxValue::Array(T)=>{
                if let Ok(index) = name.parse::<usize>(){
                    let mut vecobj = T.get_mut();
                    if index < 0{
                        vecobj.insert(0,DxValue::String(value.to_string()));
                        return;
                    }
                    if index > vecobj.len(){
                        vecobj.push(DxValue::String(value.to_string()));
                        return;
                    }
                    vecobj[index] = DxValue::String(value.to_string());
                }
            },
            _=>{

            }
        }
    }

    pub fn setIndexValue(&mut self,idx: isize,value: Self){
        match self {
            DxValue::Object(T) =>{
                let mut vecobj = T.get_mut();
                if idx < 0 || idx as usize >= vecobj.len(){
                    return;
                }
                if let Some(obj) = vecobj.get_mut(idx as usize){
                    let mut v = obj.Value.borrow_mut();
                    *v = value;
                    return;
                }
            },
            DxValue::Array(T)=>{
                let vecobj = T.get_mut();
                if idx < 0{
                    vecobj.insert(0,value);
                    return;
                }
                if idx as usize >= vecobj.len(){
                    vecobj.push(value);
                    return;
                }
                vecobj[idx as usize] = value;
            },
            _=>{

            }
        }
    }

    pub fn setKeyValue(& mut  self,name: &str,value: Self){
        match self {
            DxValue::Object(T) =>{
                let mut vecobj = T.get_mut();
                for obj in vecobj{
                    if obj.Key == name{
                        let mut v = obj.Value.borrow_mut();
                        *v = value;
                        return
                    }
                }
                T.borrow_mut().push(Key_Value::new_value(name,value));
            },
            DxValue::Array(T)=>{
                if let Ok(index) = name.parse::<usize>(){
                    let vecobj = T.get_mut();
                    if index < 0{
                        vecobj.insert(0,value);
                        return;
                    }
                    if index > vecobj.len(){
                        vecobj.push(value);
                        return;
                    }
                    vecobj[index] = value;
                }
            },
            _=>{

            }
        }
    }

    pub fn len(&self)->usize{
        match self {
            DxValue::Object(t)=>{
                return t.borrow().len();
            },
            DxValue::Array(t)=>{
                return t.borrow().len();
            },
            _=>(),
        }
        0
    }

    pub fn string_byName(&self,name: &str,defValue: &str) -> String{
        match self {
            DxValue::Object(T) =>{
                let vec = &(*T.borrow());
                for i in 0..vec.len(){
                    let v = vec[i].borrow();
                    if v.Key == name{
                        return v.Value.borrow().as_String();
                    }
                }
            },
            DxValue::Array(T) =>{
                if let Ok(index) = name.parse::<usize>(){
                    let arr = &(*T.borrow());
                    if index >= 0 && index < arr.len(){
                        return arr[index].borrow().as_String();
                    }
                }
            },
            _=>(),
        }
        defValue.to_string()
    }

    pub fn int_byName(&self,name: &str,defValue: isize) -> isize{
        match self {
            DxValue::Object(T) =>{
                let vec = &(*T.borrow());
                for i in 0..vec.len(){
                    let obj = vec[i].borrow();
                    if obj.Key == name{
                        match &(*obj.Value.borrow()) {
                            DxValue::Int32(t) => return *t  as isize,
                            DxValue::Int(t) => return *t,
                            DxValue::Int64(t) => return *t as isize,
                            DxValue::String(t) => return (*t).parse::<isize>().unwrap_or(defValue),
                            DxValue::Boolean(t) =>{
                                if *t {return 1;}
                                else {return 0;}
                            }
                            _=> return defValue,
                        }
                    }
                }
            },
            DxValue::Array(T) =>{
                if let Ok(index) = name.parse::<usize>(){
                    let arr = &(*T.borrow());
                    if index >= 0 && index < arr.len(){
                        match &(*arr[index].borrow()){
                            DxValue::Int32(t) => return *t  as isize,
                            DxValue::Int(t) => return *t,
                            DxValue::Int64(t) => return *t as isize,
                            DxValue::String(t) => return (*t).parse::<isize>().unwrap_or(defValue),
                            DxValue::Boolean(t) =>{
                                if *t {return 1;}
                                else {return 0;}
                            }
                            _=> return defValue,
                        }
                    }
                }
            },
            _=>(),
        }
        defValue
    }

    pub fn float_byName(&self,name: &str,defValue: f32) -> f32{
        match self {
            DxValue::Object(T) =>{
                let vec = &(*T.borrow());
                for i in 0..vec.len(){
                    let obj = vec[i].borrow();
                    if obj.Key == name{
                        match &(*obj.Value.borrow()) {
                            DxValue::Int32(t) => return *t  as f32,
                            DxValue::Int(t) => return *t as f32,
                            DxValue::Int64(t) => return *t as f32,
                            DxValue::String(t) => return (*t).parse::<f32>().unwrap_or(defValue),
                            DxValue::Boolean(t) =>{
                                if *t {return 1 as f32;}
                                else {return 0 as f32;}
                            }
                            _=> return defValue,
                        }
                    }
                }
            },
            DxValue::Array(T) =>{
                if let Ok(index) = name.parse::<usize>(){
                    let arr = &(*T.borrow());
                    if index >= 0 && index < arr.len(){
                        match &(*arr[index].borrow()){
                            DxValue::Int32(t) => return *t  as f32,
                            DxValue::Int(t) => return *t as f32,
                            DxValue::Int64(t) => return *t as f32,
                            DxValue::String(t) => return (*t).parse::<f32>().unwrap_or(defValue),
                            DxValue::Boolean(t) =>{
                                if *t {return 1 as f32;}
                                else {return 0 as f32;}
                            }
                            _=> return defValue,
                        }
                    }
                }
            },
            _=>(),
        }
        defValue
    }

    pub fn double_byName(&self,name: &str,defValue: f64) -> f64{
        match self {
            DxValue::Object(T) =>{
                let vec = &(*T.borrow());
                for i in 0..vec.len(){
                    let obj = vec[i].borrow();
                    if obj.Key == name{
                        match &(*obj.Value.borrow()) {
                            DxValue::Int32(t) => return *t  as f64,
                            DxValue::Int(t) => return *t as f64,
                            DxValue::Int64(t) => return *t as f64,
                            DxValue::String(t) => return (*t).parse::<f64>().unwrap_or(defValue),
                            DxValue::Boolean(t) =>{
                                if *t {return 1 as f64;}
                                else {return 0 as f64;}
                            }
                            _=> return defValue,
                        }
                    }
                }
            },
            DxValue::Array(T) =>{
                if let Ok(index) = name.parse::<usize>(){
                    let arr = &(*T.borrow());
                    if index >= 0 && index < arr.len(){
                        match &(*arr[index].borrow()){
                            DxValue::Int32(t) => return *t  as f64,
                            DxValue::Int(t) => return *t as f64,
                            DxValue::Int64(t) => return *t as f64,
                            DxValue::String(t) => return (*t).parse::<f64>().unwrap_or(defValue),
                            DxValue::Boolean(t) =>{
                                if *t {return 1 as f64;}
                                else {return 0 as f64;}
                            }
                            _=> return defValue,
                        }
                    }
                }
            },
            _=>(),
        }
        defValue
    }

    pub fn bool_byName(&self,name: &str,defValue: bool) -> bool{
        match self {
            DxValue::Object(T) =>{
                let vec = &(*T.borrow());
                for i in 0..vec.len(){
                    let obj = vec[i].borrow();
                    if obj.Key == name{
                        match &(*obj.Value.borrow()) {
                            DxValue::Int32(t) => return *t != 0,
                            DxValue::Int(t) => return *t != 0,
                            DxValue::Int64(t) => return *t != 0,
                            DxValue::String(t) => {
                                if let Ok(v64) = (*t).parse::<f64>(){
                                    return v64 != 0.0;
                                }
                            },
                            DxValue::Boolean(t) => return *t,
                            _=> return defValue,
                        }
                    }
                }
            },
            DxValue::Array(T) =>{
                if let Ok(index) = name.parse::<usize>(){
                    let arr = &(*T.borrow());
                    if index >= 0 && index < arr.len(){
                        match &(*arr[index].borrow()){
                            DxValue::Int32(t) => return *t != 0,
                            DxValue::Int(t) => return *t != 0,
                            DxValue::Int64(t) => return *t != 0,
                            DxValue::String(t) => {
                                if let Ok(v64) = (*t).parse::<f64>(){
                                    return v64 != 0.0;
                                }
                            },
                            DxValue::Boolean(t) => return *t,
                            _=> return defValue,
                        }
                    }
                }
            },
            _=>(),
        }
        defValue
    }

    pub fn num_byName<T: Display+std::str::FromStr>(&self,name: &str,defValue: T) -> T{
        match self {
            DxValue::Object(T) =>{
                let vec = &(*T.borrow());
                for vecobj in vec{
                    let obj = vecobj.borrow();
                    if obj.Key == name{
                        return format!("{}",obj.Value.borrow()).parse::<T>().unwrap_or(defValue);
                    }
                }
            },
            DxValue::Array(T) =>{
                if let Ok(index) = name.parse::<usize>(){
                    let arr = &(*T.borrow());
                    if index >= 0 && index < arr.len(){
                        let v = arr[index].borrow();
                        return format!("{}",v).parse::<T>().unwrap_or(defValue);
                    }
                }
            },
            _=>(),
        }
        defValue
    }

    /// 通过名称查找对应的值
    /// # Examples
    /// ```
    /// use rust_value::dxvalue::{Key_Value,DxValue};
    ///
    /// ```
    pub fn value_byName(&mut self, name: &str) ->Option<&mut Self> {
        match self {
            DxValue::Object(T) =>{
                let objects = T.get_mut();
                for i in 0..objects.len(){
                    if objects[i].Key == name{
                        return Some(objects[i].Value.get_mut());
                    }
                }
            },
            DxValue::Array(T) =>{
                //将字符串转换为整数
                if let Ok(index) = name.parse::<usize>(){
                    let arr = T.get_mut();
                    if index >= 0 && index < arr.len(){
                        return Some(& mut arr[index]);
                    }
                }
            },
            _=>(),
        }
        None
    }

    /// 查询指定的索引位置上的值
    ///
    pub fn value_byIndex<'a>(&'a mut self,index: usize) -> Option<&'a mut Self>{
        match self {
            DxValue::Object(T) =>{
                let vec = T.get_mut();
                if index < vec.len(){
                    return Some(vec[index].Value.get_mut());
                }
            },
            DxValue::Array(T)=>{
                let arr = T.get_mut();
                if index < arr.len(){
                    return Some(&mut arr[index]);
                }
            },
            _=>(),
        }
        None
    }

    pub fn string_byIndex(&self,index: usize,defValue: String)->String{
        match self {
            DxValue::Object(T) =>{
                let vec = T.borrow();
                if index < vec.len(){
                    if let Some(v) =  vec.get(index){
                        return v.Value.borrow().as_String();
                    }
                }
            },
            DxValue::Array(T)=>{
                let arr = T.borrow();
                if index < arr.len(){
                    if let Some(v) = arr.get(index){
                        return v.as_String();
                    }
                }
            },
            _=>(),
        }
        defValue
    }

    pub fn int_byIndex(&self,index: usize,defValue: isize)->isize{
        match self {
            DxValue::Object(T) =>{
                let vec = T.borrow();
                if index < vec.len(){
                    if let Some(v) =  vec.get(index){
                        return v.Value.borrow().as_int();
                    }
                }
            },
            DxValue::Array(T)=>{
                let arr = T.borrow();
                if index < arr.len(){
                    if let Some(v) = arr.get(index){
                        return v.as_int();
                    }
                }
            },
            _=>(),
        }
        defValue
    }

    pub fn bool_byIndex(&self,index: usize,defValue: bool)->bool{
        match self {
            DxValue::Object(t) =>{
                let vec = t.borrow();
                if index < vec.len(){
                    if let Some(v) =  vec.get(index){
                        return v.Value.borrow().as_bool();
                    }
                }
            },
            DxValue::Array(t)=>{
                let arr = t.borrow();
                if index < arr.len(){
                    if let Some(v) = arr.get(index){
                        return v.as_bool();
                    }
                }
            },
            _=>(),
        }
        defValue
    }

    pub fn float_byIndex(&self,index: usize,defValue: f32)->f32{
        match self {
            DxValue::Object(t) =>{
                let vec = t.borrow();
                if index < vec.len(){
                    if let Some(v) =  vec.get(index){
                        return v.Value.borrow().as_float();
                    }
                }
            },
            DxValue::Array(t)=>{
                let arr = t.borrow();
                if index < arr.len(){
                    if let Some(v) = arr.get(index){
                        return v.as_float();
                    }
                }
            },
            _=>(),
        }
        defValue
    }

    pub fn double_byIndex(&self,index: usize,defValue: f64)->f64{
        match self {
            DxValue::Object(t) =>{
                let vec = t.borrow();
                if index < vec.len(){
                    if let Some(v) =  vec.get(index){
                        return v.Value.borrow().as_double();
                    }
                }
            },
            DxValue::Array(t)=>{
                let arr = t.borrow();
                if index < arr.len(){
                    if let Some(v) = arr.get(index){
                        return v.as_double();
                    }
                }
            },
            _=>(),
        }
        defValue
    }
}
