import { writable } from "svelte/store";

function SaveKeys() {
  const { subscribe, set, update } = writable([[], [], []]);
  return {
    subscribe,
    setN: (new_n: any[]) => {
      update(() => (SaveKeys[0][0] = new_n));
    },
    setE: (new_e: any[]) => {
      update(() => (SaveKeys[0][1] = new_e));
    },
    setD: (new_d: any[]) => {
      update(() => (SaveKeys[0][2] = new_d));
    },
    Rest: () => {
      set([[], [], []]);
    },
    init: (n: string[], e: string[], d: string[]) => {
      set([n, e, d]);
    },
  };
}
export const KeyStore = SaveKeys();
