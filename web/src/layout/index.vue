<template>
  <div
    class="layout"
    :class="{
      'has-announcement': showAnnouncementBar && currentAnnouncement,
      'is-mobile': isMobile,
      'sidebar-collapsed': isCollapsed,
      'drawer-open': mobileDrawerOpen
    }"
  >
    <!-- 侧边栏 -->
    <div class="sidebar" :class="{ collapsed: isCollapsed }">
      <div class="sidebar-header">
        <div class="logo">
          <svg class="logo-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M3 13h2v-2H3v2zm0 4h2v-2H3v2zm0-8h2V7H3v2zm4 4h14v-2H7v2zm0 4h14v-2H7v2zM7 7v2h14V7H7z" fill="currentColor"/>
          </svg>
          <span v-if="!isCollapsed" class="logo-text">QAExchange</span>
        </div>
        <div v-if="!isCollapsed" class="logo-subtitle">量化交易系统</div>
      </div>

      <!-- 导航菜单 -->
      <el-menu
        :default-active="activeMenu"
        :collapse="isCollapsed"
        :collapse-transition="false"
        background-color="transparent"
        text-color="#8b949e"
        active-text-color="#ffffff"
        @select="handleMenuSelect"
      >
        <!-- 系统总览 -->
        <el-menu-item index="/dashboard">
          <i class="el-icon-data-board"></i>
          <span slot="title">系统总览</span>
        </el-menu-item>

        <!-- 交易中心 -->
        <div class="menu-group-title" v-if="!isCollapsed">交易中心</div>
        <el-menu-item index="/trade">
          <i class="el-icon-sell"></i>
          <span slot="title">交易面板</span>
        </el-menu-item>
        <el-menu-item index="/chart">
          <!-- ⚠️ 原来是 el-icon-trend-charts —— Element UI 2.15 **没有这个图标**
               (element-ui/lib/theme-chalk/index.css 里 0 命中),
               于是 <i> 渲染成空元素,侧边栏该项没有图标。
               合法的近义图标:el-icon-data-line / data-analysis / data-board。
               K线用 data-line(折线)最贴切。@yutiansut @quantaxis -->
          <i class="el-icon-data-line"></i>
          <span slot="title">K线图表</span>
        </el-menu-item>
        <el-menu-item index="/orders">
          <i class="el-icon-document"></i>
          <span slot="title">订单管理</span>
        </el-menu-item>
        <el-menu-item index="/positions">
          <i class="el-icon-coin"></i>
          <span slot="title">持仓管理</span>
        </el-menu-item>
        <el-menu-item index="/trades">
          <i class="el-icon-finished"></i>
          <span slot="title">成交记录</span>
        </el-menu-item>
        <!-- Phase 11: 高级交易功能 @yutiansut @quantaxis -->
        <el-menu-item index="/transfer">
          <i class="el-icon-refresh"></i>
          <span slot="title">银期转账</span>
        </el-menu-item>
        <el-menu-item index="/conditional-orders">
          <i class="el-icon-aim"></i>
          <span slot="title">条件单</span>
        </el-menu-item>
        <el-menu-item index="/batch-orders">
          <i class="el-icon-copy-document"></i>
          <span slot="title">批量下单</span>
        </el-menu-item>

        <!-- 账户管理 -->
        <div class="menu-group-title" v-if="!isCollapsed">账户管理</div>
        <el-menu-item index="/accounts">
          <i class="el-icon-wallet"></i>
          <span slot="title">账户列表</span>
        </el-menu-item>
        <el-menu-item index="/my-accounts">
          <i class="el-icon-user"></i>
          <span slot="title">我的账户</span>
        </el-menu-item>
        <el-menu-item index="/account-curve">
          <i class="el-icon-data-line"></i>
          <span slot="title">资金曲线</span>
        </el-menu-item>
        <!-- Phase 12: 用户功能 @yutiansut @quantaxis -->
        <el-menu-item index="/password">
          <i class="el-icon-key"></i>
          <span slot="title">密码管理</span>
        </el-menu-item>
        <el-menu-item index="/commission">
          <i class="el-icon-money"></i>
          <span slot="title">手续费查询</span>
        </el-menu-item>
        <el-menu-item index="/margin">
          <i class="el-icon-s-finance"></i>
          <span slot="title">保证金查询</span>
        </el-menu-item>
        <!-- ✨ 系统公告入口 @yutiansut @quantaxis -->
        <el-menu-item index="/announcements">
          <i class="el-icon-bell"></i>
          <span slot="title">系统公告</span>
        </el-menu-item>

        <!-- 市场监控 -->
        <div class="menu-group-title" v-if="!isCollapsed">市场监控</div>
        <el-menu-item index="/market-overview">
          <i class="el-icon-view"></i>
          <span slot="title">市场总览</span>
        </el-menu-item>
        <el-menu-item index="/monitoring">
          <i class="el-icon-odometer"></i>
          <span slot="title">系统监控</span>
        </el-menu-item>

        <!-- 管理中心 -->
        <template v-if="isAdmin">
          <div class="menu-group-title" v-if="!isCollapsed">
            <span>管理中心</span>
            <el-tag type="danger" size="mini">Admin</el-tag>
          </div>
          <el-menu-item index="/admin-instruments">
            <i class="el-icon-tickets"></i>
            <span slot="title">合约管理</span>
          </el-menu-item>
          <el-menu-item index="/admin-risk">
            <i class="el-icon-warning-outline"></i>
            <span slot="title">风控监控</span>
          </el-menu-item>
          <el-menu-item index="/admin-settlement">
            <i class="el-icon-notebook-2"></i>
            <span slot="title">结算管理</span>
          </el-menu-item>
          <el-menu-item index="/admin-accounts">
            <i class="el-icon-user"></i>
            <span slot="title">账户管理</span>
          </el-menu-item>
          <el-menu-item index="/admin-transactions">
            <i class="el-icon-bank-card"></i>
            <span slot="title">资金流水</span>
          </el-menu-item>
          <!-- Phase 13: 管理端功能 @yutiansut @quantaxis -->
          <el-menu-item index="/admin-account-freeze">
            <i class="el-icon-lock"></i>
            <span slot="title">账户状态管理</span>
          </el-menu-item>
          <el-menu-item index="/admin-audit-logs">
            <i class="el-icon-document-checked"></i>
            <span slot="title">审计日志</span>
          </el-menu-item>
          <el-menu-item index="/admin-announcements">
            <i class="el-icon-bell"></i>
            <span slot="title">系统公告</span>
          </el-menu-item>
        </template>
      </el-menu>

      <!-- 折叠按钮 -->
      <div class="sidebar-footer">
        <div class="collapse-btn" @click="toggleCollapse">
          <i :class="isCollapsed ? 'el-icon-s-unfold' : 'el-icon-s-fold'"></i>
        </div>
      </div>
    </div>

    <!-- ✨ 移动端抽屉遮罩: 点击关闭侧边栏 @yutiansut @quantaxis -->
    <div
      v-if="isMobile && mobileDrawerOpen"
      class="sidebar-overlay"
      @click="closeMobileDrawer"
    ></div>

    <!-- 右侧区域 -->
    <div class="main-container">
      <!-- ✨ 公告通知栏 @yutiansut @quantaxis -->
      <transition name="slide-down">
        <div
          v-if="showAnnouncementBar && currentAnnouncement"
          class="announcement-bar"
          :class="'announcement-' + announcementAlertType"
        >
          <div class="announcement-content" @click="goToAnnouncements">
            <i class="el-icon-bell announcement-icon"></i>
            <span class="announcement-label">
              <el-tag size="mini" :type="announcementAlertType === 'error' ? 'danger' : announcementAlertType">
                {{ currentAnnouncement.announcement_type || '公告' }}
              </el-tag>
            </span>
            <span class="announcement-title">{{ currentAnnouncement.title }}</span>
            <span v-if="announcements.length > 1" class="announcement-indicator">
              {{ announcementIndex + 1 }}/{{ announcements.length }}
            </span>
          </div>
          <div class="announcement-close" @click.stop="closeAnnouncementBar">
            <i class="el-icon-close"></i>
          </div>
        </div>
      </transition>

      <!-- 顶部栏 -->
      <div class="top-header">
        <div class="header-left">
          <!-- ✨ 移动端菜单按钮: 抽屉收起后, 侧边栏里的折叠按钮不可达 @yutiansut @quantaxis -->
          <div v-if="isMobile" class="mobile-menu-btn" @click="toggleCollapse">
            <i :class="mobileDrawerOpen ? 'el-icon-close' : 'el-icon-s-fold'"></i>
          </div>
          <div class="page-title">{{ pageTitle }}</div>
        </div>
        <div class="header-right">
          <!-- 系统状态指示 -->
          <div class="status-indicator">
            <span class="status-dot online"></span>
            <span class="status-text">系统正常</span>
          </div>

          <!-- 用户信息 -->
          <el-dropdown @command="handleUserCommand" trigger="click">
            <div class="user-info">
              <el-avatar :size="36" class="user-avatar">
                {{ avatarText }}
              </el-avatar>
              <div class="user-details" v-if="!isMobile">
                <div class="user-name">{{ displayName }}</div>
                <div class="user-role">
                  <el-tag v-if="isAdmin" type="danger" size="mini">管理员</el-tag>
                  <el-tag v-else type="info" size="mini">普通用户</el-tag>
                </div>
              </div>
              <i class="el-icon-caret-bottom"></i>
            </div>
            <el-dropdown-menu slot="dropdown" class="user-dropdown-menu">
              <el-dropdown-item disabled>
                <div class="dropdown-user-info">
                  <i class="el-icon-user"></i>
                  <span>用户ID: {{ currentUser }}</span>
                </div>
              </el-dropdown-item>
              <el-dropdown-item divided command="logout">
                <i class="el-icon-switch-button"></i>
                <span>退出登录</span>
              </el-dropdown-item>
            </el-dropdown-menu>
          </el-dropdown>
        </div>
      </div>

      <!-- 内容区域 -->
      <div class="content-wrapper">
        <transition name="fade" mode="out-in">
          <router-view :key="$route.fullPath" />
        </transition>
      </div>
    </div>
  </div>
