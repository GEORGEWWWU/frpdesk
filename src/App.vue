<template>
  <div class="app-container">
    <!-- 顶部标题栏 -->
    <div class="titlebar">
      <!-- 仅在中间空白/文本区域开启拖拽 -->
      <div class="title-text" data-tauri-drag-region>FRP Desk</div>

      <!-- 按钮区域：调整到代码下方并靠右对齐，符合正常逻辑 -->
      <div class="window-controls">
        <div class="control minimize" @click.stop="minimizeWindow" title="最小化">
          <!-- 最小化 SVG 图标 -->
          <svg width="12" height="12" viewBox="0 0 12 12">
            <path d="M1,6 H11" stroke="currentColor" stroke-width="1.5" />
          </svg>
        </div>
        <div class="control close" @click.stop="closeWindow" title="关闭">
          <!-- 关闭 SVG 图标 -->
          <svg width="12" height="12" viewBox="0 0 12 12">
            <path d="M2,2 L10,10 M10,2 L2,10" stroke="currentColor" stroke-width="1.5" />
          </svg>
        </div>
      </div>
    </div>

    <div class="app-body">
      <!-- 左侧边栏 -->
      <div class="sidebar">
        <div class="nav-item" :class="{ active: currentTab === 'dashboard' }" @click="currentTab = 'dashboard'">
          <!-- 概览：宫格图标 -->
          <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2"
            stroke-linecap="round" stroke-linejoin="round">
            <rect x="3" y="3" width="7" height="7" rx="1"></rect>
            <rect x="14" y="3" width="7" height="7" rx="1"></rect>
            <rect x="14" y="14" width="7" height="7" rx="1"></rect>
            <rect x="3" y="14" width="7" height="7" rx="1"></rect>
          </svg>
          <span>概览</span>
        </div>

        <div class="nav-item" :class="{ active: currentTab === 'log' }" @click="currentTab = 'log'">
          <!-- 运行日志：终端命令行图标 -->
          <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2"
            stroke-linecap="round" stroke-linejoin="round">
            <polyline points="4 17 10 11 4 5"></polyline>
            <line x1="12" y1="19" x2="20" y2="19"></line>
          </svg>
          <span>运行日志</span>
        </div>

        <div class="nav-item" :class="{ active: currentTab === 'config' }" @click="currentTab = 'config'">
          <!-- 参数配置：调节滑块图标 -->
          <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2"
            stroke-linecap="round" stroke-linejoin="round">
            <line x1="4" y1="21" x2="4" y2="14"></line>
            <line x1="4" y1="10" x2="4" y2="3"></line>
            <line x1="12" y1="21" x2="12" y2="12"></line>
            <line x1="12" y1="8" x2="12" y2="3"></line>
            <line x1="20" y1="21" x2="20" y2="16"></line>
            <line x1="20" y1="12" x2="20" y2="3"></line>
            <line x1="1" y1="14" x2="7" y2="14"></line>
            <line x1="9" y1="8" x2="15" y2="8"></line>
            <line x1="17" y1="16" x2="23" y2="16"></line>
          </svg>
          <span>参数配置</span>
        </div>

        <div class="nav-item" :class="{ active: currentTab === 'software' }" @click="currentTab = 'software'">
          <!-- 软件设置：齿轮图标 -->
          <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2"
            stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="3"></circle>
            <path
              d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z">
            </path>
          </svg>
          <span>软件设置</span>
        </div>
      </div>

      <!-- 右侧主内容区 -->
      <div class="main-content">
        <!-- 概览页面 (现代 Bento 布局) -->
        <div v-if="currentTab === 'dashboard'" class="page dashboard-page">
          <div class="page-title">
            <h2>系统概览</h2>
          </div>

          <div class="bento-grid">
            <!-- 运行时长 -->
            <div class="bento-card uptime-card">
              <div class="card-label">连续运行时长</div>
              <div class="uptime-value">{{ uptimeStr }}</div>
            </div>

            <!-- 核心引擎 -->
            <div class="bento-card control-card">
              <div class="card-header">
                <span class="card-title-main">FRP 核心引擎</span>
                <div :class="['status-badge', isRunning ? 'running' : 'stopped']">
                  <span class="dot"></span>
                  {{ isRunning ? '服务稳定运行中' : '服务当前已停止' }}
                </div>
              </div>
              <div class="card-body">
                <p v-if="!isReady" class="warning-text">⚠️ 请先在“软件设置”中配置路径并加载参数</p>
                <p v-else class="subtitle">底层网络代理服务已就绪，请控制转发引擎状态。</p>
                <div class="action-area">
                  <button class="btn modern-btn primary" v-if="!isRunning" :disabled="!isReady" @click="startFRP">
                    启动 FRP 代理
                  </button>
                  <button class="btn modern-btn danger" v-else @click="stopFRP">
                    强制停止服务
                  </button>
                </div>
              </div>
            </div>

            <!-- 本地代理 -->
            <div class="bento-card info-card">
              <div class="card-label">本地代理 (Local)</div>
              <div class="info-value">{{ configLoaded ? `${frpConfig.localIP}:${frpConfig.localPort}` : '-- : --' }}
              </div>
            </div>

            <!-- 服务器转发 -->
            <div class="bento-card info-card">
              <div class="card-label">外网访问 (Remote)</div>
              <div class="info-value remote-text">{{ configLoaded ? `${frpConfig.serverAddr}:${frpConfig.remotePort}` :
                '-- : --' }}</div>
            </div>

            <!-- 配置文件状态 -->
            <div class="bento-card info-card">
              <div class="card-label">配置文件状态</div>
              <div :class="['config-status', configLoaded ? 'ready' : 'unready']">
                {{ configLoaded ? '已加载就绪' : '未加载' }}
              </div>
            </div>
          </div>
        </div>

        <!-- 日志页面 -->
        <div v-if="currentTab === 'log'" class="page">
          <h2>运行日志</h2>
          <div class="log-container" ref="logContainerRef">
            <div v-if="logs.length === 0" class="hint">暂无日志输出...</div>
            <div v-for="(line, index) in logs" :key="index" class="log-line">{{ line }}</div>
          </div>
        </div>

        <!-- 配置页面 -->
        <div v-if="currentTab === 'config'" class="page">
          <h2>FRP 参数设置</h2>
          <p v-if="isRunning" class="warning-text">服务运行中，禁止修改配置参数。</p>
          <div v-if="configLoaded" :class="{ 'disabled-overlay': isRunning }">
            <div class="form-group">
              <label>服务器地址 (serverAddr)</label>
              <input type="text" v-model="frpConfig.serverAddr" :disabled="isRunning" />
            </div>
            <div class="form-group">
              <label>服务器端口 (serverPort)</label>
              <input type="number" v-model="frpConfig.serverPort" :disabled="isRunning" />
            </div>
            <div class="form-group">
              <label>验证密钥 (auth.token)</label>
              <input type="text" v-model="frpConfig.token" :disabled="isRunning" />
            </div>

            <div class="divider">代理设置 (TCP)</div>
            <div class="form-group">
              <label>本地 IP (localIP)</label>
              <input type="text" v-model="frpConfig.localIP" :disabled="isRunning" />
            </div>
            <div class="form-group">
              <label>本地端口 (localPort)</label>
              <input type="number" v-model="frpConfig.localPort" :disabled="isRunning" />
            </div>
            <div class="form-group">
              <label>远程端口 (remotePort)</label>
              <input type="number" v-model="frpConfig.remotePort" :disabled="isRunning" />
            </div>
            <button class="btn primary" @click="saveConfig" :disabled="isRunning">
              保存并覆盖原始配置文件
            </button>
          </div>
          <div v-else>
            <p class="warning-text">尚未加载配置文件，请前往“软件设置”选择您的 frpc.toml。</p>
          </div>
        </div>

        <!-- 软件设置页面 -->
        <div v-if="currentTab === 'software'" class="page">
          <h2>软件设置</h2>
          <p v-if="isRunning" class="warning-text">服务运行中，禁止修改路径设置。</p>
          <div class="form-group flex-row">
            <div class="input-container">
              <label>frpc 可执行文件路径 (frpc.exe)</label>
              <input type="text" v-model="appSettings.frpcPath" readonly placeholder="点击右侧按钮选择..."
                :disabled="isRunning" />
            </div>
            <button class="btn secondary align-bottom" @click="selectExe" :disabled="isRunning">浏览</button>
          </div>

          <div class="form-group flex-row">
            <div class="input-container">
              <label>配置文件路径 (frpc.toml)</label>
              <input type="text" v-model="appSettings.configPath" readonly placeholder="点击右侧按钮选择..."
                :disabled="isRunning" />
            </div>
            <button class="btn secondary align-bottom" @click="selectToml" :disabled="isRunning">浏览并加载</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted, watch, nextTick } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { open } from '@tauri-apps/plugin-dialog';

