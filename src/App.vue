<template>
  <div class="app-container">
    <!-- 顶部标题栏：去掉了父容器的 data-tauri-drag-region -->
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
          概览
        </div>
        <div class="nav-item" :class="{ active: currentTab === 'log' }" @click="currentTab = 'log'">运行日志</div>
        <div class="nav-item" :class="{ active: currentTab === 'config' }" @click="currentTab = 'config'">
          参数配置
        </div>
        <div class="nav-item" :class="{ active: currentTab === 'software' }" @click="currentTab = 'software'">
          软件设置
        </div>
      </div>

      <!-- 右侧主内容区 -->
      <div class="main-content">
        <!-- 概览页面 -->
        <div v-if="currentTab === 'dashboard'" class="page">
          <h2>控制台概览</h2>
          <div class="status-card">
            <div class="status-indicator">
              <div :class="['dot', isRunning ? 'running' : 'stopped']"></div>
              <span>{{ isRunning ? '服务运行中' : '服务已停止' }}</span>
            </div>
            <div class="actions">
              <button class="btn primary" v-if="!isRunning" :disabled="!isReady" @click="startFRP">启动 FRP</button>
              <button class="btn danger" v-else @click="stopFRP">关闭 FRP</button>
            </div>
          </div>
          <p v-if="!isReady" class="warning-text">请先在“软件设置”中配置正确的 frpc 路径并加载配置！</p>
          <div class="chart-container">
            <v-chart class="chart" :option="chartOption" autoresize />
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
import { use } from 'echarts/core';
import { CanvasRenderer } from 'echarts/renderers';
import { LineChart } from 'echarts/charts';
import { GridComponent, TooltipComponent } from 'echarts/components';
import VChart from 'vue-echarts';

use([CanvasRenderer, LineChart, GridComponent, TooltipComponent]);

const appWindow = getCurrentWindow();

// 状态管理
const currentTab = ref('software'); // 默认先引导用户去设置
const isRunning = ref(false);
const configLoaded = ref(false);
const logs = ref([]);
const logContainerRef = ref(null);
let unlistenLog = null;

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

// Echarts 逻辑
const chartData = ref(Array.from({ length: 20 }, () => 0));
const chartOption = ref({
  tooltip: { trigger: 'axis' },
  grid: { left: '3%', right: '4%', bottom: '3%', containLabel: true },
  xAxis: { type: 'category', boundaryGap: false, show: false, data: Array.from({ length: 20 }, (_, i) => i) },
  yAxis: { type: 'value', splitLine: { lineStyle: { color: 'rgba(0,0,0,0.05)' } } },
  series: [{
    name: '模拟流量', type: 'line', smooth: true, symbol: 'none',
    lineStyle: { width: 3, color: '#3b82f6' },
    areaStyle: {
      color: { type: 'linear', x: 0, y: 0, x2: 0, y2: 1, colorStops: [{ offset: 0, color: 'rgba(59,130,246,0.4)' }, { offset: 1, color: 'rgba(59,130,246,0)' }] }
    },
    data: chartData.value
  }]
});
let timer;

onMounted(async () => {
  // 原有的 Echarts 定时器逻辑
  timer = setInterval(() => {
    chartData.value.push(isRunning.value ? Math.floor(Math.random() * 50) + 10 : 0);
    chartData.value.shift();
    chartOption.value.series[0].data = [...chartData.value];
  }, 1000);

  // 新增：监听后端传来的 frpc-log 事件
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
  /* 禁止全局出现滚动条 */
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  background-color: #f5f5f7;
  /* 不透明的苹果灰底色 */
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
  background-color: #e5e5ea;
  border-bottom: 1px solid #d1d1d6;
  user-select: none;
  position: relative;
  /* 为右侧按钮绝对定位做准备 */
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
  width: 180px;
  background-color: #f0f0f3;
  border-right: 1px solid #d1d1d6;
  padding: 20px 10px;
  box-sizing: border-box;
}

.nav-item {
  padding: 10px 14px;
  margin-bottom: 8px;
  border-radius: 6px;
  color: #444;
  font-size: 14px;
  cursor: pointer;
  transition: 0.2s;
}

.nav-item:hover {
  background-color: #e5e5ea;
}

.nav-item.active {
  background-color: #007aff;
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
  background: #007aff;
  color: white;
}

.btn.primary:disabled {
  background: #a1c6ea;
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

.chart-container {
  height: 220px;
  background: #f9f9fb;
  border: 1px solid #e5e5ea;
  border-radius: 10px;
  padding: 10px;
}

.chart {
  width: 100%;
  height: 100%;
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
  border-color: #007aff;
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
</style>