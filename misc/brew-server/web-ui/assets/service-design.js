/* NetCore shell enhancement: existing DOM nodes, listeners, IDs and API calls stay intact. */
(() => {
  "use strict";
  const enhance = () => {
    const configNode = document.getElementById("netcore-service-config");
    if (!configNode || document.querySelector(".nc-service-header")) return;
    let config;
    try { config = JSON.parse(configNode.textContent); } catch { return; }
    const root = document.documentElement;
    root.dataset.netcoreUi = "service";
    const name = String(config.name || "NetCore Dienst");
    const make = (tag, cls, content) => {
      const node = document.createElement(tag);
      if (cls) node.className = cls;
      if (content !== undefined) node.textContent = content;
      return node;
    };
    const header = make("header", "nc-service-header");
    header.setAttribute("aria-label", "NetCore Dienstnavigation");
    const row = make("div", "nc-service-brandrow");
    const brand = make("div", "nc-service-brand");
    const logo = make("span", "nc-service-logo");
    ["mark", "wordmark"].forEach(part => {
      const crop = make("span", "nc-service-logo-" + part);
      const img = make("img");
      img.src = config.logo;
      img.alt = part === "wordmark" ? "NetCore-Tetra" : "";
      img.width = 1024;
      img.height = 1024;
      crop.append(img);
      logo.append(crop);
    });
    brand.append(logo, make("span", "nc-service-name", name));
    const tools = make("div", "nc-service-tools");
    const accessLabels = { "open-lab": "OPEN LAB", "session": "Sitzungszugang", "access-key": "Zugangsschlüssel", "optional-basic": "HTTP-Basic optional", "http-basic": "HTTP-Basic" };
    const access = make("span", "nc-service-access", accessLabels[config.access] || "NetCore Dienst");
    access.dataset.access = String(config.access || "");
    tools.append(access);
    const theme = make("button", "nc-theme-toggle");
    theme.type = "button";
    const themeKey = "netcore-service-theme";
    const setTheme = dark => {
      root.dataset.ncTheme = dark ? "dark" : "light";
      theme.textContent = dark ? "Hell" : "Dunkel";
      theme.setAttribute("aria-label", dark ? "Helles Design aktivieren" : "Dunkles Design aktivieren");
      theme.setAttribute("aria-pressed", String(dark));
    };
    let saved;
    try { saved = localStorage.getItem(themeKey); } catch { /* Storage may be blocked by the browser. */ }
    setTheme(saved === "dark");
    theme.addEventListener("click", () => {
      const dark = root.dataset.ncTheme !== "dark";
      setTheme(dark);
      try { localStorage.setItem(themeKey, dark ? "dark" : "light"); } catch { /* Theme still works without persistence. */ }
    });
    tools.append(theme);
    row.append(brand, tools);
    header.append(row);
    document.querySelectorAll(".banner").forEach(node => {
      if (/OPEN\s*LAB/i.test(node.textContent)) node.dataset.ncLab = "true";
    });

    // Move only service navigation, never a form, filter bar or operational button group.
    const candidates = [...document.querySelectorAll("nav, .tabs, .tabbar, .tab-nav, .service-tabs")];
    const nav = candidates.find(node => !node.closest("dialog, .modal") &&
      node.querySelectorAll("button, a").length >= 2 && !node.querySelector("input, select, textarea"));
    if (nav) {
      const context = nav.closest("aside");
      const layout = nav.closest(".layout");
      if (layout) layout.dataset.ncLayout = "horizontal";
      if (context) context.dataset.ncContext = "service";
      nav.classList.add("nc-service-nav");
      nav.setAttribute("aria-label", "Dienstseiten");
      header.append(nav);
    } else {
      // One-page services get links to existing headed panels, without new API destinations.
      const panels = [...document.querySelectorAll("main .panel, .wrap .panel, main > section, .container .panel")]
        .filter(panel => !panel.closest("dialog, .modal") && !panel.matches(".page, .view, .tab, .cards") && panel.querySelector("h2, h3"));
      if (panels.length >= 2) {
        const anchorNav = make("nav", "nc-service-nav");
        anchorNav.setAttribute("aria-label", "Bereiche dieser Seite");
        const labels = new Set();
        panels.forEach((panel, index) => {
          const heading = panel.querySelector("h2, h3");
          const label = heading.textContent.trim();
          if (!label || labels.has(label)) return;
          labels.add(label);
          if (!panel.id) {
            let id = "nc-panel-" + index;
            while (document.getElementById(id)) id += "-section";
            panel.id = id;
          }
          const link = make("a", "", label);
          link.href = "#" + panel.id;
          anchorNav.append(link);
        });
        header.append(anchorNav);
      }
    }
    document.querySelectorAll("body > header, .wrap > header").forEach(node => node.classList.add("nc-original-header"));
    document.body.prepend(header);
    // Wrap tables once to keep wide operational data scrollable on small screens.
    document.querySelectorAll("table").forEach(table => {
      if (table.closest(".tablewrap, .table-wrap, .nc-table-wrap, .scroll")) return;
      const wrapper = make("div", "nc-table-wrap");
      wrapper.setAttribute("tabindex", "0");
      wrapper.setAttribute("role", "region");
      const title = table.closest(".panel, section, article")?.querySelector("h2, h3")?.textContent.trim();
      wrapper.setAttribute("aria-label", title ? "Tabelle: " + title : "Datentabelle");
      table.before(wrapper);
      wrapper.append(table);
    });
  };
  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", enhance, { once: true });
  else enhance();
})();
