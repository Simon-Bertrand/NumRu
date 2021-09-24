use crate::matrix::*;


use std::ops::Mul;
use std::ops::Add;











impl<T> Matrix<T> {
    pub fn get(&self, i:i32, j:i32) -> &T {
        if i<0 || i>= self.shape.0 {
            eprintln!("\nfn get_mut(&mut self, i:i32, j:i32) >>> The indice is out of range : {} is not in [{}-{}] or {} is not in [{}-{}] . \n", i, 0, self.shape.0-1, j, 0, self.shape.1-1);
            std::process::exit(-1);
        }
       else {
            return self.values.iter().nth((i*self.shape.1 + j) as usize).expect("Indice not found")
        }
    }

    pub fn get_mut(&mut self, i:i32, j:i32) ->  &mut T  {
        if (i<0 || i>= self.shape.0)||((j<0 || j>= self.shape.1)) {
            eprintln!("\nfn get_mut(&mut self, i:i32, j:i32) >>> The indice is out of range : {} is not in [{}-{}] or {} is not in [{}-{}] . \n", i, 0, self.shape.0-1, j, 0, self.shape.1-1);
            std::process::exit(-1);
        }
       else {
            return self.values.iter_mut().nth((i*self.shape.1 + j) as usize).expect("Indice not found")
        }
    }
}





impl<T> Matrix<T> {
    pub fn row_iter(&self, i:i32) -> &[T] {
        if i<0 || i>= self.shape.0 {
            eprintln!("\nfn row_iter(&self, i:i32) >>> The row indice is out of range : {} is not in [{}-{}]. \n", i, 0, self.shape.0-1);
            std::process::exit(-1);
        }
       else {
            return self.values.chunks(self.shape.0 as usize).nth(i as usize).expect("Indice not found")
        }
    }

    pub fn row_iter_mut(&mut self, i:i32) ->  &mut [T]  {
        if i<0 || i>= self.shape.0 {
            eprintln!("\nfn row_iter_mut(&mut self, i:i32) >>> The row indice is out of range : {} is not in [{}-{}]. \n", i, 0, self.shape.0-1);
            std::process::exit(-1);
        }
       else {
            return self.values.chunks_mut(self.shape.0 as usize).nth(i as usize).expect("Indice not found")
        }
    }

    pub fn col_iter(&self, j:i32) -> std::iter::StepBy<std::slice::Iter<'_, T>>{
        if j<0 || j>= self.shape.1 {
            eprintln!("\nfn col_iter(&self, j:i32) >>> The col indice is out of range : {} is not in [{}-{}]. \n", j, 0, self.shape.1-1);
            std::process::exit(-1);
        }
       else {
            return self.values[j as usize..].iter().step_by(self.shape.0 as usize)
        }
    }

    pub fn col_iter_mut(&mut self, j:i32) ->  std::iter::StepBy<std::slice::IterMut<'_, T>>  {
        if j<0 || j>= self.shape.1 {
            eprintln!("\nfn col_iter(&self, j:i32) >>> The col indice is out of range : {} is not in [{}-{}]. \n", j, 0, self.shape.1-1);
            std::process::exit(-1);
        }
       else {
            return self.values[j as usize..].iter_mut().step_by(self.shape.0 as usize)
        }
    }
}


impl<T: std::clone::Clone + Copy> Matrix<T> {


    pub fn row(&self, i:i32) -> Matrix<T> {
        if i<0 || i>= self.shape.0 {
            eprintln!("\nfn row(&self, i:i32) >>> The row indice is out of range : {} is not in [{}-{}]. \n", i, 0, self.shape.0-1);
            std::process::exit(-1);
        }
       else {
            Matrix::<T> {values:(self.values[((i*self.shape.1) as usize)..((i*self.shape.1 + self.shape.0) as usize)]).to_vec(), shape:(1,self.shape.1)}
        }
    }
    
    
    pub fn col(&self, j:i32) -> Matrix<T> {
        if j<0 || j>= self.shape.1 {
            eprintln!("\nfn col(&self, i:i32) >>> The col indice is out of range : {} is not in [{}-{}]. \n", j, 0, self.shape.0-1);
            std::process::exit(-1);
        }
        else {
            Matrix::<T> {values:{let mut r : Vec<T> = Vec::with_capacity(self.shape.0 as usize); for col_el in self.col_iter(j) { r.push(*col_el)} r}, shape:(1,self.shape.1)}
        }
    }
    
}

/*
impl Matrix<i32> {
    pub fn convert_to_float(self, type_ : &str) -> Matrix<f64> {
        Matrix::<f64>{values :{ let mut r:Vec<f64>= Vec::new(); for i in 0..self.values.len() {r.push(self.values[i] as f64)} r}, shape: self.shape}
    }
}
*/


use std::mem;
impl<T : Mul<Output = T> + Add<Output = T> + Copy> Matrix<T> {

    pub fn dot(&self, m: &Matrix<T>) -> Matrix<T> {
        if self.shape.1 != m.shape.0 {
            eprintln!("\nfn dot(&self, M: &Matrix) >>> The shapes are not compatible for matrix product.\n");
            std::process::exit(-1);
        }
        Matrix::<T> {
            values:{
                let mut r:Vec<T>= Vec::with_capacity((self.shape.1*m.shape.0) as usize); 
                for i in 0..self.shape.1 {
                    for j in 0..m.shape.0 {
                        r.push({
                            let mut sum : T = unsafe { mem::zeroed() };
                                for k in 0..(self.shape.1 as usize) {
                                    sum = sum + self.row_iter(i)[k]* (*m.col_iter(j).nth(k).expect("Indice not found"))
                                    
                                } sum

                        })
                    }
                }

                r
            },
            shape: (self.shape.0,m.shape.1)
        }
 
    }
}




impl<T : std::cmp::PartialOrd<T>> Matrix<T> {
    pub fn max(&self) -> &T {
        let mut max = &self.values[0]; 
        for el in self.values.iter()  {if max < el  { max = el;}}
        max
    }
}





impl<T> Matrix<T> {
    fn is_row(&self) -> bool {
        if self.shape.0 == 1 {
            return true;
        }
        else {
            return false;
        }
    }
    fn is_col(&self) -> bool {
        if self.shape.1 == 1 {
            return true;
        }
        else {
            return false;
        }
    }
}



