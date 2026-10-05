/** Interface coupons test lateral print fit only, never electrical or final case acceptance. */
import { makeBox, type Shape3D } from 'replicad';

export interface Coupon { id: string; shape: Shape3D; description: string; openings: { count: number; width: number; depth: number }[] }
export function fitCoupons(): Coupon[] {
  function bank(id: string, width: number, depth: number, gaps: number[], pitch: number, description: string): Coupon {
    const length = pitch * gaps.length, outerDepth = depth + 12;
    let shape: Shape3D = makeBox([0, 0, 0], [length, outerDepth, 2]);
    const openings = gaps.map((gap, i) => {
      const w = width + gap * 2, d = depth + gap * 2;
      const cx = pitch * (i + 0.5), cy = outerDepth / 2;
      shape = shape.cut(makeBox([cx - w / 2, cy - d / 2, -1], [cx + w / 2, cy + d / 2, 3]));
      // One, two, three visible edge notches identify openings without tiny text.
      for (let n = 0; n <= i; n++) {
        const x = cx - i + n * 2;
        shape = shape.cut(makeBox([x - 0.4, -1, -1], [x + 0.4, 1, 3]));
      }
      return { count: i + 1, width: w, depth: d };
    });
    return { id, shape, description, openings };
  }
  return [
    bank('SWITCH_BODY_FIT', 6, 6, [0.05, 0.15, 0.25], 14,
      'Check lateral fit of the selected nominal 6 mm switch body. Does not set lead spacing or actuator travel.'),
    bank('BUTTON_GUIDE_FIT', 10, 8, [0.1, 0.2, 0.3], 18,
      'Check sliding fit of the prototype cap finger surface, inserted from beneath. Does not establish return force or travel stops.'),
  ];
}
