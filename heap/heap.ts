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

export function buildMaxHeap(heap: Heap): void {
  for(let i = Math.floor(heap.size / 2); i > 0; i--) {
    maxHeapify(heap, i);
  }
}

/**
 * 结点的高度为结点到最深的叶子结点的边的条数。
 * 堆的高度定义为根结点的高度。
 * 只有根结点的堆的高度为：0。
 * 空堆的高度为：-1。
 */
export function getHeapHeight(heap: Heap): number {
  return (heap.size <= 0)
    ? -1
    : Math.floor(Math.log2(heap.size));
}

/**
 * 结点的深度为根结点到此结点的边的条数。
 * 获取给定深度的堆可以容纳的最多结点数。
 * 每层最多结点数是等比数列。
 */
export function getMaxNodeCountWithDepth(depth: number): number {
  return 2 ** (depth + 1) - 1;
}

export function getMaxNodeCountOnDepth(depth: number): number {
  return 2 ** depth;
}

/**
 * 打印堆的树形结构。
 */
export function print(heap: Heap): void {
  const { size, elements } = heap;
  const height = getHeapHeight(heap);
  const lineWidth = getMaxNodeCountWithDepth(height);

  let output: any[][] = Array(height + 1);
  output[height] = buildDeepestLayer();

  for(let i = height - 1; i >= 0; i--) {
    const theNextTier = output[i + 1];
    let result = Array(lineWidth).fill(' ');

    const startIndex = getMaxNodeCountWithDepth(i - 1) + 1;
    const endIndex = startIndex + getMaxNodeCountOnDepth(i);
    const source = elements.slice(startIndex, endIndex);
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

    output[i] = result;
  }

  output.forEach(out => {
    console.log(out.join(''));
  })

  function buildDeepestLayer() {
    let result = Array(lineWidth).fill(' ');
    let startIndex = getMaxNodeCountWithDepth(height - 1) + 1;
    let source = elements.slice(startIndex, size + 1);
    let formatSource = source.join(' ').split('');

    for(let i = 0; i < lineWidth; i++) {
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