</template>

<script>
// @yutiansut @quantaxis - 专业量化交易系统布局
import { mapGetters } from 'vuex'
import { queryAnnouncements } from '@/api'

export default {
  name: 'Layout',
  data() {
    return {
      isCollapsed: false,
      isMobile: false,
      // ✨ 记住用户在宽屏下的折叠意愿, 从窄屏放大回来时按它恢复 @yutiansut @quantaxis
      userCollapsed: false,
      // ✨ 窄屏抽屉是否展开(抽屉浮在内容之上, 不再挤压内容)
      mobileDrawerOpen: false,
      resizeRaf: null,
      // ✨ 公告系统 @yutiansut @quantaxis
      announcements: [],
      showAnnouncementBar: false,
      announcementIndex: 0,
      announcementTimer: null
    }
  },
  computed: {
    ...mapGetters(['currentUser', 'userInfo', 'isAdmin']),
    activeMenu() {
      return this.$route.path
    },
    displayName() {
      return (this.userInfo && this.userInfo.username) || this.currentUser || '用户'
    },
    avatarText() {
      const name = this.displayName
      return name ? name.charAt(0).toUpperCase() : 'U'
    },
    // ✨ 当前显示的公告 @yutiansut @quantaxis
    currentAnnouncement() {
      if (this.announcements.length === 0) return null
      return this.announcements[this.announcementIndex % this.announcements.length]
    },
    // 公告优先级对应的样式类型
    announcementAlertType() {
      if (!this.currentAnnouncement) return 'info'
      const priorityMap = {
        'Urgent': 'error',
        'High': 'warning',
        'Normal': 'info',
        'Low': 'info'
      }
      return priorityMap[this.currentAnnouncement.priority] || 'info'
    },
    pageTitle() {
      const titles = {
        '/dashboard': '系统总览',
        '/trade': '交易面板',
        '/chart': 'K线图表',
        '/orders': '订单管理',
        '/positions': '持仓管理',
        '/trades': '成交记录',
        // Phase 11: 高级交易功能 @yutiansut @quantaxis
        '/transfer': '银期转账',
        '/conditional-orders': '条件单',
        '/batch-orders': '批量下单',
        // 账户管理
        '/accounts': '账户列表',
        '/my-accounts': '我的账户',
        '/account-curve': '资金曲线',
        // Phase 12: 用户功能 @yutiansut @quantaxis
        '/password': '密码管理',
        '/commission': '手续费查询',
        '/margin': '保证金查询',
        // 市场监控
        '/market-overview': '市场总览',
        '/monitoring': '系统监控',
        // 管理中心
        '/admin-instruments': '合约管理',
        '/admin-risk': '风控监控',
        '/admin-settlement': '结算管理',
        '/admin-accounts': '账户管理',
        '/admin-transactions': '资金流水',
        // Phase 13: 管理端功能 @yutiansut @quantaxis
        '/admin-account-freeze': '账户状态管理',
        '/admin-audit-logs': '审计日志',
        '/admin-announcements': '系统公告',
        '/announcements': '系统公告'  // ✨ 用户端公告页面 @yutiansut @quantaxis
      }
      return titles[this.$route.path] || '量化交易系统'
    }
  },
  mounted() {
    this.checkMobile()
    // ✨ 用 rAF 合并 resize 回调: 原来 checkMobile 直接绑在 resize 上,
    //    拖动窗口时每帧都会触发一次 Vue 重渲染 @yutiansut @quantaxis
    window.addEventListener('resize', this.onWindowResize)
    // ✨ 加载公告 @yutiansut @quantaxis
    this.loadAnnouncements()
  },
  beforeDestroy() {
    window.removeEventListener('resize', this.onWindowResize)
    if (this.resizeRaf) {
      window.cancelAnimationFrame(this.resizeRaf)
      this.resizeRaf = null
    }
    // ✨ 清理公告轮播定时器 @yutiansut @quantaxis
    if (this.announcementTimer) {
      clearInterval(this.announcementTimer)
    }
  },
  methods: {
    handleMenuSelect(index) {
      this.$router.push(index)
      // ✨ 窄屏下选完菜单立刻收起抽屉, 否则遮罩挡住刚跳转的页面 @yutiansut @quantaxis
      if (this.isMobile) {
        this.closeMobileDrawer()
      }
    },
    handleUserCommand(command) {
      if (command === 'logout') {
        this.$confirm('确定要退出登录吗?', '提示', {
          confirmButtonText: '确定',
          cancelButtonText: '取消',
          type: 'warning'
        }).then(() => {
          this.$store.dispatch('logout')
          this.$message.success('已退出登录')
          this.$router.push('/login')
        }).catch(() => {})
      }
    },
    toggleCollapse() {
      if (this.isMobile) {
        // 窄屏: 开/关抽屉。抽屉展开时显示完整菜单文字
        this.mobileDrawerOpen = !this.mobileDrawerOpen
        this.isCollapsed = !this.mobileDrawerOpen
      } else {
        this.isCollapsed = !this.isCollapsed
        // ✨ 记录宽屏下的折叠意愿, 供 checkMobile 恢复用
        this.userCollapsed = this.isCollapsed
      }
      this.notifyLayoutChange()
    },
    closeMobileDrawer() {
      this.mobileDrawerOpen = false
      this.isCollapsed = true
    },
    checkMobile() {
      const wasMobile = this.isMobile
      this.isMobile = window.innerWidth < 768
      if (this.isMobile) {
        this.isCollapsed = true
        if (!wasMobile) {
          this.mobileDrawerOpen = false
        }
      } else if (wasMobile) {
        // ✨ 原来这里没有 else 分支: 窗口一旦被缩窄过, isCollapsed 就永久为 true,
        //    再放大也不会恢复展开。实测(headless Chrome): 700px 宽时 sidebar 64px,
        //    放大到 1600px 后仍然是 64px。 @yutiansut @quantaxis
        this.isCollapsed = this.userCollapsed
        this.mobileDrawerOpen = false
      }
      if (wasMobile !== this.isMobile) {
        this.notifyLayoutChange()
      }
    },
    onWindowResize() {
      if (this.resizeRaf) return
      this.resizeRaf = window.requestAnimationFrame(() => {
        this.resizeRaf = null
        this.checkMobile()
      })
    },
    // ✨ 侧边栏宽度变化不是 window resize, 但对图表而言完全等价。
    //    实测: 1440x900 折叠侧边栏后 .main-container 由 1220px 变为 1376px,
    //    而三个 ECharts canvas 仍然是 546/546/1146 —— 右侧凭空多出 156px 空白。
    //    等 CSS transition(0.25s) 走完后广播一次 resize, 让所有监听 resize 的
    //    图表(见 KLineChart.vue / dashboard / chart 页)重新计算尺寸。 @yutiansut @quantaxis
    notifyLayoutChange() {
      window.setTimeout(() => {
        window.dispatchEvent(new Event('resize'))
      }, 300)
    },
    // ✨ 加载公告列表 @yutiansut @quantaxis
    async loadAnnouncements() {
      try {
        const res = await queryAnnouncements({ active_only: true, page_size: 20 })
        // 过滤有效期内的公告
        const now = Date.now()
        this.announcements = (res.announcements || []).filter(a => {
          const from = a.publish_time ? a.publish_time : 0
          const until = a.expire_time ? a.expire_time : Number.MAX_SAFE_INTEGER
          return now >= from && now <= until
        })
        // 按优先级排序：Urgent > High > Normal > Low
        const priorityOrder = { 'Urgent': 0, 'High': 1, 'Normal': 2, 'Low': 3 }
        this.announcements.sort((a, b) =>
          (priorityOrder[a.priority] || 3) - (priorityOrder[b.priority] || 3)
        )
        if (this.announcements.length > 0) {
          this.showAnnouncementBar = true
          this.startAnnouncementRotation()
        }
      } catch (error) {
        console.error('加载公告失败', error)
      }
    },
    // ✨ 开始公告轮播 @yutiansut @quantaxis
    startAnnouncementRotation() {
      if (this.announcementTimer) {
        clearInterval(this.announcementTimer)
      }
      if (this.announcements.length > 1) {
        this.announcementTimer = setInterval(() => {
          this.announcementIndex = (this.announcementIndex + 1) % this.announcements.length
        }, 8000) // 每8秒切换一条
      }
    },
    // ✨ 关闭公告栏 @yutiansut @quantaxis
    closeAnnouncementBar() {
      this.showAnnouncementBar = false
      if (this.announcementTimer) {
        clearInterval(this.announcementTimer)
      }
    },
    // ✨ 点击公告查看详情 @yutiansut @quantaxis
    goToAnnouncements() {
      this.$router.push('/announcements')
    }
  }
}
</script>

