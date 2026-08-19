<template>
  <!-- 最外层毛玻璃容器 -->
  <div class="macos-window">

    <!-- 顶部拖拽栏 (Tauri 专属 data-tauri-drag-region 属性) -->
    <div class="titlebar" data-tauri-drag-region>
      <div class="window-controls">
        <div class="control close" @click="closeWindow"></div>
        <div class="control minimize" @click="minimizeWindow"></div>
        <div class="control maximize"></div>
      </div>
      <div class="title-text" data-tauri-drag-region>FRP Manager</div>
    </div>

    <div class="app-body">
      <!-- 左侧边栏 -->
      <div class="sidebar">
        <div class="nav-item" :class="{ active: currentTab === 'dashboard' }" @click="currentTab = 'dashboard'">
          <!-- Dashboard SVG -->
          <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="7" height="9" rx="1" />
            <rect x="14" y="3" width="7" height="5" rx="1" />
            <rect x="14" y="12" width="7" height="9" rx="1" />
            <rect x="3" y="16" width="7" height="5" rx="1" />
          </svg>
          概览
        </div>
        <div class="nav-item" :class="{ active: currentTab === 'config' }" @click="currentTab = 'config'">
          <!-- Settings SVG -->
          <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="3" />
            <path
              d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
          </svg>
          参数配置
        </div>
        <div class="nav-item" :class="{ active: currentTab === 'software' }" @click="currentTab = 'software'">
          <!-- Tool SVG -->
          <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2">
            <path
              d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z" />
          </svg>
          软件设置
        </div>
      </div>

      <!-- 右侧主内容区 -->
      <div class="main-content">
        <!-- 页面: 概览 -->
        <div v-if="currentTab === 'dashboard'" class="page">
          <h2>控制台概览</h2>
          <div class="status-card">
            <div class="status-indicator">
              <div :class="['dot', isRunning ? 'running' : 'stopped']"></div>
              <span>{{ isRunning ? '服务运行中' : '服务已停止' }}</span>
            </div>
            <div class="actions">
              <button class="btn primary" v-if="!isRunning" @click="startFRP">启动 FRP</button>
              <button class="btn danger" v-else @click="stopFRP">关闭 FRP</button>
            </div>
          </div>

          <div class="chart-container">
            <v-chart class="chart" :option="chartOption" autoresize />
          </div>
        </div>

        <!-- 页面: FRP 配置 -->
        <div v-if="currentTab === 'config'" class="page">
          <h2>FRP 参数设置</h2>
          <div class="form-group">
            <label>服务器地址 (serverAddr)</label>
            <input type="text" v-model="frpConfig.serverAddr" placeholder="如: 8.137.173.82" />
          </div>
          <div class="form-group">
            <label>服务器端口 (serverPort)</label>
            <input type="number" v-model="frpConfig.serverPort" placeholder="默认: 7000" />
          </div>
          <div class="form-group">
            <label>验证密钥 (auth.token)</label>
            <input type="text" v-model="frpConfig.token" placeholder="123456a" />
          </div>

          <div class="divider">代理设置 (TCP)</div>
          <div class="form-group">
            <label>本地 IP (localIP)</label>
            <input type="text" v-model="frpConfig.localIP" placeholder="127.0.0.1" />
          </div>
          <div class="form-group">
            <label>本地端口 (localPort)</label>
            <input type="number" v-model="frpConfig.localPort" placeholder="25565" />
          </div>
          <div class="form-group">
            <label>远程端口 (remotePort)</label>
            <input type="number" v-model="frpConfig.remotePort" placeholder="25565" />
          </div>
          <button class="btn primary" @click="saveConfig">保存配置 (frpc.toml)</button>
        </div>

        <!-- 页面: 软件设置 -->
        <div v-if="currentTab === 'software'" class="page">
          <h2>软件设置</h2>
          <div class="form-group">
            <label>frpc 可执行文件路径</label>
            <input type="text" v-model="appSettings.frpcPath" placeholder="./frpc.exe" />
          </div>
          <div class="form-group">
            <label>配置文件路径</label>
            <input type="text" v-model="appSettings.configPath" placeholder="./frpc.toml" />
          </div>
          <p class="hint">默认假定 frpc.exe 和 frpc.toml 与本程序在同一目录下。</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';
import { use } from 'echarts/core';
import { CanvasRenderer } from 'echarts/renderers';
import { LineChart } from 'echarts/charts';
import { GridComponent, TooltipComponent } from 'echarts/components';
import VChart from 'vue-echarts';

// 注册 Echarts 组件
use([CanvasRenderer, LineChart, GridComponent, TooltipComponent]);

// 状态管理
const currentTab = ref('dashboard');
const isRunning = ref(false);
const chartData = ref(Array.from({ length: 20 }, () => 0));

// 对应来源 的配置数据结构
const frpConfig = ref({
  serverAddr: '8.137.173.82',
  serverPort: 7000,
  token: '123456a',
  localIP: '127.0.0.1',
  localPort: 25565,
  remotePort: 25565
});

const appSettings = ref({
  frpcPath: 'frpc.exe',
  configPath: 'frpc.toml'
});