const appWindow = getCurrentWindow();

// 状态管理
const currentTab = ref('software'); // 默认先引导用户去设置
const isRunning = ref(false);
const configLoaded = ref(false);
const logs = ref([]);
const logContainerRef = ref(null);
let unlistenLog = null;
const uptimeStr = ref('00:00:00');

// 统一处理日志追加与滚动
const addLog = (message) => {
  logs.value.push(message);
  nextTick(() => {
    if (logContainerRef.value) {
      logContainerRef.value.scrollTop = logContainerRef.value.scrollHeight;
    }
  });
};

// 初始化时优先从 localStorage 读取
const appSettings = ref({
  frpcPath: localStorage.getItem('frpcPath') || '',
  configPath: localStorage.getItem('configPath') || ''
});

// 深度监听 settings 变化，一旦改变自动存入 localStorage
watch(appSettings, (newVal) => {
  localStorage.setItem('frpcPath', newVal.frpcPath);
  localStorage.setItem('configPath', newVal.configPath);
}, { deep: true });

// 计算属性：判断是否准备好启动
const isReady = computed(() => {
  return appSettings.value.frpcPath !== '' && appSettings.value.configPath !== '' && configLoaded.value;
});

// 真实的配置数据（初始为空）
const frpConfig = ref({
  serverAddr: '',
  serverPort: null,
  token: '',
  localIP: '',
  localPort: null,
  remotePort: null
});

