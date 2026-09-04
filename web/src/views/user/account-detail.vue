<template>
  <!-- ⚠️ 本页数据来自 /api/management/account/{id}/detail,
     它返回的是 **QIFI 视图**(qifi.orders / qifi.positions),
     字段名与交易所视图不同 —— 绑错了整列空白:
       cost_long   → open_cost_long      cost_short → open_cost_short
       price       → limit_price         volume     → volume_orign
     另注:QIFI 的 order_id 是 qars 内部 UUID,与订单管理页显示的
     交易所订单号(O178846...)是两套编号,同一笔单对不上。
     @yutiansut @quantaxis -->

  <div class="account-detail" v-loading="loading">
    <div class="page-header">
      <el-button icon="el-icon-arrow-left" @click="$router.back()">返回</el-button>
      <h2>账户详情 - {{ accountId }}</h2>
    </div>

    <el-card shadow="hover" class="info-card" v-if="detail">
      <el-descriptions :column="2" border>
        <el-descriptions-item label="账户ID">{{ accountId }}</el-descriptions-item>
        <el-descriptions-item label="资金">{{ formatCurrency(detail.account_info.balance) }}</el-descriptions-item>
        <el-descriptions-item label="可用资金">{{ formatCurrency(detail.account_info.available) }}</el-descriptions-item>
        <el-descriptions-item label="占用保证金">{{ formatCurrency(detail.account_info.margin) }}</el-descriptions-item>
        <el-descriptions-item label="浮动盈亏">
          <span :class="detail.account_info.position_profit >= 0 ? 'profit' : 'loss'">
            {{ formatCurrency(detail.account_info.position_profit) }}
          </span>
        </el-descriptions-item>
        <el-descriptions-item label="风险率">
          {{ (detail.account_info.risk_ratio * 100).toFixed(2) }}%
        </el-descriptions-item>
      </el-descriptions>
    </el-card>

    <el-row :gutter="20" v-if="detail">
      <el-col :xs="24" :lg="12">
        <el-card shadow="hover">
          <div slot="header">持仓</div>
          <el-table :data="detail.positions" height="300" size="mini" border>
            <el-table-column prop="instrument_id" label="合约" min-width="120" show-overflow-tooltip/>
            <el-table-column prop="volume_long" label="多头" width="80" align="right" />
            <el-table-column prop="volume_short" label="空头" width="80" align="right" />
            <el-table-column prop="open_cost_long" label="多头均价" min-width="120" align="right" />
            <el-table-column prop="open_cost_short" label="空头均价" min-width="120" align="right" />
            <el-table-column prop="float_profit" label="浮动盈亏" min-width="120" align="right">
              <template slot-scope="{ row }">
                <span :class="row.float_profit >= 0 ? 'profit' : 'loss'">
                  {{ formatCurrency(row.float_profit) }}
                </span>
              </template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-col>

      <el-col :xs="24" :lg="12">
        <el-card shadow="hover">
          <div slot="header">订单</div>
          <el-table :data="detail.orders" height="300" size="mini" border>
            <!-- 两个订单号并存,不是冗余 @yutiansut @quantaxis
                 order_id          = qars 内部 UUID(本页所属的 QIFI 视图)
                 exchange_order_id = 交易所订单号(订单管理页/DIFF WS 显示的那个)
                 只显示前者会导致本页的单在订单管理页里查不到。 -->
            <el-table-column prop="order_id" label="内部单号" min-width="150" show-overflow-tooltip/>
            <el-table-column prop="exchange_order_id" label="交易所单号" min-width="150" show-overflow-tooltip/>
            <el-table-column prop="instrument_id" label="合约" min-width="120" show-overflow-tooltip/>
            <el-table-column prop="direction" label="方向" width="80" align="center" />
            <el-table-column prop="offset" label="开平" width="80" align="center" />
            <el-table-column prop="limit_price" label="价格" min-width="100" align="right" />
            <el-table-column prop="volume_orign" label="数量" width="80" align="right" />
            <!-- QIFI 只有 volume_left(剩余),交易所视图只有 filled_volume(已成)——
                 两者互补。这里由 volume_orign - volume_left 算出已成交量,
                 避免用户在两个页面看到不同口径。 -->
            <el-table-column label="已成交" width="90" align="right">
              <template slot-scope="scope">
                {{ (Number(scope.row.volume_orign) || 0) - (Number(scope.row.volume_left) || 0) }}
              </template>
            </el-table-column>
            <el-table-column prop="last_msg" label="回报" min-width="110" show-overflow-tooltip />
            <el-table-column prop="status" label="状态" min-width="100" />
          </el-table>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script>
import { getAccountDetail } from '@/api'

export default {
  name: 'AccountDetail',
  data() {
    return {
      loading: false,
      detail: null
    }
  },
  computed: {
    accountId() {
      return this.$route.params.accountId
    }
  },
  watch: {
    accountId() {
      this.fetchDetail()
    }
  },
  mounted() {
    this.fetchDetail()
  },
  methods: {
    async fetchDetail() {
      if (!this.accountId) return
      this.loading = true
      try {
        const data = await getAccountDetail(this.accountId)
        this.detail = {
          account_info: data.account_info || {},
          positions: data.positions || [],
          orders: data.orders || []
        }
      } catch (error) {
        this.$message.error('加载账户详情失败')
        console.error(error)
        this.detail = null
      } finally {
        this.loading = false
      }
    },
    formatCurrency(value) {
      return Number(value || 0).toLocaleString('zh-CN', { minimumFractionDigits: 2 })
    }
  }
}
</script>

<style scoped>
.account-detail {
  padding: 20px;
}

.page-header {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 20px;
}

.info-card {
  margin-bottom: 20px;
}

.profit {
  color: #F56C6C;
}

.loss {
  color: #67C23A;
}
</style>