<style lang="scss" scoped>
// @yutiansut @quantaxis - 专业量化交易系统布局样式
$sidebar-width: 220px;
$sidebar-collapsed-width: 64px;
$header-height: 56px;
$announcement-height: 40px;   // ✨ 与 .announcement-bar 的 height 保持一致
$primary-color: #1890ff;
$dark-bg-primary: #0d1117;
$dark-bg-secondary: #161b22;
$dark-bg-tertiary: #21262d;
$dark-border: #30363d;
$dark-text-primary: #f0f6fc;
$dark-text-secondary: #8b949e;

// ✨ 深色主题配色 @yutiansut @quantaxis
.layout {
  display: flex;
  height: 100vh;          // ✨ 定高，为内部 flex 子项提供确定的主尺寸
  overflow: hidden;       // ✨ 外层不滚动，滚动交给 .content-wrapper
  background: $dark-bg-primary;  // 深色主题背景
  // ✨ 全站唯一的布局预算口径 @yutiansut @quantaxis
  // 之前全仓库没有一处把这几个数加对: 散落着 -56px / -60px /
  // -100px / -150px / -220px / -250px 六种互相矛盾的写法, 没有一处等于真实值。
  // 真实几何: top-header 56px + content-wrapper 上下 padding 各 20px = 96px;
  //          公告条出现时再 +40px = 136px。
  // 各页面请一律用 var(--qa-content-h) / var(--qa-content-w),
  // 不要再手写 calc(100vh - N)。
  --qa-header-h: #{$header-height};
  --qa-content-pad: 20px;
  --qa-announcement-h: 0px;
  --qa-sidebar-w: #{$sidebar-width};
  --qa-content-h: calc(
    100vh - var(--qa-header-h) - var(--qa-announcement-h) - var(--qa-content-pad) * 2
  );
  --qa-content-w: calc(100vw - var(--qa-sidebar-w) - var(--qa-content-pad) * 2);

  // ✨ 公告条出现时预算再减 40px, 得到真实的 100vh - 136px @yutiansut @quantaxis
  &.has-announcement {
    --qa-announcement-h: #{$announcement-height};
  }

  // 侧边栏折叠时可用宽度变宽。注意 .collapsed 在后代元素上, CSS 无法向上冒泡,
  // 所以由 Vue 在 .layout 上同步一个 .sidebar-collapsed 类。 @yutiansut @quantaxis
  &.sidebar-collapsed {
    --qa-sidebar-w: #{$sidebar-collapsed-width};
  }
}

