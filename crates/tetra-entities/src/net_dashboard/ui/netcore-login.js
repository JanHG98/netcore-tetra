/* NetCore-Tetra Anmeldung: öffentliche Ansicht und Sitzung bleiben getrennte API-Abläufe. */
(() => {
  'use strict';

  const form = document.getElementById('login-form');
  const errBox = document.getElementById('err');
  const button = document.getElementById('submit-btn');
  const buttonLabel = document.getElementById('submit-label');
  const username = document.getElementById('username');
  const password = document.getElementById('password');
  const passwordToggle = document.getElementById('password-toggle');

  // Was: Zeigt das Passwort auf ausdrücklichen Tastendruck und behält den Fokus am Schalter.
  // Warum: Die Eingabe bleibt mit Tastatur, Touch und Passwortmanager verwendbar.
  passwordToggle.addEventListener('click', () => {
    const visible = password.type === 'password';
    password.type = visible ? 'text' : 'password';
    passwordToggle.setAttribute('aria-pressed', String(visible));
    passwordToggle.setAttribute('aria-label', visible ? 'Passwort verbergen' : 'Passwort anzeigen');
    passwordToggle.title = visible ? 'Passwort verbergen' : 'Passwort anzeigen';
  });

  // Was: Sendet ausschließlich die vorhandenen Zugangsdaten an den Sitzungsendpunkt.
  // Warum: Die serverseitige Cookie-Authentifizierung bleibt unverändert maßgeblich.
  form.addEventListener('submit', async (event) => {
    event.preventDefault();
    if (button.disabled) return;
    errBox.textContent = '';
    button.disabled = true;
    buttonLabel.textContent = 'Anmeldung läuft…';
    form.setAttribute('aria-busy', 'true');
    const controller = new AbortController();
    const timeout = window.setTimeout(() => controller.abort(), 15000);
    try {
      const response = await fetch('/api/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ user: username.value, password: password.value }),
        credentials: 'same-origin',
        signal: controller.signal,
      });
      if (response.ok) {
        window.location.assign('/');
        return;
      }
      errBox.textContent = response.status === 401
        ? 'Benutzername oder Passwort ist falsch.'
        : 'Anmeldung fehlgeschlagen (' + response.status + ').';
    } catch (error) {
      errBox.textContent = error.name === 'AbortError'
        ? 'Die Anmeldung hat zu lange gedauert. Bitte erneut versuchen.'
        : 'Die Basisstation ist gerade nicht erreichbar. Bitte erneut versuchen.';
    } finally {
      window.clearTimeout(timeout);
      button.disabled = false;
      buttonLabel.textContent = 'Anmelden';
      form.removeAttribute('aria-busy');
    }
  });

  // Was: Fokussiert das erste Feld nur auf größeren Ansichten.
  // Warum: Auf Mobilgeräten soll die Tastatur nicht ungefragt die Stationsvorschau verdecken.
  if (window.innerWidth > 850) username.focus();

  const overview = document.getElementById('public-status');
  const statusIcon = document.getElementById('public-status-icon');
  const statusLabel = document.getElementById('public-status-label');
  const statusDetail = document.getElementById('public-status-detail');
  const updated = document.getElementById('public-updated');
  const numberFormat = new Intl.NumberFormat('de-DE', { maximumFractionDigits: 1 });
  const integerFormat = new Intl.NumberFormat('de-DE', { maximumFractionDigits: 0 });
  let pollTimer = null;
  let publicRequest = null;
  let publicDisabled = false;
  let lastSuccess = null;
  let pageActive = true;

  // Was: Formatiert ausschließlich vorhandene Messwerte; null ist ausdrücklich nicht null Prozent.
  // Warum: Fehlende Telemetrie darf keinen gesunden oder gemessenen Wert vortäuschen.
  function setMetric(id, value, suffix = '') {
    const element = document.getElementById(id);
    const available = typeof value === 'number' && Number.isFinite(value)
      && (id !== 'public-load' || (value >= 0 && value <= 100));
    element.textContent = available ? numberFormat.format(value) + suffix : 'Nicht verfügbar';
    element.classList.toggle('unavailable', !available);
  }

  function uptimeText(seconds) {
    if (typeof seconds !== 'number' || !Number.isFinite(seconds) || seconds < 0) return null;
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    if (hours >= 24) return Math.floor(hours / 24) + ' d ' + (hours % 24) + ' h';
    if (hours > 0) return hours + ' h ' + (minutes % 60) + ' min';
    return minutes + ' min';
  }

  function setStatus(state, label, detail, icon) {
    overview.dataset.state = state;
    statusLabel.textContent = label;
    statusDetail.textContent = detail;
    statusIcon.textContent = icon;
  }

  function clearMetrics() {
    ['public-devices', 'public-uptime', 'public-temperature', 'public-load'].forEach((id) => {
      const element = document.getElementById(id);
      element.textContent = '—';
      element.classList.remove('unavailable');
    });
  }

  // Was: Holt nur die ausdrücklich freigegebene Aggregatansicht, mit Abbruch und begrenztem Takt.
  // Warum: Vor der Anmeldung werden weder privilegierte APIs noch WebSockets geöffnet.
  async function pollPublicStatus() {
    if (!pageActive || publicDisabled || document.hidden || publicRequest) return;
    const controller = new AbortController();
    publicRequest = controller;
    const timeout = window.setTimeout(() => controller.abort(), 5000);
    try {
      const response = await fetch('/api/public', {
        credentials: 'same-origin',
        cache: 'no-store',
        signal: controller.signal,
      });
      if ([401, 403, 404].includes(response.status)) {
        publicDisabled = true;
        clearMetrics();
        setStatus('disabled', 'Öffentlicher Status deaktiviert', 'Stationsdaten sind nach Anmeldung verfügbar.', '·');
        updated.textContent = '';
        return;
      }
      if (!response.ok) throw new Error('public_status_unavailable');
      const snapshot = await response.json();
      if (!snapshot || typeof snapshot !== 'object' || Array.isArray(snapshot) || !Number.isInteger(snapshot.registered_ms) || snapshot.registered_ms < 0) {
        throw new Error('public_status_invalid');
      }
      lastSuccess = new Date();
      const devices = document.getElementById('public-devices');
      devices.textContent = integerFormat.format(snapshot.registered_ms);
      devices.classList.remove('unavailable');
      const uptime = document.getElementById('public-uptime');
      const duration = uptimeText(snapshot.uptime_secs);
      uptime.textContent = duration || 'Nicht verfügbar';
      uptime.classList.toggle('unavailable', !duration);
      setMetric('public-temperature', snapshot.cpu_temp_c, ' °C');
      setMetric('public-load', snapshot.cpu_load_pct, ' %');
      if (snapshot.health_status === 'ok') {
        setStatus('ok', 'Basisstation betriebsbereit', 'Öffentlicher Betriebsstatus', '✓');
      } else if (snapshot.health_status === 'degraded') {
        setStatus('degraded', 'Betrieb eingeschränkt', 'Die Basisstation meldet einen eingeschränkten Zustand.', '!');
      } else if (snapshot.health_status === 'critical') {
        setStatus('critical', 'Betriebszustand kritisch', 'Details sind nach Anmeldung verfügbar.', '!');
      } else {
        setStatus('unknown', 'Basisstation erreichbar', 'Eine vollständige Zustandsbewertung ist nicht verfügbar.', '·');
      }
      updated.textContent = 'Status abgerufen um ' + lastSuccess.toLocaleTimeString('de-DE', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
    } catch (error) {
      if (!pageActive || document.hidden) return;
      if (lastSuccess) {
        setStatus('stale', 'Status derzeit nicht aktuell', 'Die angezeigten Werte stammen aus dem letzten Abruf.', '!');
        updated.textContent = 'Letzter erfolgreicher Abruf: ' + lastSuccess.toLocaleTimeString('de-DE', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
      } else {
        clearMetrics();
        setStatus('unavailable', 'Status derzeit nicht verfügbar', 'Bitte später erneut versuchen.', '·');
        updated.textContent = '';
      }
    } finally {
      window.clearTimeout(timeout);
      publicRequest = null;
      if (pageActive && !publicDisabled && !document.hidden) pollTimer = window.setTimeout(pollPublicStatus, 10000);
    }
  }

  document.addEventListener('visibilitychange', () => {
    window.clearTimeout(pollTimer);
    if (document.hidden) {
      if (publicRequest) publicRequest.abort();
    } else {
      pollPublicStatus();
    }
  });
  window.addEventListener('pagehide', () => {
    pageActive = false;
    window.clearTimeout(pollTimer);
    if (publicRequest) publicRequest.abort();
  });
  window.addEventListener('pageshow', (event) => {
    if (event.persisted) {
      pageActive = true;
      pollPublicStatus();
    }
  });
  pollPublicStatus();
})();
