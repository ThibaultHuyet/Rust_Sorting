
pub mod sorting   
{
    // O(n^2)
    pub fn insertion_sort<T: PartialOrd>(arr: &mut [T])
    {
        // When doing the let (mut i, j) format
        // j is not mutable
        let (mut j, size) = (0, arr.len());
        while j < size
        {
            let mut i = j;
            while i > 0 && arr[i - 1] > arr[i]
            {
                arr.swap(i, i - 1);
                i = i - 1;
            }
        j += 1;
        }
    }

    fn heapify<T: PartialOrd>(arr: &mut [T], len: usize, i: usize)
    {
        let mut largest = i;

        // Assume array of [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        // the left node is defined as the child of x at position
        // 2x + 1. right is the same but at 2x + 2
        let left = 2*i + 1;
        let right = 2*i + 2;

        // Following code is ensuring that root node is highest value
        if left < len && arr[left] > arr[largest]
        {
            largest = left;
        }

        if right < len && arr[right] > arr[largest]
        {
            largest = right;
        }

        if largest != i
        {
            arr.swap(largest, i);
            heapify(arr, len, largest);
        }
    }

    pub fn heap_sort<T: PartialOrd>(arr: &mut [T])
    {
        let len = arr.len();

        if len <= 1
        {
            return;
        }

        for i in (0..len/2).rev()
        {
            heapify(arr, len, i);
        }

        for i in (1..len).rev()
        {
            arr.swap(0, i);
            heapify(arr, i, 0)
        }
    }

    pub fn bubble_sort<T: PartialOrd>(arr: &mut [T])
    {
        let n = arr.len();

        for _ in 0..n
        {
            for j in 0..n - 1
            {
                if arr[j] > arr[j + 1]
                {
                    arr.swap(j, j + 1);
                }

            }

        }
    }

    pub fn selection_sort<T: PartialOrd>(arr: &mut [T])
    {
        // track position of elements finished
        // swap lowest in array with current tracked position

        let n = arr.len();

        for i in 0..n
        {
            for j in 0..n - 1
            {
                if arr[j] > arr[i]
                {
                    arr.swap(j, i);
                }
            }
        }
    }
}