// 增加 catch 处理，防止失败无提示
const selectExe = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [{ name: 'Executable', extensions: ['exe'] }]
    });
    if (selected) {
      appSettings.value.frpcPath = selected;
    }
  } catch (err) {
    alert("打开对话框失败，请检查权限配置: " + err);
  }
};

const selectToml = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [{ name: 'TOML Config', extensions: ['toml'] }]
    });
    if (selected) {
      appSettings.value.configPath = selected;
      await loadRealConfig(selected);
    }
  } catch (err) {
    alert("打开对话框失败，请检查权限配置: " + err);
  }
};

// 真实解析 TOML (轻量级正则解析，因为只需要处理特定字段)
const loadRealConfig = async (path) => {
  try {
    const content = await invoke('read_config', { path });
    // 简单的正则匹配提取值
    const extractString = (key, text) => {
      const match = text.match(new RegExp(`${key}\\s*=\\s*"([^"]+)"`));
      return match ? match[1] : '';
    };
    const extractNumber = (key, text) => {
      const match = text.match(new RegExp(`${key}\\s*=\\s*(\\d+)`));
      return match ? parseInt(match[1], 10) : null;
    };

    frpConfig.value.serverAddr = extractString('serverAddr', content);
    frpConfig.value.serverPort = extractNumber('serverPort', content);
    frpConfig.value.token = extractString('auth.token', content);
    frpConfig.value.localIP = extractString('localIP', content);
    frpConfig.value.localPort = extractNumber('localPort', content);
    frpConfig.value.remotePort = extractNumber('remotePort', content);

    configLoaded.value = true;
    currentTab.value = 'dashboard'; // 加载成功跳转概览
  } catch (err) {
    alert("读取配置文件失败: " + err);
  }
};

