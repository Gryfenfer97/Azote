use crate::value::Value;

pub fn conjugate(omega: &Value) -> Result<Value, String> {
    todo!()
}

pub fn plus(alpha: &Value, omega: &Value) -> Result<Value, String> {
    match (alpha, omega) {
        (Value::Number(value1), Value::Number(value2)) => {
            return Ok(Value::Number(value1 + value2));
        }
        (Value::Array(values1), Value::Array(values2)) => {
            if values1.len() != values2.len() {
                return Err("Vectors don't have the same rank".to_string());
            }
            let mut vector: Vec<Value> = Vec::new();
            for i in 0..values1.len() {
                if let (Value::Number(v1), Value::Number(v2)) = (&values1[i], &values2[i]) {
                    vector.push(Value::Number(v1 + v2))
                }
            }
            return Ok(Value::Array(vector));
        }
        (Value::Array(values), Value::Number(value))
        | (Value::Number(value), Value::Array(values)) => {
            let mut vector: Vec<Value> = Vec::new();
            for v in values {
                if let Value::Number(v1) = &v {
                    vector.push(Value::Number(v1 + value))
                }
            }
            return Ok(Value::Array(vector));
        }
        _ => unreachable!(),
    }
}

pub fn direction(omega: &Value) -> Result<Value, String> {
    match omega {
        Value::Number(value) => {
            return Ok(Value::Number(value / value.abs()));
        }
        Value::Array(values) => {
            let mut vector: Vec<Value> = Vec::new();
            for value in values {
                if let Value::Number(n) = value {
                    vector.push(Value::Number(n / n.abs()))
                }
            }
            return Ok(Value::Array(vector));
        }
        _ => {
            return Err("Domain Error".to_string());
        }
    }
}

pub fn times(alpha: &Value, omega: &Value) -> Result<Value, String> {
    match (alpha, omega) {
        (Value::Number(value1), Value::Number(value2)) => {
            return Ok(Value::Number(value1 * value2));
        }
        (Value::Array(values1), Value::Array(values2)) => {
            if values1.len() != values2.len() {
                return Err("Vectors don't have the same rank".to_string());
            }
            let mut vector: Vec<Value> = Vec::new();
            for i in 0..values1.len() {
                if let (Value::Number(v1), Value::Number(v2)) = (&values1[i], &values2[i]) {
                    vector.push(Value::Number(v1 * v2))
                }
            }
            return Ok(Value::Array(vector));
        }
        (Value::Array(values), Value::Number(value))
        | (Value::Number(value), Value::Array(values)) => {
            let mut vector: Vec<Value> = Vec::new();
            for v in values {
                if let Value::Number(v1) = &v {
                    vector.push(Value::Number(v1 * value))
                }
            }
            return Ok(Value::Array(vector));
        }
        _ => unreachable!(),
    }
}

pub fn ceiling(omega: &Value) -> Result<Value, String> {
    match omega {
        Value::Number(value) => {
            return Ok(Value::Number(value.ceil()));
        }
        Value::Array(values) => {
            let mut vector: Vec<Value> = Vec::new();
            for value in values {
                if let Value::Number(n) = value {
                    vector.push(Value::Number(n.ceil()))
                }
            }
            return Ok(Value::Array(vector));
        }
        _ => {
            return Err("Domain Error".to_string());
        }
    }
}

pub fn maximum(alpha: &Value, omega: &Value) -> Result<Value, String> {
    match (alpha, omega) {
        (Value::Number(value1), Value::Number(value2)) => {
            return Ok(Value::Number(f32::max(*value1, *value2)));
        }
        (Value::Array(values1), Value::Array(values2)) => {
            if values1.len() != values2.len() {
                return Err("Vectors don't have the same rank".to_string());
            }
            let mut vector: Vec<Value> = Vec::new();
            for i in 0..values1.len() {
                if let (Value::Number(v1), Value::Number(v2)) = (&values1[i], &values2[i]) {
                    vector.push(Value::Number(f32::max(*v1, *v2)))
                }
            }
            return Ok(Value::Array(vector));
        }
        _ => unreachable!(),
    }
}

pub fn floor(omega: &Value) -> Result<Value, String> {
    match omega {
        Value::Number(value) => {
            return Ok(Value::Number(value.floor()));
        }
        Value::Array(values) => {
            let mut vector: Vec<Value> = Vec::new();
            for value in values {
                if let Value::Number(n) = value {
                    vector.push(Value::Number(n.floor()))
                }
            }
            return Ok(Value::Array(vector));
        }
        _ => {
            return Err("Domain Error".to_string());
        }
    }
}

pub fn minimum(alpha: &Value, omega: &Value) -> Result<Value, String> {
    match (alpha, omega) {
        (Value::Number(value1), Value::Number(value2)) => {
            return Ok(Value::Number(f32::min(*value1, *value2)));
        }
        (Value::Array(values1), Value::Array(values2)) => {
            if values1.len() != values2.len() {
                return Err("Vectors don't have the same rank".to_string());
            }
            let mut vector: Vec<Value> = Vec::new();
            for i in 0..values1.len() {
                if let (Value::Number(v1), Value::Number(v2)) = (&values1[i], &values2[i]) {
                    vector.push(Value::Number(f32::min(*v1, *v2)))
                }
            }
            return Ok(Value::Array(vector));
        }
        _ => unreachable!(),
    }
}