// ✨ 移动端抽屉遮罩 @yutiansut @quantaxis
.sidebar-overlay {
  position: fixed;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  background: rgba(0, 0, 0, 0.55);
  z-index: 999;   // 低于 .sidebar 的 1000, 高于内容
}

// ✨ 移动端顶栏菜单按钮 @yutiansut @quantaxis
.mobile-menu-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  margin-right: 10px;
  border-radius: 6px;
  cursor: pointer;
  color: $dark-text-secondary;
  transition: all 0.2s ease;

  i {
    font-size: 20px;
  }

  &:hover {
    background: $dark-bg-tertiary;
    color: $dark-text-primary;
  }
}

// 侧边栏
.sidebar {
  width: $sidebar-width;
  background: $dark-bg-primary;
  display: flex;
  flex-direction: column;
  transition: width 0.25s ease;
  position: fixed;
  left: 0;
  top: 0;
  height: 100vh;
  z-index: 1000;

  &.collapsed {
    width: $sidebar-collapsed-width;

    .sidebar-header {
      padding: 16px 12px;
    }

    .logo {
      justify-content: center;
    }

    .logo-subtitle {
      display: none;
    }

    .menu-group-title {
      display: none;
    }
  }
}

.sidebar-header {
  padding: 20px 16px;
  border-bottom: 1px solid $dark-border;
}

