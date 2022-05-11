import { Object3D } from 'three';

export class CSS2DObject extends Object3D {
  element: HTMLDivElement;

  constructor(element = document.createElement('div')) {
    super();

    this.element = element;

    this.element.style.position = 'absolute';
    this.element.style.userSelect = 'none';

    this.element.setAttribute('draggable', 'false');

    this.addEventListener('removed', () => {
      this.traverse((obj) => {
        const object = obj as CSS2DObject;

        if (
          object.element instanceof Element &&
          object.element.parentNode !== null
        ) {
          object.element.parentNode.removeChild(object.element);
        }
      });
    });
  }

  override copy(source: this, recursive: boolean) {
    super.copy(source, recursive);

    this.element = source.element.cloneNode(true) as HTMLDivElement;

    return this;
  }
}
