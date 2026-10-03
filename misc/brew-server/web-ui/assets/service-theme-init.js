/* Blocking head initializer: apply the saved service theme before the first paint. */
(() => {
  "use strict";
  let saved = null;
  try {
    saved = localStorage.getItem("netcore-theme");
    if (saved === null) saved = localStorage.getItem("netcore-service-theme");
  } catch { /* Light remains the default when browser storage is unavailable. */ }
  const root = document.documentElement;
  root.dataset.netcoreUi = "service";
  // The base station also supports "blue"; an explicit blue choice is light here.
  root.dataset.ncTheme = saved === "dark" ? "dark" : "light";
})();