.logo {
  display: flex;
  align-items: center;
  gap: 10px;

  .logo-icon {
    width: 32px;
    height: 32px;
    color: $primary-color;
    flex-shrink: 0;
  }

  .logo-text {
    font-size: 18px;
    font-weight: 700;
    color: $dark-text-primary;
    white-space: nowrap;
  }
}

.logo-subtitle {
  font-size: 12px;
  color: $dark-text-secondary;
  margin-top: 4px;
  padding-left: 42px;
}

// 菜单样式
.menu-group-title {
  padding: 20px 16px 8px;
  font-size: 11px;
  font-weight: 600;
  color: $dark-text-secondary;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  display: flex;
  align-items: center;
  gap: 8px;
}

::v-deep .el-menu {
  border: none;
  flex: 1;
  overflow-y: auto;
  padding: 8px;

  .el-menu-item {
    height: 44px;
    line-height: 44px;
    margin: 2px 0;
    border-radius: 8px;
    transition: all 0.2s ease;

    i {
      color: $dark-text-secondary;
      font-size: 18px;
      margin-right: 12px;
    }

    &:hover {
      background: $dark-bg-tertiary !important;
    }

    &.is-active {
      background: linear-gradient(135deg, $primary-color 0%, #096dd9 100%) !important;
      color: white !important;

      i {
        color: white;
      }
    }
  }

  &.el-menu--collapse {
    .el-menu-item {
      padding: 0 20px;

      i {
        margin-right: 0;
      }
    }
  }
}

.sidebar-footer {
  padding: 12px;
  border-top: 1px solid $dark-border;
}

.collapse-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 40px;
  border-radius: 8px;
  cursor: pointer;
  color: $dark-text-secondary;
  transition: all 0.2s ease;

  i {
    font-size: 20px;
  }

  &:hover {
    background: $dark-bg-tertiary;
    color: $dark-text-primary;
  }
}

