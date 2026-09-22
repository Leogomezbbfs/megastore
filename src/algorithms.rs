use rayon::prelude::*;

// --- 1. ORDENAÇÃO: Merge Sort Recursivo Estável ---
pub fn merge_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    let mid = arr.len() / 2;
    let left = merge_sort(&arr[..mid]);
    let right = merge_sort(&arr[mid..]);

    merge(&left, &right)
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i].clone());
            i += 1;
        } else {
            result.push(right[j].clone());
            j += 1;
        }
    }
    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

// --- 2. ORDENAÇÃO: Quick Sort Otimizado com Stack Segura ---
pub fn quick_sort<T: Ord>(mut arr: &mut [T]) {
    while arr.len() > 1 {
        let pivot_index = partition(arr);

        // Chamada recursiva na partição menor para limitar a profundidade de pilha a O(log n)
        if pivot_index < arr.len() - pivot_index {
            quick_sort(&mut arr[..pivot_index]);
            arr = &mut arr[pivot_index + 1..];
        } else {
            quick_sort(&mut arr[pivot_index + 1..]);
            arr = &mut arr[..pivot_index];
        }
    }
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let mid = len / 2;
    // Coloca o elemento do meio no fim para servir de pivô (evita pior caso em arrays ordenados/invertidos)
    arr.swap(mid, len - 1);

    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}

// --- 3. BUSCAS CLÁSSICAS: Busca Binária O(log n) ---
pub fn binary_search(arr: &[u32], target: u32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len().checked_sub(1)?;

    while low <= high {
        let mid = low + (high - low) / 2;
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => {
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            }
        }
    }
    None
}

// --- 4. BUSCAS CLÁSSICAS: Busca Interpolada com Salvaguardas ---
pub fn interpolation_search(arr: &[u32], target: u32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    let mut low = 0usize;
    let mut high = arr.len() - 1;

    while low <= high && target >= arr[low] && target <= arr[high] {
        if arr[high] == arr[low] {
            if arr[low] == target {
                return Some(low);
            }
            return None;
        }

        // Fórmula de interpolação com prevenção de overflow
        let pos = low + (((target - arr[low]) as usize * (high - low)) / (arr[high] - arr[low]) as usize);

        if pos >= arr.len() {
            break;
        }

        match arr[pos].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(pos),
            std::cmp::Ordering::Less => low = pos + 1,
            std::cmp::Ordering::Greater => {
                if pos == 0 {
                    break;
                }
                high = pos - 1;
            }
        }
    }
    None
}

// --- 5. PARALELISMO E ITERADORES FUNCIONAIS COM RAYON ---
pub fn sum_prices_parallel(prices: &[u64]) -> u64 {
    prices.par_iter().sum()
}

pub fn parallel_sort<T: Ord + Send>(arr: &mut [T]) {
    arr.par_sort_unstable();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorting_algorithms() {
        let original = vec![45, 12, 85, 32, 89, 39, 69, 44, 42, 1, 45];
        
        // Merge Sort
        let sorted_merge = merge_sort(&original);
        let mut expected = original.clone();
        expected.sort();
        assert_eq!(sorted_merge, expected);

        // Quick Sort
        let mut quick_arr = original.clone();
        quick_sort(&mut quick_arr);
        assert_eq!(quick_arr, expected);
    }

    #[test]
    fn test_searches() {
        let sorted = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        assert_eq!(binary_search(&sorted, 70), Some(6));
        assert_eq!(binary_search(&sorted, 95), None);

        assert_eq!(interpolation_search(&sorted, 30), Some(2));
        assert_eq!(interpolation_search(&sorted, 105), None);
    }

    #[test]
    fn test_parallel_operations() {
        let prices = vec![1000, 2000, 3000, 4000];
        let total = sum_prices_parallel(&prices);
        assert_eq!(total, 10000);

        let mut parallel_data = vec![5, 1, 9, 3];
        parallel_sort(&mut parallel_data);
        assert_eq!(parallel_data, vec![1, 3, 5, 9]);
    }
}