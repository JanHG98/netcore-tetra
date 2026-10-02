/* NetCore-Tetra RF-Arbeitsbereich. Die vorhandenen Telemetrie-Elemente behalten
 * ihre IDs; Messwerte und Trägerbelegung stammen weiterhin aus dem Dashboard. */
(() => {
  'use strict';
  const page = document.getElementById('page-rf');
  if (!page || page.dataset.netcoreRf === '1') return;
  const spectrum = document.getElementById('rf-spectrum');
  const waterfall = document.getElementById('rf-waterfall');
  const constellation = document.getElementById('rf-constellation');
  const spectrumPanel = spectrum?.closest('.rf-panel');
  const waterfallPanel = waterfall?.closest('.rf-panel');
  const constellationPanel = constellation?.closest('.rf-panel');
  const qualityCards = Array.from(page.querySelectorAll('.rf-quality-card'));
  if (!spectrumPanel || !waterfallPanel || !constellationPanel || qualityCards.length < 2) return;
  page.dataset.netcoreRf = '1';

  // Erstellt ausschließlich statische Beschriftungen; Telemetrietext wird nie als HTML eingesetzt.
  function element(tag, className, text) {
    const node = document.createElement(tag);
    node.className = className;
    if (text) node.textContent = text;
    return node;
  }

  const workbench = element('div', 'nc-rf-workbench');
  const plots = element('section', 'nc-rf-plots');
  plots.setAttribute('aria-label', 'TX-DSP-Spektrum und Wasserfall');
  const plotHeading = element('div', 'nc-rf-card-heading');
  plotHeading.append(element('h2', '', 'Spektrum & Wasserfall'), element('span', '', 'TX-DSP vor dem Leistungsverstärker'));
  plots.append(plotHeading, spectrumPanel, waterfallPanel);
  waterfallPanel.style.marginTop = '';
  const telemetry = element('aside', 'nc-rf-telemetry');
  telemetry.setAttribute('aria-label', 'DSP-Qualität und SDR-Istwerte');
  telemetry.append(...qualityCards);
  const constellationDetails = element('details', 'nc-rf-constellation');
  constellationDetails.append(element('summary', '', 'Konstellation anzeigen · π/4-DQPSK'), constellationPanel);
  telemetry.append(constellationDetails);
  workbench.append(plots, telemetry);

  // Entfernt nur die durch den neuen Arbeitsbereich ersetzten, nun leeren Layout-Hüllen.
  const oldGrid = page.querySelector('.rf-grid');
  oldGrid?.remove();
  page.querySelectorAll(':scope > .section-label').forEach(node => node.remove());
  page.append(workbench);

  const carrierSection = element('section', 'nc-rf-carriers');
  carrierSection.setAttribute('aria-label', 'Träger und Zeitschlitze');
  const carrierHeading = element('div', 'nc-rf-card-heading');
  carrierHeading.append(element('h2', '', 'Träger & Zeitschlitze'), element('span', '', 'Aktuelle Belegung'));
  const carrierRows = element('div', 'nc-rf-carrier-rows');
  carrierSection.append(carrierHeading, carrierRows);
  page.append(carrierSection);

  const emptyOverlays = new Map();
  [spectrumPanel, waterfallPanel, constellationPanel].forEach(panel => {
    const canvas = panel.querySelector('canvas');
    const frame = element('div', 'nc-rf-canvas-frame');
    const empty = element('div', 'nc-rf-awaiting', 'Warte auf TX-DSP-Daten…');
    panel.append(frame);
    frame.append(canvas, empty);
    emptyOverlays.set(canvas.id, empty);
  });

  let latestVisual = null;
  let carrierSignature = '';
  let scheduled = false;
  let carriersScheduled = false;

  // Gibt eine schreibgeschützte Ansicht der vorhandenen TS-Kacheln wieder, ohne deren IDs zu duplizieren.
  function syncCarriers() {
    const source = document.getElementById('ts-grid');
    const tiles = source ? Array.from(source.querySelectorAll('.ts-block')) : [];
    const snapshot = tiles.map(tile => ({
      carrier: tile.dataset.carrier || '—',
      logical: tile.dataset.ts || '—',
      air: tile.dataset.airTs || tile.dataset.ts || '—',
      classes: ['mcch', 'call', 'voice', 'emergency'].filter(name => tile.classList.contains(name)),
      label: tile.querySelector('.ts-label')?.textContent || '—',
      sub: tile.querySelector('.ts-sub')?.textContent || '',
      timer: tile.querySelector('.ts-timer')?.textContent || ''
    }));
    const signature = JSON.stringify(snapshot);
    if (signature === carrierSignature) return;
    carrierSignature = signature;
    carrierRows.replaceChildren();
    if (!snapshot.length || snapshot.every(slot => slot.carrier === '—')) {
      carrierRows.append(element('p', 'nc-rf-carrier-empty', 'Noch keine Trägerdaten verfügbar.'));
      return;
    }
    const grouped = new Map();
    snapshot.forEach(slot => {
      if (!grouped.has(slot.carrier)) grouped.set(slot.carrier, []);
      grouped.get(slot.carrier).push(slot);
    });
    grouped.forEach((slots, carrier) => {
      const secondary = slots.some(slot => Number(slot.logical) >= 5);
      const row = element('div', 'nc-rf-carrier-row');
      const description = element('div', 'nc-rf-carrier-description');
      description.append(element('strong', '', 'Träger ' + carrier), element('span', '', secondary ? 'Sekundärträger' : 'Hauptträger'));
      row.append(description);
      slots.forEach(slot => {
        const cell = element('div', 'nc-rf-slot ' + slot.classes.map(name => 'is-' + name).join(' '));
        cell.title = 'Träger ' + carrier + ' · Funk-TS ' + slot.air + ' · logischer TS ' + slot.logical;
        const label = element('div', 'nc-rf-slot-number', 'TS ' + slot.air);
        if (secondary && slot.air === '1') label.append(element('span', '', 'Steuerung / Guard'));
        else if (secondary) label.append(element('span', '', 'log. TS ' + slot.logical));
        const state = element('div', 'nc-rf-slot-state', slot.label);
        const detail = element('div', 'nc-rf-slot-detail', [slot.sub, slot.timer].filter(Boolean).join(' · '));
        cell.append(label, state, detail);
        row.append(cell);
      });
      carrierRows.append(row);
    });
  }

  // Zeichnet beim Öffnen oder Vergrößern zuletzt empfangene Daten neu; erzeugt keine Beispielmessungen.
  function redraw() {
    if (!page.classList.contains('active')) return;
    syncCarriers();
    if (!latestVisual) return;
    const spec = (latestVisual.spectrum_db_tenths || []).map(value => value / 10);
    if (spec.length && typeof drawRfSpectrum === 'function') drawRfSpectrum(spec, latestVisual.sample_rate || 0);
    if (constellationDetails.open && typeof drawRfConstellation === 'function') drawRfConstellation(latestVisual.constellation_iq || []);
    if (typeof drawRfWaterfall === 'function') drawRfWaterfall();
  }

  // Bündelt Layout- und Belegungsänderungen auf einen Browser-Frame.
  function scheduleRefresh() {
    if (scheduled || !page.classList.contains('active')) return;
    scheduled = true;
    requestAnimationFrame(() => { scheduled = false; redraw(); });
  }

  // Häufige Sprachaktivität aktualisiert nur die Belegung, ohne die FFT erneut zu zeichnen.
  function scheduleCarrierRefresh() {
    if (carriersScheduled || !page.classList.contains('active')) return;
    carriersScheduled = true;
    requestAnimationFrame(() => { carriersScheduled = false; syncCarriers(); });
  }

  if (typeof handleTxVisual === 'function') {
    const originalVisualHandler = handleTxVisual;
    handleTxVisual = function netcoreHandleTxVisual(message) {
      latestVisual = message;
      originalVisualHandler(message);
      const hasSpectrum = Boolean(message.spectrum_db_tenths?.length);
      emptyOverlays.get('rf-spectrum').hidden = hasSpectrum;
      emptyOverlays.get('rf-waterfall').hidden = hasSpectrum;
      emptyOverlays.get('rf-constellation').hidden = Boolean(message.constellation_iq?.length);
    };
  }

  const sourceGrid = document.getElementById('ts-grid');
  if (sourceGrid) new MutationObserver(scheduleCarrierRefresh).observe(sourceGrid, {childList: true, subtree: true, attributes: true, characterData: true});
  new MutationObserver(scheduleRefresh).observe(page, {attributes: true, attributeFilter: ['class']});
  new MutationObserver(scheduleRefresh).observe(document.documentElement, {attributes: true, attributeFilter: ['data-theme', 'data-uisize']});
  if (typeof ResizeObserver === 'function') new ResizeObserver(scheduleRefresh).observe(plots);
  constellationDetails.addEventListener('toggle', scheduleRefresh);
  window.addEventListener('resize', scheduleRefresh);
  syncCarriers();
})();