// 主内容区
.main-container {
  flex: 1;
  // ✨✨ 小屏「内容显示不全」的头号根因 @yutiansut @quantaxis
  // flex item 的 min-width 默认是 auto (= min-content), 而本容器里装着
  // el-table-column 固定宽度合计 1110~3960px 的宽表, 于是 min-content
  // 被撑到 1280px+, flex:1 再也收缩不下去。
  // 实测 (headless Chrome 141, 1024x768, #/orders):
  //   .main-container 宽 1364px, 右边缘 1584px, 超出视口 560px;
  //   .top-header 同样 1364px, 右上角用户头像/退出菜单被推到 536px 之外;
  //   而 documentElement.scrollWidth === clientWidth, 即「没有横向滚动条」
  //   —— 内容不是溢出, 是彻底不可达。
  // 这是 patch 01 修的 min-height:0 在水平方向上的完全对应物。
  // 加上 min-width:0 后它重新可收缩, 宽表随即由 el-table 自己的
  // body-wrapper (overflow-x:auto) 横向滚动 —— 这才是 Element UI 的预期行为。
  min-width: 0;
  margin-left: $sidebar-width;
  display: flex;
  flex-direction: column;
  height: 100vh;          // ✨ 由 min-height 改为 height：给 flex 子项确定的主尺寸
  min-height: 0;          // ✨ 允许被内容压缩，否则 flex 子项不会出现滚动条
  transition: margin-left 0.25s ease;

  .sidebar.collapsed ~ & {
    margin-left: $sidebar-collapsed-width;
  }
}

