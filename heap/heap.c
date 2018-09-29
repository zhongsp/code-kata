/**
 * （二叉）最大堆，数组实现（数组索引从0开始）。
 */

// 父结点索引
#define PARENT(i) (i-1)/2

// 左儿子索引
#define LEFT(i) 2*i+1

// 右儿子索引
#define RIGHT(i) 2*i+2

/**
 * 保持最大堆性质。使以索引 i 为根的子树成为最大堆。
 * 即将当前根结点与其左/右儿子比较，若当前根结点非最大值，
 * 则令其下降，与最大子结点交换。
 * 
 * 当给定结点数为（n）时，堆的最大高度为（lg n）。
 * 时间复杂度为：O(lg n)。
 */
void max_heapify(int heap[], int heap_size, int i)
{
  int left = LEFT(i);
  int right = RIGHT(i);
  int largest = i;

  if (left < heap_size && heap[left] > heap[largest]) {
    largest = left;
  }

  if (right < heap_size && heap[right] > heap[largest]) {
    largest = right;
  }

  int temp = heap[i];
  if (largest != i) {
    heap[i] = heap[largest];
    heap[largest] = temp;
    max_heapify(heap, heap_size, largest);
  }
}
