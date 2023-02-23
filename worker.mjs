import init, { calculate_worker } from "./pkg/enchant_calc_gui.js";
self.onmessage = async (event) => {
  await init();
  calculate_worker(event.data);
};
