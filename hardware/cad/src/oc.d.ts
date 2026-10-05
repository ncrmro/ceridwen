declare module 'replicad-opencascadejs/src/replicad_single.js' {
  const init: (options: { locateFile: (file: string) => string }) => Promise<any>;
  export default init;
}
