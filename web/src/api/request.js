import axios from 'axios'
import { Message } from 'element-ui'

// 创建 axios 实例
// @yutiansut @quantaxis
// 使用环境变量配置 API 地址，支持开发和生产环境
const service = axios.create({
  baseURL: process.env.VUE_APP_API_BASE_URL || '/api',
  timeout: 30000
})

// 请求拦截器
service.interceptors.request.use(
  config => {
    // ✨ 带上 JWT @yutiansut @quantaxis
    //
    // 此前这个拦截器是空的(直接 return config),从不发 Authorization 头 ——
    // 而 token 在登录时就已写入 localStorage(store/index.js:159),从没被用过。
    //
    // 后果:后端一旦开启管理端鉴权(QAEX_REQUIRE_ADMIN_AUTH=1),
    // /api/admin/*、/api/management/*、/api/account-admin/* 全部 401,
    // 整个管理端界面不可用 —— 这就是那个开关一直不能开的真正原因。
    //
    // 加上之后:
    //   · 鉴权关闭时 —— 后端忽略这个头,行为不变
    //   · 鉴权开启时 —— 管理员登录后界面正常,非管理员收到 403(符合预期)
    const token = localStorage.getItem('token')
    if (token) {
      config.headers = config.headers || {}
      config.headers['Authorization'] = `Bearer ${token}`
    }
    return config
  },
  error => {
    console.error('Request error:', error)
    return Promise.reject(error)
  }
)

// 响应拦截器
service.interceptors.response.use(
  response => {
    const res = response.data

    // 处理标准响应格式 { success, data, error }
    if (res.hasOwnProperty('success')) {
      if (res.success) {
        return res.data
      } else {
        const errorMsg = res.error && res.error.message || '请求失败'
        Message.error(errorMsg)
        return Promise.reject(new Error(errorMsg))
      }
    }

    // 直接返回数据
    return res
  },
  error => {
    console.error('Response error:', error)
    const errorMsg = (error.response && error.response.data && error.response.data.error && error.response.data.error.message) || error.message
    Message.error(errorMsg || '网络错误')
    return Promise.reject(error)
  }
)

export default service