.collapsed ~ .main-container {
  margin-left: $sidebar-collapsed-width;
}

// ✨ 顶部栏 - 深色主题 @yutiansut @quantaxis
.top-header {
  height: $header-height;
  flex-shrink: 0;         // ✨ 顶栏不参与纵向压缩, 高度恒为 56px (布局预算的依据)
  min-width: 0;           // ✨ 允许横向收缩, 否则右侧用户菜单会把整行撑出视口
  background: $dark-bg-secondary;
  border-bottom: 1px solid $dark-border;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  position: sticky;
  top: 0;
  z-index: 100;
}

.header-left {
  display: flex;
  align-items: center;
  min-width: 0;           // ✨ 允许标题在窄屏被省略号截断而不是撑宽顶栏

  .page-title {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 18px;
    font-weight: 600;
    color: $dark-text-primary;
  }
}

.header-right {
  display: flex;
  align-items: center;
  gap: 20px;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 12px;
  background: rgba(82, 196, 26, 0.15);
  border: 1px solid rgba(82, 196, 26, 0.3);
  border-radius: 20px;

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;

    &.online {
      background: #52c41a;
      box-shadow: 0 0 8px rgba(82, 196, 26, 0.6);
      animation: pulse 2s infinite;
    }
  }

  .status-text {
    font-size: 13px;
    color: #52c41a;
    font-weight: 500;
  }
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.6; }
}

