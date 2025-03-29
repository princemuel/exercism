// @ts-check

// Position,
// ProgramWindow,
// Size,
// changeWindow,
/**
 * The Size class for storing the dimensions of the window
 * @param {number | undefined} width
 * @param {number | undefined} height
 */
export function Size(width = 80, height = 60) {
  this.width = width;
  this.height = height;
}

Size.prototype.resize = function (
  /** @type {number} */ newWidth,
  /** @type {number} */ newHeight
) {
  this.width = newWidth;
  this.height = newHeight;
};

/**
 * The Position class to store a window position
 * @param {number | undefined} x
 * @param {number | undefined} y
 */
export function Position(x = 0, y = 0) {
  this.x = x;
  this.y = y;
}

Position.prototype.move = function (
  /** @type {number} */ newX,
  /** @type {number} */ newY
) {
  this.x = newX;
  this.y = newY;
};

export class ProgramWindow {
  constructor() {
    this.screenSize = new Size(800, 600);
    this.size = new Size();
    this.position = new Position();
  }

  /**
   * @param {Position} position
   */

  move(position) {
    const maxX = this.screenSize.width - this.size.width;
    const maxY = this.screenSize.height - this.size.height;

    const newX = Math.max(0, Math.min(position.x, maxX));
    const newY = Math.max(0, Math.min(position.y, maxY));

    this.position.move(newX, newY);
    return this;
  }

  /**
   * @param {Size} size
   */
  resize(size) {
    const maxWidth = this.screenSize.width - this.position.x;
    const maxHeight = this.screenSize.height - this.position.y;

    const newWidth = Math.max(1, Math.min(size.width, maxWidth));
    const newHeight = Math.max(1, Math.min(size.height, maxHeight));

    this.size.resize(newWidth, newHeight);
    return this;
  }
}

/**
 * This function  accepts a ProgramWindow instance and
 * changes the window to the specified size and position
  @param {ProgramWindow}  programWindow
 */
export const changeWindow = (/** @type {ProgramWindow} */ programWindow) => {
  return programWindow.resize(new Size(400, 300)).move(new Position(100, 150));
};
