use std::collections::HashMap;

fn main() {
    let mut temperatures = vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5, 0.0]; // вектор хранящий значения температур
    println!("{temperatures:?}");
    temperatures.sort_by(|a, b| a.partial_cmp(b).unwrap()); // сортируем значения по возрастанию
    println!("{temperatures:?}");
    let mut map = HashMap::new();
    let (mut min, mut max) = create_range(temperatures[0]); // получаем границы интервала для минимального числа
    let mut values_vec = vec![];
    // проходим по значениям температур
    for temperature in temperatures {
        if temperature as i64 >= max {
            //если текущее значение больше верхней границы интервала, заносим ранее полученные значения и границы интервала в хэш-таблицу, очищаем вектор хранящий значения температур, генерируем новый интервал для текущего числа
            map.insert((min, max), values_vec.clone());
            (min, max) = create_range(temperature);
            values_vec.clear();
        }
        // заносим текущее значение температуры в вектор
        values_vec.push(temperature);
    }

    // для последнего(самого старшего) интервала условие выполняться не будет(если он существует), поэтому обработаем этот случай
    if !values_vec.is_empty() {
        map.insert((min, max), values_vec);
    }
    //вывод результата
    println!("{:?}", map);
}

// Функция для создания интервала с шагом в 10 градусов, возвращает верхнюю и нижнюю границы интервала, т.е. округляет число до десятков в большую и в меньшую сторону
fn create_range(temperature: f64) -> (i64, i64) {
    let int_temp = (temperature / 10.0) as i64;
    if int_temp >= 0 {
        (int_temp * 10, (int_temp + 1) * 10)
    } else {
        ((int_temp - 1) * 10, int_temp * 10)
    }
}