// 生成要保存的 TOML 内容
const generateToml = () => {
  return `serverAddr = "${frpConfig.value.serverAddr}"
serverPort = ${frpConfig.value.serverPort}
auth.token = "${frpConfig.value.token}"

[[proxies]]
name = "test-tcp"
type = "tcp"
localIP = "${frpConfig.value.localIP}"
localPort = ${frpConfig.value.localPort}
remotePort = ${frpConfig.value.remotePort}`;
};

const saveConfig = async (showAlert = true) => {
  try {
    const tomlContent = generateToml();
    await invoke('save_config', {
      path: appSettings.value.configPath,
      content: tomlContent
    });
    // Vue 的 @click 默认会传 Event 对象进来，所以我们只判断如果明确传了 false 就不弹窗
    if (showAlert !== false) {
      alert("配置已成功覆盖原文件！");
    }
  } catch (error) {
    alert('保存失败: ' + error);
  }
};

// 启动与关闭逻辑
const startFRP = async () => {
  if (!isReady.value) return;
  try {
    logs.value = []; // 每次启动前清空旧日志
    addLog('=== [系统提示] 正在准备启动 FRP 服务... ===');

    await saveConfig(false); // 静默保存配置
    await invoke('start_frp', {
      execPath: appSettings.value.frpcPath,
      configPath: appSettings.value.configPath
    });

    isRunning.value = true;
  } catch (error) {
    addLog(`=== [系统错误] FRP 启动失败: ${error} ===`);
    alert(error);
  }
};

const stopFRP = async () => {
  try {
    await invoke('stop_frp');
    isRunning.value = false;
    addLog('=== [系统提示] FRP 服务已手动关闭。 ===');
  } catch (error) {
    addLog(`=== [系统错误] 关闭 FRP 失败: ${error} ===`);
    alert(error);
  }
};

let timer;

onMounted(async () => {
  // 只请求运行时长数据
  timer = setInterval(async () => {
    if (isRunning.value) {
      try {
        const stats = await invoke('get_frp_stats');
        uptimeStr.value = stats.uptime;
      } catch (e) {
        console.error("获取后端状态失败", e);
      }
    } else {
      uptimeStr.value = '00:00:00';
    }
  }, 1000);

  // 监听后端传来的 frpc-log 事件
  unlistenLog = await listen('frpc-log', (event) => {
    logs.value.push(event.payload);
    // 使用 nextTick 确保 DOM 更新后自动滚动到底部
    nextTick(() => {
      if (logContainerRef.value) {
        logContainerRef.value.scrollTop = logContainerRef.value.scrollHeight;
      }
    });
  });

  // 原有的自动加载配置逻辑
  if (appSettings.value.configPath) {
    try {
      await loadRealConfig(appSettings.value.configPath);
    } catch (e) {
      console.warn("自动加载上次的配置文件失败", e);
    }
  }
});

onUnmounted(() => {
  clearInterval(timer);
  if (unlistenLog) unlistenLog();
});

// 窗口控制
const minimizeWindow = () => appWindow.minimize();
const closeWindow = () => appWindow.close();
</script>

<style>
/* 全局样式，修复滚动条 */
body,
html {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  background-color: #f5f5f7;
}

p,
h1,
h2,
h3,
h4,
h5,
h6.span,
b,
em {
  user-select: none !important;
  -webkit-user-drag: none !important;
  color: #2b2b2b !important;
}
</style>

<style scoped>
/* 容器修复了超出的问题 */
.app-container {
  width: 100%;
  height: 100vh;
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
}

/* 顶部拖拽栏 */
.titlebar {
  height: 38px;
  display: flex;
  align-items: center;
  background-color: #fff;
  user-select: none;
  position: relative;
}

