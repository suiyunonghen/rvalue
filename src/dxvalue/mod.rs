use std::fmt::{Debug, Formatter, Result, Error, Display};
use std::{result,cell,boxed,rc};
use std::string;
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
        write!(f,"{}: {}",self.Key,self.Value.borrow())
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
            DxValue::String(T)=> write!(f,"{}",T),
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
                for v in vecArr{
                    write!(f,"{},",v)?;
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
    /// ```
    pub fn newObject() -> DxValue{
        DxValue::Object(RefCell::new(Vec::new()))
    }

    pub fn newArray() -> DxValue{
        DxValue::Array(RefCell::new(Vec::new()))
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


    pub fn as_String(&self) -> String{
        format!("{}",self)
    }

    pub fn as_Value<T: FromStr>(&self,defValue: T) -> T{
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
                let mut vecobj = T.get_mut();
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

}
