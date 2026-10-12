import assert from 'node:assert/strict';
import fs from 'node:fs';
import vm from 'node:vm';

const policy = new URL('../system-backend/deployment-core/image/49-netcore-networkmanager.rules', import.meta.url);
let rule;
vm.runInNewContext(fs.readFileSync(policy, 'utf8'), {
  polkit: { addRule(callback) { rule = callback; }, Result: { YES: 'yes' } },
});
assert.equal(typeof rule, 'function');
const allowed = [
  'enable-disable-wifi', 'wifi.scan', 'network-control', 'settings.modify.system',
];
for (const action of allowed) {
  assert.equal(rule({ id: `org.freedesktop.NetworkManager.${action}` }, { user: 'netcore' }), 'yes');
  assert.equal(rule({ id: `org.freedesktop.NetworkManager.${action}` }, { user: 'someone-else' }), undefined);
}
for (const action of ['enable-disable-network', 'enable-disable-wwan', 'reload', 'settings.modify.hostname', 'settings.modify.system-extra']) {
  assert.equal(rule({ id: `org.freedesktop.NetworkManager.${action}` }, { user: 'netcore' }), undefined);
}
assert.equal(rule({ id: 'org.freedesktop.systemd1.manage-units' }, { user: 'netcore' }), undefined);
console.log('WiFi Polkit: permitted WiFi operations, unrelated users/actions preserved');