/* 确保拖拽区域铺满且层级正确 */
.titlebar[data-tauri-drag-region] {
  cursor: grab;
}

.title-text {
  flex: 1;
  text-align: center;
  font-size: 13px;
  font-weight: 500;
  color: #333;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: grab;
}

/* Windows 风格右侧按钮组 */
.window-controls {
  position: absolute;
  right: 0;
  top: 0;
  height: 100%;
  display: flex;
  z-index: 20;
}

.control {
  width: 46px;
  /* 经典 Windows 按钮宽度 */
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  color: #333;
  transition: background-color 0.2s, color 0.1s;
}

.control:hover {
  background-color: rgba(0, 0, 0, 0.1);
}

.control.close:hover {
  background-color: #e81123;
  /* Windows 默认的关闭红 */
  color: white;
}

/* 主体布局 */
.app-body {
  display: flex;
  flex: 1;
  height: calc(100vh - 38px);
  overflow: hidden;
  /* 防止溢出 */
}

.sidebar {
  width: 130px;
  background-color: #fff;
  padding: 20px 10px;
  box-sizing: border-box;
  flex-shrink: 0;
}

.nav-item {
  display: flex;
  /* 开启 Flex 布局 */
  align-items: center;
  /* 让图标和文字垂直居中对齐 */
  gap: 10px;
  /* 设置图标和文字之间的间距 */
  padding: 10px 12px;
  /* 稍微调整了一下内边距适配窄边栏 */
  margin-bottom: 8px;
  border-radius: 6px;
  color: #444;
  font-size: 13px;
  /* 字体稍微缩小一点，让侧边栏看起来更精致 */
  cursor: pointer;
  transition: 0.2s;
  user-select: none;
  /* 防止双击时选中文字 */
}

.nav-item:hover {
  background-color: #e5e5ea;
}

.nav-item.active {
  background-color: #2b2b2b;
  color: white;
}

.main-content {
  flex: 1;
  padding: 20px 30px;
  overflow-y: auto;
  /* 仅主内容区允许纵向滚动 */
  box-sizing: border-box;
  background-color: #ffffff;
}

h2 {
  margin-top: 0;
  font-size: 20px;
  color: #1d1d1f;
  margin-bottom: 20px;
}

/* 卡片 */
.status-card {
  background: #f9f9fb;
  border: 1px solid #e5e5ea;
  border-radius: 10px;
  padding: 16px 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  margin-right: 8px;
  display: inline-block;
}

.dot.running {
  background: #34c759;
}

.dot.stopped {
  background: #ff3b30;
}

.btn {
  border: none;
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  transition: 0.2s;
  font-weight: 500;
}

.btn.primary {
  background: #2b2b2b;
  color: white;
}

.btn.primary:disabled {
  background: #e9ecee;
  cursor: not-allowed;
}

.btn.secondary {
  background: #e5e5ea;
  color: #333;
}

.btn.secondary:hover {
  background: #d1d1d6;
}

.btn.danger {
  background: #ff3b30;
  color: white;
}

.form-group {
  margin-bottom: 16px;
}

.flex-row {
  display: flex;
  gap: 10px;
  align-items: flex-end;
}

.input-container {
  flex: 1;
}

.form-group label {
  display: block;
  font-size: 13px;
  color: #666;
  margin-bottom: 6px;
}

.form-group input {
  width: 100%;
  padding: 8px 12px;
  border-radius: 6px;
  border: 1px solid #c7c7cc;
  background: #fff;
  box-sizing: border-box;
  font-size: 14px;
  outline: none;
}

.form-group input:focus {
  border-color: #2b2b2b;
}

.form-group input[readonly] {
  background: #f0f0f3;
  color: #888;
}

.divider {
  margin: 20px 0 10px;
  font-size: 14px;
  font-weight: bold;
  color: #333;
  border-bottom: 1px solid #e5e5ea;
  padding-bottom: 5px;
}

.warning-text {
  color: #ff9500;
  font-size: 13px;
  margin-bottom: 10px;
}

