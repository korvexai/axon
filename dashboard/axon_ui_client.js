// ============================================================================
// AXON UI â†’ Engine WebSocket Client (Production Ready)
// Connects your HTML dashboard to the real Rust backend
// ============================================================================

class AxonEngineClient {
  constructor() {
    this.socket = null;
    this.reconnectDelay = 2000;
    this.isConnected = false;
    this.pendingMessages = [];
    this.eventHandlers = new Map();
  }

  // -------------------------------------------------------------------------
  // Utility: HTML escaping
  // -------------------------------------------------------------------------
  escapeHtml(text) {
    if (typeof text !== 'string') return text;
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }

  // -------------------------------------------------------------------------
  // Connection Management
  // -------------------------------------------------------------------------
  connect(url = 'ws://127.0.0.1:7878/ws') {
    console.log('[WS] Connecting to AXON engine...', url);

    this.socket = new WebSocket(url);

    this.socket.onopen = () => {
      console.log('[WS] Connected to AXON engine');
      this.isConnected = true;
      this.updateConnectionStatus(true);
      
      while (this.pendingMessages.length > 0) {
        const msg = this.pendingMessages.shift();
        this.socket.send(msg);
      }
    };

    this.socket.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        this.handleEngineEvent(data);
      } catch (e) {
        console.error('[WS] Failed to parse engine event:', e);
      }
    };

    this.socket.onerror = (error) => {
      console.error('[WS] WebSocket error:', error);
    };

    this.socket.onclose = () => {
      console.log('[WS] Disconnected from engine. Reconnecting...');
      this.isConnected = false;
      this.updateConnectionStatus(false);
      setTimeout(() => this.connect(url), this.reconnectDelay);
    };
  }

  // -------------------------------------------------------------------------
  // Message Sending
  // -------------------------------------------------------------------------
  send(type, payload = {}) {
    const message = JSON.stringify({ type, payload });

    if (this.isConnected && this.socket.readyState === WebSocket.OPEN) {
      this.socket.send(message);
    } else {
      console.warn('[WS] Not connected, queuing message:', type);
      this.pendingMessages.push(message);
    }
  }

  // -------------------------------------------------------------------------
  // Event Subscription
  // -------------------------------------------------------------------------
  on(eventType, handler) {
    if (!this.eventHandlers.has(eventType)) {
      this.eventHandlers.set(eventType, []);
    }
    this.eventHandlers.get(eventType).push(handler);
  }

  // -------------------------------------------------------------------------
  // Event Routing
  // -------------------------------------------------------------------------
  handleEngineEvent(event) {
    const { type, payload } = event;

    if (this.eventHandlers.has(type)) {
      this.eventHandlers.get(type).forEach(handler => handler(payload));
    }

    switch (type) {
      case 'InitialState': this.handleInitialState(payload); break;
      case 'SystemMetrics': this.handleSystemMetrics(payload); break;
      case 'LogLine': this.handleLogLine(payload); break;
      case 'BuildStarted': this.handleBuildStarted(payload); break;
      case 'BuildLog': this.handleBuildLog(payload); break;
      case 'BuildFinished': this.handleBuildFinished(payload); break;
      case 'ChatResponse': this.handleChatResponse(payload); break;
      case 'RagIndexComplete': this.handleRagIndexComplete(payload); break;
      case 'RagSearchResult': this.handleRagSearchResult(payload); break;
      case 'AlertCreated': this.handleAlertCreated(payload); break;
      case 'WorkerStatusUpdate': this.handleWorkerStatus(payload); break;
      case 'TelegramMessage': this.handleTelegramMessage(payload); break;
      default: console.log('[WS] Unhandled event type:', type, payload);
    }
  }

  // =========================================================================
  // Event Handlers
  // =========================================================================

  handleInitialState(state) {
    console.log('[STATE] Received initial state:', state);

    if (state.session_id) {
      const el = document.getElementById('sess-id');
      if (el) el.textContent = state.session_id;
    }

    if (state.workers) this.updateWorkersList(state.workers);
    if (state.alerts) state.alerts.forEach(alert => this.renderAlert(alert));

    if (state.rag_indexed !== undefined) {
      const val = state.rag_indexed.toLocaleString();
      ['indexed-val', 'rag-total'].forEach(id => {
        const el = document.getElementById(id);
        if (el) el.textContent = val;
      });
    }
  }

  handleSystemMetrics(metrics) {
    const cpuVal = Math.round(metrics.cpu);
    ['cpu-top', 'cpu-r'].forEach(id => {
      const el = document.getElementById(id);
      if (el) el.textContent = cpuVal + '%';
    });

    const cpuBarEl = document.getElementById('cpu-rbar');
    if (cpuBarEl) {
      cpuBarEl.style.width = cpuVal + '%';
      cpuBarEl.style.background = cpuVal > 70 ? 'var(--red)' : 
                                  cpuVal > 50 ? 'var(--yellow)' : 'var(--teal)';
    }

    const ramEl = document.getElementById('ram-top');
    if (ramEl) ramEl.textContent = metrics.ram_gb.toFixed(1) + 'G';
  }

  handleLogLine(log) {
    const panel = document.getElementById('log-panel');
    if (!panel) return;

    const levelClass = (log.level || 'info').toLowerCase();
    const sourceClass = this.getSourceClass(log.source);

    const line = document.createElement('div');
    line.className = 'log-line';
    line.innerHTML = `
      <span class="ll-time">${this.escapeHtml(log.time || '')}</span>
      <span class="ll-src ${sourceClass}">${this.escapeHtml(log.source || '')}</span>
      <span class="ll-text ${levelClass}">${this.escapeHtml(log.message || '')}</span>
    `;

    panel.appendChild(line);
    panel.scrollTop = panel.scrollHeight;

    while (panel.children.length > 100) {
      panel.removeChild(panel.firstChild);
    }
  }

  handleBuildStarted(build) {
    console.log('[BUILD] Started:', build?.project);
    const statusPill = document.querySelector('.status-pill.warn');
    if (statusPill) {
      statusPill.innerHTML = '<span style="width:6px;height:6px;border-radius:50%;background:var(--yellow);animation:pulse 2s infinite;display:inline-block"></span> BUILDING';
    }
  }

  handleBuildLog(log) {
    const panel = document.getElementById('log-panel');
    if (!panel) return;

    const now = new Date();
    const time = `${String(now.getHours()).padStart(2,'0')}:${String(now.getMinutes()).padStart(2,'0')}`;

    const line = document.createElement('div');
    line.className = 'log-line';
    line.innerHTML = `
      <span class="ll-time">${time}</span>
      <span class="ll-src bld">BLD</span>
      <span class="ll-text">${this.escapeHtml(log.line || '')}</span>
    `;

    panel.appendChild(line);
    panel.scrollTop = panel.scrollHeight;
  }

  handleBuildFinished(build) {
    console.log('[BUILD] Finished:', build);
    
    const statusPill = document.querySelector('.status-pill.warn');
    if (statusPill) {
      if (build?.success) {
        statusPill.classList.remove('warn');
        statusPill.classList.add('online');
        statusPill.innerHTML = '<span style="width:6px;height:6px;border-radius:50%;background:var(--teal);box-shadow:0 0 6px var(--teal);display:inline-block"></span> BUILD OK';
      } else {
        statusPill.innerHTML = '<span style="width:6px;height:6px;border-radius:50%;background:var(--red);display:inline-block"></span> BUILD FAIL';
      }
    }

    const statCard = document.querySelector('.stat-card .sc-value.red');
    if (statCard) {
      statCard.textContent = build?.success ? 'OK' : 'FAIL';
      statCard.className = build?.success ? 'sc-value teal' : 'sc-value red';
    }
  }

  handleChatResponse(response) {
    const typing = document.querySelector('.typing-indicator');
    if (typing?.closest('.chat-msg')) {
      typing.closest('.chat-msg').remove();
    }
    this.addChatMessage('ai', response.text, response.model);
  }

  addChatMessage(role, text, model = null) {
    const container = document.getElementById('chat-messages');
    if (!container) return;

    const now = new Date();
    const ts = `${String(now.getHours()).padStart(2,'0')}:${String(now.getMinutes()).padStart(2,'0')}:${String(now.getSeconds()).padStart(2,'0')}`;

    let html = text
      .replace(/```(\w*)\n?([\s\S]*?)```/g, (_, lang, code) => `<div class="msg-code">${this.escapeHtml(code.trim())}</div>`)
      .replace(/`([^`]+)`/g, '<code style="background:var(--bg2);padding:1px 5px;border-radius:3px;font-family:JetBrains Mono,monospace;font-size:11px;color:#a78bfa">$1</code>')
      .replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>')
      .replace(/\n/g, '<br>');

    const avatar = role === 'ai' 
      ? '<div class="msg-avatar ai">AI</div>' 
      : '<div class="msg-avatar user-av">U</div>';

    const modelBadge = model 
      ? `<div style="font-family:JetBrains Mono,monospace;font-size:9px;color:var(--dim);margin-top:2px;">via ${this.escapeHtml(model)}</div>` 
      : '';

    const div = document.createElement('div');
    div.className = `chat-msg ${role}`;
    div.innerHTML = `
      ${avatar}
      <div>
        <div class="msg-bubble">${html}</div>
        <div class="msg-time">${ts}</div>
        ${modelBadge}
      </div>
    `;

    container.appendChild(div);
    container.scrollTop = container.scrollHeight;
  }

  handleRagIndexComplete(data) {
    console.log('[RAG] Indexing complete:', data.total_files);
    const val = data.total_files?.toLocaleString?.() || data.total_files;
    ['indexed-val', 'rag-total'].forEach(id => {
      const el = document.getElementById(id);
      if (el) el.textContent = val;
    });
  }

  handleRagSearchResult(data) {
    console.log('[RAG] Search results:', data);
    
    const resultsPanel = document.getElementById('rag-results');
    const resultsBody = document.getElementById('rag-results-body');
    
    if (!resultsBody) return;
    if (resultsPanel) resultsPanel.style.display = 'flex';
    resultsBody.textContent = '';

    if (!data.results || data.results.length === 0) {
      resultsBody.innerHTML = '<div style="color:var(--dim);padding:10px;">No results found</div>';
      return;
    }

    data.results.forEach(result => {
      const div = document.createElement('div');
      div.style.cssText = 'margin-top:8px;padding:8px;background:var(--bg3);border-radius:4px;border:1px solid var(--border);font-family:JetBrains Mono,monospace;font-size:11px;color:var(--text);line-height:1.7;';
      
      const lineInfo = result.line ? ':' + result.line : '';
      const score = typeof result.score === 'number' ? result.score.toFixed(2) : 'N/A';
      
      div.innerHTML = `
        <div style="color:var(--teal);margin-bottom:4px;">[FILE] ${this.escapeHtml(result.file || '')}${lineInfo} (score: ${score})</div>
        <div>${this.escapeHtml(result.chunk || '')}</div>
      `;
      
      resultsBody.appendChild(div);
    });
  }

  handleAlertCreated(alert) {
    this.renderAlert(alert);
  }

  renderAlert(alert) {
    const alertList = document.querySelector('.alert-list');
    if (alertList) {
      const card = this.createAlertCard(alert);
      alertList.insertBefore(card, alertList.firstChild);
    }
    this.updateAlertBadges();
  }

  createAlertCard(alert) {
    const div = document.createElement('div');
    div.className = `alert-card ${alert.severity || 'info'}`;
    div.dataset.alertId = alert.id || '';

    const iconMap = { critical: '!', warn: '!', info: 'i', ok: 'âœ“' };

    div.innerHTML = `
      <div class="ac-icon">${iconMap[alert.severity] || '?'}</div>
      <div class="ac-body">
        <div class="ac-title">${this.escapeHtml(alert.title || '')}</div>
        <div class="ac-msg">${this.escapeHtml(alert.message || '')}</div>
        ${alert.details ? `<div class="ac-log">${this.escapeHtml(alert.details)}</div>` : ''}
        <div class="ac-actions">
          <button class="ac-btn teal" data-action="apply-fix" data-id="${alert.id || ''}">APPLY FIX</button>
          <button class="ac-btn ghost" data-action="ask-ai">Ask AI</button>
          <button class="ac-btn red" data-action="dismiss" data-id="${alert.id || ''}">DISMISS</button>
        </div>
      </div>
    `;

    div.querySelector('[data-action="apply-fix"]')?.addEventListener('click', () => {
      if (alert.id) this.applyFix(alert.id);
    });
    div.querySelector('[data-action="ask-ai"]')?.addEventListener('click', () => {
      if (typeof switchPage === 'function') switchPage('chat');
    });
    div.querySelector('[data-action="dismiss"]')?.addEventListener('click', () => {
      if (alert.id) this.dismissAlert(alert.id);
    });

    return div;
  }

  handleWorkerStatus(worker) {
    this.updateWorkerInList(worker);
  }

  updateWorkersList(workers) {
    const workerList = document.querySelector('.worker-list');
    if (!workerList) return;
    workerList.textContent = '';
    workers.forEach(worker => workerList.appendChild(this.createWorkerRow(worker)));
  }

  updateWorkerInList(worker) {
    const workerList = document.querySelector('.worker-list');
    if (!workerList) return;

    const existing = Array.from(workerList.children).find(
      row => row.dataset.workerName === worker.name
    );

    if (existing) {
      existing.replaceWith(this.createWorkerRow(worker));
    } else {
      workerList.appendChild(this.createWorkerRow(worker));
    }
  }

  createWorkerRow(worker) {
    const div = document.createElement('div');
    div.className = 'worker-row';
    div.dataset.workerName = worker.name || '';

    const health = worker.health || 'IDLE';
    const healthClass = health === 'RUNNING' ? 'run' : health === 'ERROR' ? 'err' : 'idle';

    div.innerHTML = `
      <div class="w-dot ${healthClass}"></div>
      <div class="w-name">${this.escapeHtml(worker.name || '')}</div>
      <div class="w-status ${healthClass}">${this.escapeHtml(health)}</div>
    `;
    return div;
  }

  handleTelegramMessage(msg) {
    const feed = document.getElementById('tg-feed');
    if (!feed) return;

    const div = document.createElement('div');
    div.className = 'tg-msg';
    div.innerHTML = `
      <div class="tg-icon">${this.escapeHtml(msg.icon || 'T')}</div>
      <div class="tg-body">
        <div class="tg-text">${this.escapeHtml(msg.text || '')}</div>
        <div class="tg-meta">${this.escapeHtml(msg.timestamp || '')} Â· AXON</div>
      </div>
    `;

    feed.insertBefore(div, feed.firstChild);
    while (feed.children.length > 20) feed.removeChild(feed.lastChild);
  }

  // =========================================================================
  // Helper Methods
  // =========================================================================

  updateConnectionStatus(connected) {
    const indicator = document.querySelector('.status-pill.online, .status-pill.warn');
    if (!indicator) return;
    
    if (connected) {
      indicator.className = 'status-pill online';
      indicator.innerHTML = '<span style="width:6px;height:6px;border-radius:50%;background:var(--teal);box-shadow:0 0 6px var(--teal);animation:pulse 2s infinite;display:inline-block"></span> ONLINE';
    } else {
      indicator.className = 'status-pill warn';
      indicator.innerHTML = '<span style="width:6px;height:6px;border-radius:50%;background:var(--red);display:inline-block"></span> OFFLINE';
    }
  }

  updateAlertBadges() {
    const count = document.querySelectorAll('.alert-card').length;
    document.querySelectorAll('.ni-badge').forEach(badge => { badge.textContent = count; });
  }

  getSourceClass(source) {
    const map = { 'SYS': 'sys', 'RAG': 'rag', 'BLD': 'bld', 'AI': 'ai', 'TG': 'tg', 'ERR': 'err' };
    return map[source?.toUpperCase()] || 'sys';
  }

  // =========================================================================
  // Commands to Engine
  // =========================================================================

  sendChat(message) {
    this.send('Chat', { message });
    this.addChatMessage('user', message);
    this.showTypingIndicator();
  }

  rebuild(project = null) {
    this.send('Rebuild', { project });
  }

  applyFix(alertId) {
    this.send('ApplyFix', { alert_id: alertId });
    console.log('[CMD] Applying fix:', alertId);
  }

  searchRag(query) {
    this.send('RagSearch', { query });
  }

  executeCommand(command) {
    this.send('ExecuteCommand', { command });
  }

  dismissAlert(alertId) {
    const card = document.querySelector(`[data-alert-id="${alertId}"]`);
    if (card) card.remove();
    this.updateAlertBadges();
  }

  showTypingIndicator() {
    const container = document.getElementById('chat-messages');
    if (!container) return;

    const div = document.createElement('div');
    div.className = 'chat-msg ai';
    div.innerHTML = `
      <div class="msg-avatar ai">AI</div>
      <div>
        <div class="msg-bubble">
          <div class="typing-indicator">
            <span class="typing-dot"></span>
            <span class="typing-dot"></span>
            <span class="typing-dot"></span>
          </div>
        </div>
      </div>
    `;

    container.appendChild(div);
    container.scrollTop = container.scrollHeight;
  }
}

// ============================================================================
// Global Initialization
// ============================================================================
const axon = new AxonEngineClient();

document.addEventListener('DOMContentLoaded', () => {
  axon.connect();
});

// ============================================================================
// UI Helper Functions (global scope for HTML onclick handlers)
// ============================================================================

function sendMessage() {
  const input = document.getElementById('chat-input');
  const msg = input?.value.trim();
  if (!msg) return;
  axon.sendChat(msg);
  if (input) {
    input.value = '';
    input.style.height = 'auto';
  }
}

function simulateBuild() {
  axon.rebuild();
}

function sendQuick(text) {
  const input = document.getElementById('chat-input');
  if (input) {
    input.value = text;
    sendMessage();
  }
}

function ragSearch(query) {
  if (!query) {
    const panel = document.getElementById('rag-results');
    if (panel) panel.style.display = 'none';
    return;
  }
  axon.searchRag(query);
}

// Optional: inject CSS for typing indicator if not present
if (!document.getElementById('axon-typing-styles')) {
  const style = document.createElement('style');
  style.id = 'axon-typing-styles';
  style.textContent = `
    @keyframes pulse { 0%,100%{opacity:1} 50%{opacity:.4} }
    .typing-indicator { display:flex; gap:3px; padding:4px 0; }
    .typing-dot { width:6px; height:6px; border-radius:50%; background:var(--dim); animation:pulse 1.4s infinite ease-in-out; }
    .typing-dot:nth-child(2){animation-delay:.2s}
    .typing-dot:nth-child(3){animation-delay:.4s}
  `;
  document.head.appendChild(style);
}
// â† END OF FILE â€” no more code, no orphaned braces
