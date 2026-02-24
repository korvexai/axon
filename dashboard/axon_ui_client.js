// ============================================================================
// AXON UI -> Engine WebSocket Client (Production Ready)
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

  send(type, payload = {}) {
    const message = JSON.stringify({ type, payload });
    if (this.isConnected && this.socket.readyState === WebSocket.OPEN) {
      this.socket.send(message);
    } else {
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
    }
  }

  handleInitialState(state) {
    if (state.session_id) {
      const el = document.getElementById('sess-id');
      if (el) el.textContent = state.session_id;
    }
    if (state.workers) this.updateWorkersList(state.workers);
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
      cpuBarEl.style.background = cpuVal > 70 ? 'var(--red)' : cpuVal > 50 ? 'var(--yellow)' : 'var(--teal)';
    }
    const ramEl = document.getElementById('ram-top');
    if (ramEl) ramEl.textContent = metrics.ram_gb.toFixed(1) + 'G';
  }

  handleLogLine(log) {
    const panel = document.getElementById('log-panel');
    if (!panel) return;
    const line = document.createElement('div');
    line.className = 'log-line';
    line.innerHTML = `
      <span class="ll-time">${this.escapeHtml(log.time || '')}</span>
      <span class="ll-src sys">${this.escapeHtml(log.source || '')}</span>
      <span class="ll-text info">${this.escapeHtml(log.message || '')}</span>
    `;
    panel.appendChild(line);
    panel.scrollTop = panel.scrollHeight;
  }

  handleChatResponse(response) {
    const typing = document.querySelector('.typing-indicator');
    if (typing?.closest('.chat-msg')) typing.closest('.chat-msg').remove();
    this.addChatMessage('ai', response.text, response.model);
  }

  addChatMessage(role, text, model = null) {
    const container = document.getElementById('chat-messages');
    if (!container) return;

    const now = new Date();
    const ts = now.toLocaleTimeString();

    // Fix: AceastÄƒ parte proceseazÄƒ Markdown-ul simplu
    let html = text
      .replace(/```(\w*)\n?([\s\S]*?)```/g, (_, lang, code) => `<div class="msg-code">${this.escapeHtml(code.trim())}</div>`)
      .replace(/`([^`]+)`/g, '<code>$1</code>')
      .replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>')
      .replace(/\n/g, '<br>');

    const avatar = role === 'ai' ? '<div class="msg-avatar ai">AI</div>' : '<div class="msg-avatar user-av">U</div>';
    
    const div = document.createElement('div');
    div.className = `chat-msg ${role}`;
    div.innerHTML = `
      ${avatar}
      <div>
        <div class="msg-bubble">${html}</div>
        <div class="msg-time">${ts}</div>
      </div>
    `;

    container.appendChild(div);
    container.scrollTop = container.scrollHeight;
  }

  updateWorkersList(workers) {
    const workerList = document.querySelector('.worker-list');
    if (!workerList) return;
    workerList.innerHTML = '';
    workers.forEach(worker => {
        const div = document.createElement('div');
        div.className = 'worker-row';
        div.innerHTML = `<div class="w-dot run"></div><div class="w-name">${worker.name}</div>`;
        workerList.appendChild(div);
    });
  }

  updateConnectionStatus(connected) {
    const indicator = document.querySelector('.status-pill');
    if (!indicator) return;
    indicator.className = connected ? 'status-pill online' : 'status-pill warn';
    indicator.innerHTML = connected ? 'ONLINE' : 'OFFLINE';
  }

  sendChat(message) {
    this.send('Chat', { message });
    this.addChatMessage('user', message);
  }
}

const axon = new AxonEngineClient();
document.addEventListener('DOMContentLoaded', () => axon.connect());

function sendMessage() {
  const input = document.getElementById('chat-input');
  if (input?.value.trim()) {
    axon.sendChat(input.value.trim());
    input.value = '';
  }
}