.hint {
  font-size: 12px;
  color: #888;
  margin-top: 20px;
}

/* 新增：终端日志页面样式 */
.log-container {
  background: #1e1e1e;
  color: #cfcfcf;
  padding: 15px;
  border-radius: 8px;
  height: calc(100vh - 160px);
  overflow-y: auto;
  font-family: Consolas, "Courier New", monospace;
  font-size: 13px;
  line-height: 1.5;
  box-shadow: inset 0 0 10px rgba(0, 0, 0, 0.5);
}

.log-line {
  word-wrap: break-word;
  white-space: pre-wrap;
}

.log-container .hint {
  color: #666;
  text-align: center;
  margin-top: 50px;
}

/* 新增：表单元素禁用状态样式 */
.form-group input:disabled {
  background-color: #f0f0f3;
  color: #a0a0a5;
  border-color: #e5e5ea;
  cursor: not-allowed;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed !important;
  pointer-events: none;
}

/* 如果需要整块变灰，可以使用这个类 */
.disabled-overlay {
  opacity: 0.7;
  pointer-events: none;
  /* 彻底阻止所有子元素的鼠标事件 */
}

/* --- 现代化 Bento Box 宫格布局 --- */
.dashboard-page {
  padding-right: 15px;
}

.page-title h2 {
  font-size: 24px;
  font-weight: 600;
  color: #1d1d1f;
  margin-bottom: 24px;
  margin-top: 0;
}

.bento-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  /* 变更为3列网格 */
  gap: 16px;
  /* 舒适的间距 */
}

/* 基础卡片样式 */
.bento-card {
  background: #f5f5f5;
  border: 1px solid rgba(0, 0, 0, 0.05);
  box-shadow: 0 2px 5px #20202010;
  border-radius: 20px;
  padding: 24px;
  display: flex;
  flex-direction: column;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

/* 核心开关卡片跨2列 */
.control-card {
  grid-column: span 2;
  justify-content: space-between;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 20px;
}

.card-title-main {
  font-size: 20px;
  font-weight: 600;
  color: #111;
}

/* 现代化的状态徽章 (代替大背景) */
.status-badge {
  display: inline-flex;
  align-items: center;
  padding: 6px 12px;
  border-radius: 30px;
  font-size: 13px;
  font-weight: 600;
}

.status-badge.running {
  background: #e1fdeb;
  color: #16a34a;
}

.status-badge.stopped {
  background: #fef2f2;
  color: #dc2626;
}

.status-badge .dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  margin-right: 6px;
  background: currentColor;
  /* 跟随文字颜色 */
}

.subtitle {
  font-size: 14px;
  color: #888;
  margin-bottom: 24px;
}

.action-area{
  width: 100%;
  display: flex;
  flex-direction: row;
  justify-content: right;
}

/* 按钮现代化 */
.modern-btn {
  font-size: 15px !important;
  padding: 10px 24px !important;
  border-radius: 10px !important;
  font-weight: 500 !important;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

.modern-btn.primary {
  background: #111;
  color: #fff;
}

.modern-btn.danger {
  background: #ff3b30;
  color: #fff;
}

.uptime-value {
  font-family: ui-monospace, SFMono-Regular, Consolas, monospace;
  font-size: 26px;
  font-weight: 600;
  color: #28c76f;
  margin-top: 20px;
}

/* 信息数据排版 */
.card-label {
  font-size: 13px;
  color: #888;
  margin-bottom: auto;
  /* 让数据推到底部 */
}

.info-value {
  font-family: ui-monospace, SFMono-Regular, Consolas, monospace;
  font-size: 17px;
  font-weight: 600;
  color: #111;
  margin-top: 16px;
}

.remote-text {
  color: #007aff;
  /* 给外网地址一点强调色 */
}

.config-status {
  font-size: 16px;
  font-weight: 600;
  margin-top: 16px;
}

.config-status.ready {
  color: #16a34a;
}

.config-status.unready {
  color: #dc2626;
}
</style>