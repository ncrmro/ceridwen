import {makeCompound,type Shape3D} from 'replicad';
/** A geometric STEP compound, with names/transforms supplied by component-metadata.json.
 * Avoids the pinned library's assembly writer owning its work session twice.
 */
export function assemblySTEP(shapes:Shape3D[]):Blob {
 const assembly=makeCompound(shapes.map(shape=>shape.clone()));
 const blob=assembly.blobSTEP();assembly.delete();return blob;
}