.user-info {
  display: flex;
  align-items: center;
  gap: 12px;
  cursor: pointer;
  padding: 6px 12px;
  border-radius: 8px;
  transition: background 0.2s ease;

  &:hover {
    background: $dark-bg-tertiary;
  }

  .user-avatar {
    background: linear-gradient(135deg, $primary-color 0%, #096dd9 100%);
    color: white;
    font-weight: 600;
  }

  .user-details {
    .user-name {
      font-size: 14px;
      font-weight: 500;
      color: $dark-text-primary;
      line-height: 1.2;
    }

    .user-role {
      margin-top: 2px;
    }
  }

  .el-icon-caret-bottom {
    color: $dark-text-secondary;
    font-size: 12px;
  }
}

// ✨ 用户下拉菜单 - 深色主题 @yutiansut @quantaxis
.user-dropdown-menu {
  background: $dark-bg-secondary !important;
  border: 1px solid $dark-border !important;

  .dropdown-user-info {
    display: flex;
    align-items: center;
    gap: 8px;
    color: $dark-text-secondary;
  }
}

// 内容区域
.content-wrapper {
  flex: 1;
  min-height: 0;          // ✨ 关键：flex item 默认 min-height:auto 会撑破容器，
                          //    导致 overflow 永不触发（整页无滚动条的根因）
  padding: 20px;
  overflow-y: auto;
  overflow-x: hidden;
}

// 页面过渡
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter,
.fade-leave-to {
  opacity: 0;
}

// ✨ 公告通知栏样式 @yutiansut @quantaxis
.announcement-bar {
  height: 40px;
  flex-shrink: 0;         // ✨ 公告条高度恒为 40px, 否则 --qa-content-h 算不准
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
  background: rgba($primary-color, 0.15);
  border-bottom: 1px solid rgba($primary-color, 0.3);
  transition: all 0.3s ease;

  &.announcement-warning {
    background: rgba(250, 173, 20, 0.15);
    border-bottom-color: rgba(250, 173, 20, 0.3);
  }

  &.announcement-error {
    background: rgba(245, 108, 108, 0.15);
    border-bottom-color: rgba(245, 108, 108, 0.3);
  }

  .announcement-content {
    display: flex;
    align-items: center;
    gap: 12px;
    cursor: pointer;
    flex: 1;
    overflow: hidden;

    &:hover .announcement-title {
      color: $primary-color;
    }
  }

  .announcement-icon {
    color: $primary-color;
    font-size: 16px;
    animation: bell-ring 2s infinite;
  }

  .announcement-label {
    flex-shrink: 0;
  }

  .announcement-title {
    color: $dark-text-primary;
    font-size: 14px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    transition: color 0.2s ease;
  }

  .announcement-indicator {
    color: $dark-text-secondary;
    font-size: 12px;
    flex-shrink: 0;
    padding: 2px 8px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 10px;
  }

  .announcement-close {
    cursor: pointer;
    padding: 4px;
    color: $dark-text-secondary;
    transition: color 0.2s ease;
    flex-shrink: 0;

    &:hover {
      color: $dark-text-primary;
    }

    i {
      font-size: 16px;
    }
  }
}

@keyframes bell-ring {
  0%, 50%, 100% { transform: rotate(0); }
  10%, 30% { transform: rotate(10deg); }
  20%, 40% { transform: rotate(-10deg); }
}

// 公告栏滑入动画
.slide-down-enter-active,
.slide-down-leave-active {
  transition: all 0.3s ease;
}

.slide-down-enter,
.slide-down-leave-to {
  transform: translateY(-100%);
  opacity: 0;
}

// 响应式
// ✨ 窄屏改为「抽屉式」侧边栏 @yutiansut @quantaxis
// 原来的做法是把 sidebar 压到 64px, 内容区再让出 64px。在 768px 宽的屏幕上
// 这 64px 是 8.3% 的可用宽度, 却只能显示图标; 内容区反而更挤。
// 改成: 侧边栏整体移出视口, 需要时浮在内容之上(带遮罩), 内容区拿回全部宽度。
@media (max-width: 768px) {
  .sidebar {
    // 抽屉展开时给足 220px 显示完整菜单文字
    width: $sidebar-width;
    transform: translateX(-100%);
    transition: transform 0.25s ease, width 0.25s ease;
    box-shadow: 4px 0 24px rgba(0, 0, 0, 0.6);

    &.collapsed {
      width: $sidebar-width;
    }
  }

  .layout.drawer-open .sidebar {
    transform: translateX(0);
  }

  // ✨ 内容区不再被侧边栏挤压, 拿回整整 64px
  .main-container,
  .collapsed ~ .main-container {
    margin-left: 0;
  }

  .content-wrapper {
    padding: 12px;
  }

  .status-indicator {
    display: none;
  }

  // 抽屉里的折叠按钮改为「关闭」语义, 顶栏那个按钮才是入口
  .sidebar-footer {
    display: none;
  }
}

// ✨ 补齐中间断点 @yutiansut @quantaxis
// 原来全仓库只有 768px 一个断点(共 12 处 @media, 34 个 .vue 一处都没有),
// 1024 / 1366 / 1440 这些最常见的笔记本分辨率完全没有任何适配。
// 断点对齐 Element UI 栅格: lg < 1200 / md < 992 / xs < 768。

// lg 以下 (<1200px): 顶栏瘦身, 让出宽度给内容
@media (max-width: 1199px) {
  .top-header {
    padding: 0 14px;
  }

  .header-right {
    gap: 12px;
  }

  // 用户名/角色两行文字在窄屏没有信息价值, 头像+下拉已足够
  .user-details {
    display: none;
  }
}

// md 以下 (<992px): 去掉纯装饰性的系统状态指示, 收紧 padding
@media (max-width: 991px) {
  .status-indicator {
    display: none;
  }

  .content-wrapper {
    padding: 14px;
  }

  .layout {
    --qa-content-pad: 14px;   // ✨ 预算变量跟着 padding 一起改, 否则各页算高会偏 12px
  }

  .header-left .page-title {
    font-size: 16px;
  }
}

// xs (<768px): 内容区 padding 与原有 768 断点保持一致(12px), 同步预算变量
@media (max-width: 768px) {
  .layout {
    --qa-content-pad: 12px;
    // ✨ 抽屉式侧边栏浮在内容之上, 不占用内容宽度, 所以预算里侧边栏记 0
    --qa-sidebar-w: 0px;
  }

  .layout.sidebar-collapsed {
    --qa-sidebar-w: 0px;
  }
}
</style>