// Echarts 配置
const chartOption = ref({
  tooltip: { trigger: 'axis' },
  grid: { left: '3%', right: '4%', bottom: '3%', containLabel: true },
  xAxis: { type: 'category', boundaryGap: false, show: false, data: Array.from({ length: 20 }, (_, i) => i) },
  yAxis: { type: 'value', splitLine: { lineStyle: { color: 'rgba(0,0,0,0.05)' } } },
  series: [{
    name: '网络流量 (模拟)',
    type: 'line',
    smooth: true,
    symbol: 'none',
    lineStyle: { width: 3, color: '#3b82f6' },
    areaStyle: {
      color: {
        type: 'linear', x: 0, y: 0, x2: 0, y2: 1,
        colorStops: [{ offset: 0, color: 'rgba(59, 130, 246, 0.4)' }, { offset: 1, color: 'rgba(59, 130, 246, 0)' }]
      }
    },
    data: chartData.value
  }]
});

// 模拟流量动画
let timer;
onMounted(() => {
  timer = setInterval(() => {
    if (isRunning.value) {
      chartData.value.push(Math.floor(Math.random() * 50) + 10);
    } else {
      chartData.value.push(0);
    }
    chartData.value.shift();
    chartOption.value.series[0].data = [...chartData.value];
  }, 1000);
});
onUnmounted(() => clearInterval(timer));

// 窗口控制
const minimizeWindow = () => appWindow.minimize();
const closeWindow = () => appWindow.close();

// 核心逻辑: 启动与关闭
const startFRP = async () => {
  try {
    await saveConfig(); // 启动前先保存最新配置
    await invoke('start_frp', {
      execPath: appSettings.value.frpcPath,
      configPath: appSettings.value.configPath
    });
    isRunning.value = true;
  } catch (error) {
    alert(error);
  }
};

const stopFRP = async () => {
  try {
    await invoke('stop_frp');
    isRunning.value = false;
  } catch (error) {
    alert(error);
  }
};

// 生成 TOML 格式字符串[cite: 1]
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

const saveConfig = async () => {
  try {
    const tomlContent = generateToml();
    await invoke('save_config', {
      path: appSettings.value.configPath,
      content: tomlContent
    });
  } catch (error) {
    alert('保存配置失败: ' + error);
  }
};
</script>

<style scoped>
body,
html {
  margin: 0;
  padding: 0;
  background-color: transparent !important;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
}

/* 核心: MacOS 毛玻璃质感 */
.macos-window {
  width: 100vw;
  height: 100vh;
  background: rgba(245, 245, 247, 0.6);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-radius: 12px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
  /* 柔和的边框，代替突兀的高对比阴影 */
  border: 1px solid rgba(255, 255, 255, 0.4);
}

/* 自定义标题栏 */
.titlebar {
  height: 38px;
  display: flex;
  align-items: center;
  padding: 0 16px;
  background: rgba(255, 255, 255, 0.3);
  border-bottom: 1px solid rgba(0, 0, 0, 0.05);
}

.window-controls {
  display: flex;
  gap: 8px;
  z-index: 10;
}

.control {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  cursor: pointer;
}

.control.close {
  background: #ff5f56;
}

.control.minimize {
  background: #ffbd2e;
}

.control.maximize {
  background: #27c93f;
}

.title-text {
  flex: 1;
  text-align: center;
  font-size: 13px;
  font-weight: 500;
  color: #333;
  margin-left: -50px;
  /* 修正居中偏移 */
}

/* 布局 */
.app-body {
  display: flex;
  flex: 1;
  height: calc(100vh - 38px);
}

/* 左侧边栏 */
.sidebar {
  width: 200px;
  background: rgba(255, 255, 255, 0.4);
  border-right: 1px solid rgba(0, 0, 0, 0.05);
  padding: 20px 10px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
  margin-bottom: 8px;
  border-radius: 8px;
  color: #555;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.nav-item:hover {
  background: rgba(0, 0, 0, 0.04);
}

.nav-item.active {
  background: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
  font-weight: 500;
}

/* 主内容区 */
.main-content {
  flex: 1;
  padding: 30px;
  overflow-y: auto;
}

h2 {
  margin-top: 0;
  font-size: 20px;
  color: #1d1d1f;
  margin-bottom: 20px;
}

/* 卡片与表单 */
.status-card {
  background: rgba(255, 255, 255, 0.6);
  border-radius: 12px;
  padding: 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.02);
  margin-bottom: 20px;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 10px;
  font-weight: 500;
  color: #333;
}

.dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
}

.dot.running {
  background: #34c759;
  box-shadow: 0 0 8px rgba(52, 199, 89, 0.6);
}

.dot.stopped {
  background: #ff3b30;
}

.btn {
  border: none;
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
  transition: 0.2s;
  font-weight: 500;
}

.btn.primary {
  background: #007aff;
  color: white;
}

.btn.primary:hover {
  background: #006ee6;
}

.btn.danger {
  background: rgba(255, 59, 48, 0.1);
  color: #ff3b30;
}

.btn.danger:hover {
  background: rgba(255, 59, 48, 0.2);
}

.chart-container {
  height: 250px;
  background: rgba(255, 255, 255, 0.6);
  border-radius: 12px;
  padding: 10px;
}

.chart {
  width: 100%;
  height: 100%;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  font-size: 13px;
  color: #666;
  margin-bottom: 6px;
}

.form-group input {
  width: 100%;
  padding: 10px;
  border-radius: 8px;
  border: 1px solid rgba(0, 0, 0, 0.1);
  background: rgba(255, 255, 255, 0.7);
  box-sizing: border-box;
  font-size: 14px;
  outline: none;
  transition: 0.2s;
}

.form-group input:focus {
  border-color: #007aff;
  background: white;
}

.divider {
  margin: 20px 0 10px;
  font-size: 14px;
  font-weight: 500;
  color: #333;
  border-bottom: 1px solid rgba(0, 0, 0, 0.05);
  padding-bottom: 5px;
}

.hint {
  font-size: 12px;
  color: #888;
}
</style>