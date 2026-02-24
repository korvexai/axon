// ═══════════════════════════════════════════════════════════════
// AXON UI → Engine WebSocket Client (Production Ready)
// Connects your HTML dashboard to the real Rust backend
// ═══════════════════════════════════════════════════════════════

class AxonEngineClient {
  constructor() {
    this.socket = null;
    this.reconnectDelay = 2000;
    this.isConnected = false;
    this.pendingMessages = [];
    this.eventHandlers = new Map();
  }

  connect(url = 'ws://127.0.0.1:7878/ws') {
    console.log('🔌 Connecting to AXON engine...', url);

    this.socket = new WebSocket(url);

    this.socket.onopen = () => {
      console.log('✅ Connected to AXON engine');
      this.isConnected = true;
      this.updateConnectionStatus(true);
      
      // Send any queued messages
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
        console.error('Failed to parse engine event:', e);
      }
    };

    this.socket.onerror = (error) => {
      console.error('❌ WebSocket error:', error);
    };

    this.socket.onclose = () => {
      console.log('🔌 Disconnected from engine. Reconnecting...');
      this.isConnected = false;
      this.updateConnectionStatus(false);
      setTimeout(() => this.connect(url), this.reconnectDelay);
    };
  }

  send(type, payload = {}) {
    const message = JSON.stringify({ type, payload });

    if (this.isConnected && this.socket.readyState === WebSocket.OPEN) {
      this.socket.send(message);
    } else {
      console.warn('⚠️ Not connected, queuing message:', type);
      this.pendingMessages.push(message);
    }
  }

  on(eventType, handler) {
    if (!this.eventHandlers.has(eventType)) {
      this.eventHandlers.set(eventType, []);
    }
    this.eventHandlers.get(eventType).push(handler);
  }

  handleEngineEvent(event) {
    const { type, payload } = event;

    // Call registered handlers
    if (this.eventHandlers.has(type)) {
      this.eventHandlers.get(type).forEach(handler => handler(payload));
    }

    // Built-in routing
    switch (type) {
      case 'InitialState':
        this.handleInitialState(payload);
        break;

      case 'SystemMetrics':
        this.handleSystemMetrics(payload);
        break;

      case 'LogLine':
        this.handleLogLine(payload);
        break;

      case 'BuildStarted':
        this.handleBuildStarted(payload);
        break;

      case 'BuildLog':
        this.handleBuildLog(payload);
        break;

      case 'BuildFinished':
        this.handleBuildFinished(payload);
        break;

      case 'ChatResponse':
        this.handleChatResponse(payload);
        break;

      case 'RagIndexComplete':
        this.handleRagIndexComplete(payload);
        break;

      case 'RagSearchResult':
        this.handleRagSearchResult(payload);
        break;

      case 'AlertCreated':
        this.handleAlertCreated(payload);
        break;

      case 'WorkerStatusUpdate':
        this.handleWorkerStatus(payload);
        break;

      case 'TelegramMessage':
        this.handleTelegramMessage(payload);
        break;

      default:
        console.log('Unhandled event type:', type, payload);
    }
  }

  // ═══ Initial State ═══
  handleInitialState(state) {
    console.log('📦 Received initial state:', state);

    // Update session info
    if (state.session_id) {
      const el = document.getElementById('sess-id');
      if (el) el.textContent = state.session_id;
    }

    // Update workers
    if (state.workers) {
      this.updateWorkersList(state.workers);
    }

    // Update alerts
    if (state.alerts) {
      state.alerts.forEach(alert => this.renderAlert(alert));
    }

    // Update RAG stats
    if (state.rag_indexed !== undefined) {
      const el = document.getElementById('indexed-val');
      if (el) el.textContent = state.rag_indexed.toLocaleString();
      
      const el2 = document.getElementById('rag-total');
      if (el2) el2.textContent = state.rag_indexed.toLocaleString();
    }

    // Update safe mode indicator
    if (state.safe_mode !== undefined) {
      // Update UI to show safe mode status
    }
  }

  // ═══ System Metrics ═══
  handleSystemMetrics(metrics) {
    // Update CPU
    const cpuEl = document.getElementById('cpu-top');
    const cpuREl = document.getElementById('cpu-r');
    const cpuBarEl = document.getElementById('cpu-rbar');
    
    if (cpuEl) cpuEl.textContent = Math.round(metrics.cpu) + '%';
    if (cpuREl) cpuREl.textContent = Math.round(metrics.cpu) + '%';
    if (cpuBarEl) {
      cpuBarEl.style.width = Math.round(metrics.cpu) + '%';
      const color = metrics.cpu > 70 ? 'var(--red)' : 
                    metrics.cpu > 50 ? 'var(--yellow)' : 'var(--teal)';
      cpuBarEl.style.background = color;
    }

    // Update RAM
    const ramEl = document.getElementById('ram-top');
    if (ramEl) ramEl.textContent = metrics.ram_gb.toFixed(1) + 'G';
  }

  // ═══ Log Lines ═══
  handleLogLine(log) {
    const panel = document.getElementById('log-panel');
    if (!panel) return;

    const levelClass = log.level.toLowerCase();
    const sourceClass = this.getSourceClass(log.source);

    const line = document.createElement('div');
    line.className = 'log-line';
    line.innerHTML = `
      <span class="ll-time">${log.time}</span>
      <span class="ll-src ${sourceClass}">${log.source}</span>
      <span class="ll-text ${levelClass}">${this.escapeHtml(log.message)}</span>
    `;

    panel.appendChild(line);
    
    // Auto-scroll
    panel.scrollTop = panel.scrollHeight;

    // Keep only last 100 lines
    while (panel.children.length > 100) {
      panel.removeChild(panel.firstChild);
    }
  }

  // ═══ Build System ═══
  handleBuildStarted(build) {
    console.log('🔨 Build started:', build.project);
    // Update UI build indicator
    const statusPill = document.querySelector('.status-pill.warn');
    if (statusPill) {
      statusPill.innerHTML = '<span style="width:6px;height:6px;border-radius:50%;background:var(--yellow);animation:pulse 2s infinite"></span>BUILDING';
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
      <span class="ll-text">${this.escapeHtml(log.line)}</span>
    `;

    panel.appendChild(line);
    panel.scrollTop = panel.scrollHeight;
  }

  handleBuildFinished(build) {
    console.log('🔨 Build finished:', build);
    
    const statusPill = document.querySelector('.status-pill.warn');
    if (statusPill) {
      if (build.success) {
        statusPill.classList.remove('warn');
        statusPill.classList.add('online');
        statusPill.innerHTML = '<span style="width:6px;height:6px;border-radius:50%;background:var(--teal);box-shadow:0 0 6px var(--teal)"></span>BUILD OK';
      } else {
        statusPill.innerHTML = '<span style="width:6px;height:6px;border-radius:50%;background:var(--red)"></span>BUILD FAIL';
      }
    }

    // Update stats card
    const statCard = document.querySelector('.stat-card .sc-value.red');
    if (statCard) {
      statCard.textContent = build.success ? 'OK' : 'FAIL';
      statCard.className = build.success ? 'sc-value teal' : 'sc-value red';
    }
  }

  // ═══ AI Chat ═══
  handleChatResponse(response) {
    const typing = document.querySelector('.typing-indicator');
    if (typing) {
      typing.closest('.chat-msg').remove();
    }

    this.addChatMessage('ai', response.text, response.model);
  }

  addChatMessage(role, text, model = null) {
    const container = document.getElementById('chat-messages');
    if (!container) return;

    const now = new Date();
    const ts = `${String(now.getHours()).padStart(2,'0')}:${String(now.getMinutes()).padStart(2,'0')}:${String(now.getSeconds()).padStart(2,'0')}`;

    // Convert markdown-style formatting
    let html = text
      .replace(/```(\w*)\n?([\s\S]*?)```/g, (_, lang, code) => {
        return `<div class="msg-code">${this.escapeHtml(code.trim())}</div>`;
      })
      .replace(/`([^`]+)`/g, '<code style="background:var(--bg2);padding:1px 5px;border-radius:3px;font-family:JetBrains Mono,monospace;font-size:11px;color:#a78bfa">$1</code>')
      .replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>')
      .replace(/\n/g, '<br>');

    const avatar = role === 'ai' ? 
      '<div class="msg-avatar ai">⬡</div>' : 
      '<div class="msg-avatar user-av">▶</div>';

    const modelBadge = model ? `<div style="font-family:JetBrains Mono,monospace;font-size:9px;color:var(--dim);margin-top:2px;">via ${model}</div>` : '';

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

  // ═══ RAG ═══
  handleRagIndexComplete(data) {
    console.log('📚 RAG indexing complete:', data.total_files);
    
    const el = document.getElementById('indexed-val');
    if (el) el.textContent = data.total_files.toLocaleString();

    const el2 = document.getElementById('rag-total');
    if (el2) el2.textContent = data.total_files.toLocaleString();
  }

  handleRagSearchResult(data) {
    console.log('🔍 RAG search results:', data);
    
    const resultsPanel = document.getElementById('rag-results');
    const resultsBody = document.getElementById('rag-results-body');
    
    if (!resultsBody) return;

    resultsPanel.style.display = 'flex';
    resultsBody.innerHTML = '';

    if (data.results.length === 0) {
      resultsBody.innerHTML = '<div style="color:var(--dim);padding:10px;">No results found</div>';
      return;
    }

    data.results.forEach(result => {
      const div = document.createElement('div');
      div.style.cssText = 'margin-top:8px;padding:8px;background:var(--bg3);border-radius:4px;border:1px solid var(--border);font-family:JetBrains Mono,monospace;font-size:11px;color:var(--text);line-height:1.7;';
      
      div.innerHTML = `
        <div style="color:var(--teal);margin-bottom:4px;">📄 ${result.file}${result.line ? ':' + result.line : ''} (score: ${result.score.toFixed(2)})</div>
        <div>${this.escapeHtml(result.chunk)}</div>
      `;
      
      resultsBody.appendChild(div);
    });
  }

  // ═══ Alerts ═══
  handleAlertCreated(alert) {
    this.renderAlert(alert);
  }

  renderAlert(alert) {
    // Render in main alert list
    const alertList = document.querySelector('.alert-list');
    if (alertList) {
      const card = this.createAlertCard(alert);
      alertList.insertBefore(card, alertList.firstChild);
    }

    // Update alert count badges
    this.updateAlertBadges();
  }

  createAlertCard(alert) {
    const div = document.createElement('div');
    div.className = `alert-card ${alert.severity}`;
    div.dataset.alertId = alert.id;

    const iconMap = {
      critical: '🔴',
      warn: '🟡',
      info: '🔵',
      ok: '🟢'
    };

    div.innerHTML = `
      <div class="ac-icon">${iconMap[alert.severity] || '⚪'}</div>
      <div class="ac-body">
        <div class="ac-title">${this.escapeHtml(alert.title)}</div>
        <div class="ac-msg">${this.escapeHtml(alert.message)}</div>
        ${alert.details ? `<div class="ac-log">${this.escapeHtml(alert.details)}</div>` : ''}
        <div class="ac-actions">
          <button class="ac-btn teal" onclick="axon.applyFix('${alert.id}')">⚡ APPLY FIX</button>
          <button class="ac-btn ghost" onclick="switchPage('chat')">◎ Ask AI</button>
          <button class="ac-btn red" onclick="axon.dismissAlert('${alert.id}')">⊘ DISMISS</button>
        </div>
      </div>
    `;

    return div;
  }

  // ═══ Workers ═══
  handleWorkerStatus(worker) {
    this.updateWorkerInList(worker);
  }

  updateWorkersList(workers) {
    const workerList = document.querySelector('.worker-list');
    if (!workerList) return;

    workerList.innerHTML = '';

    workers.forEach(worker => {
      const row = this.createWorkerRow(worker);
      workerList.appendChild(row);
    });
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
    div.dataset.workerName = worker.name;

    const healthClass = worker.health === 'RUNNING' ? 'run' : 
                       worker.health === 'ERROR' ? 'err' : 'idle';

    div.innerHTML = `
      <div class="w-dot ${healthClass}"></div>
      <div class="w-name">${worker.name}</div>
      <div class="w-status ${healthClass}">${worker.health}</div>
    `;

    return div;
  }

  // ═══ Telegram ═══
  handleTelegramMessage(msg) {
    const feed = document.getElementById('tg-feed');
    if (!feed) return;

    const div = document.createElement('div');
    div.className = 'tg-msg';
    div.innerHTML = `
      <div class="tg-icon">${msg.icon}</div>
      <div class="tg-body">
        <div class="tg-text">${this.escapeHtml(msg.text)}</div>
        <div class="tg-meta">${msg.timestamp} · AXON</div>
      </div>
    `;

    feed.insertBefore(div, feed.firstChild);

    // Keep only last 20
    while (feed.children.length > 20) {
      feed.removeChild(feed.lastChild);
    }
  }

  // ═══ Helper Methods ═══
  updateConnectionStatus(connected) {
    const indicator = document.querySelector('.status-pill.online');
    if (indicator) {
      if (connected) {
        indicator.innerHTML = '<span style="width:6px;height:6px;border-radius:50%;background:var(--teal);box-shadow:0 0 6px var(--teal);animation:pulse 2s infinite"></span>ONLINE';
      } else {
        indicator.innerHTML = '<span style="width:6px;height:6px;border-radius:50%;background:var(--red)"></span>OFFLINE';
        indicator.classList.remove('online');
        indicator.classList.add('warn');
      }
    }
  }

  updateAlertBadges() {
    const count = document.querySelectorAll('.alert-card').length;
    document.querySelectorAll('.ni-badge').forEach(badge => {
      badge.textContent = count;
    });
  }

  getSourceClass(source) {
    const map = {
      'SYS': 'sys',
      'RAG': 'rag',
      'BLD': 'bld',
      'AI': 'ai',
      'TG': 'tg',
      'ERR': 'err'
    };
    return map[source] || 'sys';
  }

  escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }

  // ═══ Commands to Engine ═══
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
    console.log('⚡ Applying fix:', alertId);
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
      <div class="msg-avatar ai">⬡</div>
      <div>
        <div class="msg-bubble">
          <div class="typing-indicator">
            <div class="typing-dot"></div>
            <div class="typing-dot"></div>
            <div class="typing-dot"></div>
          </div>
        </div>
      </div>
    `;

    container.appendChild(div);
    container.scrollTop = container.scrollHeight;
  }
}

// ═══════════════════════════════════════════════════════════════
// Initialize global AXON client
// ═══════════════════════════════════════════════════════════════
const axon = new AxonEngineClient();

// Connect when page loads
document.addEventListener('DOMContentLoaded', () => {
  axon.connect();
});

// ═══════════════════════════════════════════════════════════════
// Update UI functions to use real engine
// ═══════════════════════════════════════════════════════════════

function sendMessage() {
  const input = document.getElementById('chat-input');
  const msg = input.value.trim();
  if (!msg) return;

  axon.sendChat(msg);
  
  input.value = '';
  input.style.height = 'auto';
}

function simulateBuild() {
  axon.rebuild();
}

function sendQuick(text) {
  const input = document.getElementById('chat-input');
  input.value = text;
  sendMessage();
}

function ragSearch(query) {
  if (!query) {
    document.getElementById('rag-results').style.display = 'none';
    return;
  }
  axon.searchRag(query);
}

// Keep existing page switching, clock, etc...
