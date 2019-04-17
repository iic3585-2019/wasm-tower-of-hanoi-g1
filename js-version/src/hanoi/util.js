
export const createReverseRange = (length) =>
  Array(length).fill().map((x, i) => length - i);

export const moveDisk = (target, source) => {
  target.push(source.pop());
};
