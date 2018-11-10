/**
 * （二叉）最大堆，数组实现（数组索引从0开始）。
 */

// 左儿子索引
const left = (i: number) => 2 * i;
// 右儿子索引
const right = (i: number) => 2 * i + 1;
// 父结点索引
const parent = (i: number) => Math.floor(i / 2);

export interface Heap {
  size: number;
  elements: number[];
}

export function printHeap(heap: Heap): void {
  const { size, elements } = heap;

  elements.forEach((ele, idx) => {
    if (idx > 0 && idx <= size) {
      console.log(ele);
    }
  });
}

/**
 * 保持最大堆性质。使以索引 i 为根的子树成为最大堆。
 * 即将当前根结点与其左/右儿子比较，若当前根结点非最大值，
 * 则令其下降，与最大子结点交换。
 * 
 * 当给定结点数为（n）时，堆的最大高度为（lg n）。
 * 时间复杂度为：O(lg n)。
 */
export function maxHeapify(heap: Heap, i: number): void {
  const { size, elements } = heap;

  // "i" is out of heap index range
  if (i > size) return;

  let largest = i;
  if (left(i) <= size && elements[largest] < elements[left(i)]) {
    largest = left(i);
  }
  if (right(i) <= size && elements[largest] < elements[right(i)]) {
    largest = right(i);
  }

  if (i !== largest) {
    [elements[i], elements[largest]] = [elements[largest], elements[i]];
    maxHeapify(heap, largest);
  }
}
