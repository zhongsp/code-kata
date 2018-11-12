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

export function print(heap: Heap): void {
  const { size, elements } = heap;
  const height = getHeapHeight(heap);

  let output: any[][] = Array[height];
  output[height - 1] = buildDeepestTier();

  for(let i = height - 1; i >= 0; i--) {
    const theNextTier = output[i + 1];
    let result = Array(size).fill(' ');
    
    let startIndex = getMaxHeapElementsWithHeight(i - 1) + 1;
    const source = heap.elements.slice(startIndex, startIndex + getElementsOnHeight(i) + 1);
    let pos = 0;

    let stack = [];

    theNextTier.forEach((element, index) => {
      if (element !== ' ') {
        stack.push(index);

        if (stack.length === 2) {
          let end = stack.pop();
          let start = stack.pop();
          const insertIdx = (start + end) / 2;
          result[insertIdx] = source[pos];
          pos++
        }
      }
    });
  }

  console.log(output);

  function buildDeepestTier() {
    let result = Array(heap.size).fill(' ');
    let startIndex = getMaxHeapElementsWithHeight(height - 1) + 1;
    let source = heap.elements.slice(startIndex, heap.size + 1);
    let formatSource = source.join(' ').split('');

    for(let i = 0; i < heap.size; i++) {
      if (formatSource[i] !== undefined) {
        result[i] = formatSource[i];
      } else {
        if (i % 2 === 0) {
          result[i] = null;
        }
      }
    }

    return result;
  }
}

/**
 * 堆的高度定义为根结点的高度。
 * 根结点的高度为根结点到叶子结点的最长边的条数。
 */
function getHeapHeight(heap: Heap): number {
  return Math.floor(Math.log2(heap.size));
}

/**
 * 获取给定高度的堆可以拥有的最多结点数。
 * 每层最多结点数是等比数列。
 */
function getMaxHeapElementsWithHeight(height: number): number {
  return 2 ** (height + 1) - 1;
}

function getElementsOnHeight(height: number): number {
  return 2 ** height;
}