// https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem
/// Задача: Migratory Birds
/// Знайти тип птаха, який зустрічається найчастіше. 
/// Якщо таких декілька, повернути найменший ID.

pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0; 6]; // Індекси 1-5 для типів птахів
    
    for &bird_type in arr {
        if bird_type >= 1 && bird_type <= 5 {
            counts[bird_type as usize] += 1;
        }
    }

    let mut max_count = 0;
    let mut result_id = 0;

    for (id, &count) in counts.iter().enumerate() {
        if count > max_count {
            max_count = count;
            result_id = id;
        }
    }

    result_id as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds_standard() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&arr), 4);
    }

    #[test]
    fn test_migratory_birds_tie() {
        // Рівна кількість птахів типу 1 та 2. Маємо вибрати менший ID (1).
        let arr = vec![1, 2, 1, 2, 3];
        assert_eq!(migratory_birds(&arr), 1);
    }

    #[test]
    fn test_migratory_birds_all_same() {
        let arr = vec![5, 5, 5, 5];
        assert_eq!(migratory_birds(&arr), 5);
    }
}