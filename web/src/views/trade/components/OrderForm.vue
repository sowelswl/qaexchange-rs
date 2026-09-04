<template>
  <!-- 紧凑下单面板 @yutiansut @quantaxis
       原布局用 el-form 纵向堆了 9 个 form-item(账户/合约/方向/开平/类型/
       价格/数量/预估金额/保证金/按钮),每项独占一行、label 靠左 ——
       在交易面板里这样纵向拉得过长,信息密度太低。
       改为:① 顶部一行摘要(合约+方向+可用) ② 类型与价格/数量并排
             ③ 快捷键贴在输入框下方 ④ 费用估算合成一行 ⑤ 按钮吸底 -->
  <div class="order-form" :class="direction === 'BUY' ? 'is-buy' : 'is-sell'">
    <!-- ① 摘要条:把「合约 / 方向 / 开平」三个只读 form-item 压成一行 -->
    <div class="of-summary">
      <span class="of-inst">{{ instrumentId || '—' }}</span>
      <span class="of-side" :class="direction === 'BUY' ? 'side-buy' : 'side-sell'">
        {{ direction === 'BUY' ? '买入' : '卖出' }}{{ offset === 'OPEN' ? '开仓' : '平仓' }}
      </span>
      <span class="of-avail" v-if="currentAccount">
        可用 ¥{{ Math.round(currentAccount.available).toLocaleString() }}
      </span>
    </div>

    <el-form :model="form" :rules="rules" ref="form" label-position="top" size="small">
      <!-- ② 账户 + 订单类型 同一行 -->
      <div class="of-row">
        <el-form-item label="交易账户" prop="account_id" class="of-grow">
          <el-select
            v-model="form.account_id"
            placeholder="选择账户"
            style="width: 100%"
            @change="handleAccountChange"
          >
            <el-option
              v-for="account in accounts"
              :key="account.account_id"
              :label="account.account_name"
              :value="account.account_id"
            >
              <span>{{ account.account_name }}</span>
              <span class="of-opt-avail">¥{{ Math.round(account.available).toLocaleString() }}</span>
            </el-option>
          </el-select>
        </el-form-item>
        <el-form-item label="订单类型" prop="order_type" class="of-type">
          <el-radio-group v-model="form.order_type" size="small" @change="handleTypeChange">
            <el-radio-button label="LIMIT">限价</el-radio-button>
            <el-radio-button label="MARKET">市价</el-radio-button>
          </el-radio-group>
        </el-form-item>
      </div>

      <!-- ③ 价格 + 数量 同一行,快捷键紧贴各自输入框下方 -->
      <div class="of-row">
        <el-form-item
          label="价格"
          prop="price"
          class="of-half"
          v-if="form.order_type === 'LIMIT'"
        >
          <el-input-number
            v-model="form.price"
            :min="0"
            :step="0.2"
            :precision="1"
            size="small"
            style="width: 100%"
            controls-position="right"
          />
          <div class="of-chips">
            <button type="button" @click="setPriceOffset(-5)">-5</button>
            <button type="button" @click="setPriceOffset(-2)">-2</button>
            <button type="button" @click="setPriceOffset(-1)">-1</button>
            <button type="button" class="chip-primary" @click="setCurrentPrice">现价</button>
            <button type="button" @click="setPriceOffset(1)">+1</button>
            <button type="button" @click="setPriceOffset(2)">+2</button>
            <button type="button" @click="setPriceOffset(5)">+5</button>
          </div>
        </el-form-item>
        <el-form-item
          label="数量（手）"
          prop="volume"
          :class="form.order_type === 'LIMIT' ? 'of-half' : 'of-grow'"
        >
          <el-input-number
            v-model="form.volume"
            :min="1"
            :max="100"
            :step="1"
            size="small"
            style="width: 100%"
            controls-position="right"
          />
          <div class="of-chips">
            <button type="button" @click="setVolume(1)">1</button>
            <button type="button" @click="setVolume(5)">5</button>
            <button type="button" @click="setVolume(10)">10</button>
            <button type="button" @click="setVolume(20)">20</button>
          </div>
        </el-form-item>
      </div>

      <!-- ④ 费用估算:原来是两个独占一行的 form-item,压成一行两列 -->
      <div class="of-estimate">
        <div class="of-est-item">
          <span class="est-label">预估金额</span>
          <span class="est-value">¥{{ estimatedAmount.toLocaleString() }}</span>
        </div>
        <div class="of-est-item">
          <span class="est-label">预估保证金</span>
          <span class="est-value">¥{{ estimatedMargin.toLocaleString() }}</span>
        </div>
      </div>

      <!-- ⑤ 提交按钮吸底 -->
      <el-button
        type="primary"
        class="of-submit"
        :class="direction === 'BUY' ? 'buy-button' : 'sell-button'"
        @click="handleSubmit"
        :loading="submitting"
      >
        {{ direction === 'BUY' ? '买入' : '卖出' }}{{ offset === 'OPEN' ? '开仓' : '平仓' }}
      </el-button>
    </el-form>
  </div>
</template>

<script>
import { getUserAccounts } from '@/api'
import { mapGetters } from 'vuex'

export default {
  name: 'OrderForm',
  props: {
    instrumentId: {
      type: String,
      required: true
    },
    currentPrice: {
      type: Number,
      default: 0
    },
    direction: {
      type: String,
      required: true,
      validator: val => ['BUY', 'SELL'].includes(val)
    },
    offset: {
      type: String,
      required: true,
      validator: val => ['OPEN', 'CLOSE'].includes(val)
    }
  },
  data() {
    return {
      submitting: false,
      accounts: [],
      selectedAccount: null,
      form: {
        account_id: '',
        order_type: 'LIMIT',
        price: 3800,
        volume: 1
      },
      rules: {
        account_id: [{ required: true, message: '请选择交易账户', trigger: 'change' }],
        order_type: [{ required: true, message: '请选择订单类型' }],
        price: [{ required: true, message: '请输入价格' }],
        volume: [
          { required: true, message: '请输入数量' },
          { type: 'number', min: 1, max: 100, message: '数量范围 1-100' }
        ]
      }
    }
  },
  computed: {
    // 当前选中账户 —— 摘要条显示可用资金用 @yutiansut @quantaxis
    currentAccount() {
      return this.accounts.find(a => a.account_id === this.form.account_id) || null
    },

    ...mapGetters(['currentUser']),
    estimatedAmount() {
      if (this.form.order_type === 'LIMIT') {
        return this.form.price * this.form.volume * 300
      } else {
        return this.currentPrice * this.form.volume * 300
      }
    },
    estimatedMargin() {
      return this.estimatedAmount * 0.15  // 15% 保证金率
    }
  },
  mounted() {
    this.loadAccounts()
  },
  watch: {
    currentPrice: {
      immediate: true,
      handler(val) {
        if (val && this.form.price === 0) {
          this.form.price = val
        }
      }
    }
  },
  methods: {
    async loadAccounts() {
      if (!this.currentUser) {
        this.$message.warning('请先登录')
        return
      }

      try {
        const res = await getUserAccounts(this.currentUser)
        this.accounts = res.accounts || []

        // 自动选择第一个账户
        if (this.accounts.length > 0 && !this.form.account_id) {
          this.form.account_id = this.accounts[0].account_id
          this.selectedAccount = this.accounts[0]
          // 通知父组件初始账户选择
          this.$emit('account-change', this.form.account_id)
        }
      } catch (error) {
        this.$message.error('加载账户列表失败: ' + (error.message || '未知错误'))
      }
    },

    handleAccountChange(accountId) {
      this.selectedAccount = this.accounts.find(acc => acc.account_id === accountId)
      // 通知父组件账户选择变化
      this.$emit('account-change', accountId)
    },

    handleTypeChange() {
      if (this.form.order_type === 'LIMIT' && this.form.price === 0) {
        this.form.price = this.currentPrice
      }
    },

    setCurrentPrice() {
      this.form.price = this.currentPrice
    },

    setPriceOffset(offset) {
      this.form.price = Math.max(0, this.form.price + offset * 0.2)
    },

    setVolume(volume) {
      this.form.volume = volume
    },

    handleSubmit() {
      this.$refs.form.validate(valid => {
        if (valid) {
          this.submitting = true

          // ✨ 交易所模式：user_id 和 account_id 都使用账户ID @yutiansut @quantaxis
          // 原因：交易所只关心账户，User→Account映射是经纪商业务
          const orderData = {
            user_id: this.form.account_id,    // 交易所模式：使用账户ID
            account_id: this.form.account_id, // 交易账户ID
            direction: this.direction,
            offset: this.offset,
            order_type: this.form.order_type,
            price: this.form.order_type === 'LIMIT' ? this.form.price : 0,
            volume: this.form.volume
          }

          this.$emit('submit', orderData)

          setTimeout(() => {
            this.submitting = false
          }, 1000)
        }
      })
    }
  }
}
</script>

<style lang="scss" scoped>
@import '@/styles/variables.scss';
/* 紧凑下单面板样式 @yutiansut @quantaxis
   目标:在不改变任何交易语义的前提下提高信息密度 ——
   原布局 9 个纵向 form-item 约需 620px 高,现约 340px。 */
.order-form {
  --of-accent: #f56c6c;

  &.is-sell { --of-accent: #67c23a; }

  /* ① 摘要条:合约 / 方向 / 可用资金 */
  .of-summary {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-wrap: wrap;
    padding: 8px 10px;
    margin-bottom: 12px;
    background: $dark-bg-tertiary;
    border-left: 3px solid var(--of-accent);
    border-radius: 3px;
    font-size: 13px;

    .of-inst { color: $dark-text-primary; font-weight: 600; font-size: 15px; letter-spacing: .3px; }
    .of-side {
      padding: 1px 7px;
      border-radius: 2px;
      color: #fff;
      font-size: 12px;
      &.side-buy  { background: #f56c6c; }
      &.side-sell { background: #67c23a; }
    }
    .of-avail { margin-left: auto; color: $dark-text-secondary; font-size: 12px; }
  }

  /* ② 行容器:让同类字段并排,窄屏自动堆叠 */
  .of-row {
    display: flex;
    gap: 12px;
    flex-wrap: wrap;
    .of-grow { flex: 1 1 200px; min-width: 0; }
    .of-half { flex: 1 1 calc(50% - 6px); min-width: 0; }
    .of-type { flex: 0 0 auto; }
  }

  /* form-item 压紧:label 置顶且小字号 */
  ::v-deep .el-form-item { margin-bottom: 12px; }
  ::v-deep .el-form-item__label {
    padding: 0 0 2px;
    line-height: 1.4;
    font-size: 12px;
    color: #909399;
  }
  ::v-deep .el-form-item__content { line-height: 1.4; }

  /* ③ 快捷键:用原生 button,避免 el-button 的 margin 把行撑开 */
  .of-chips {
    display: flex;
    gap: 4px;
    margin-top: 5px;
    button {
      flex: 1;
      min-width: 0;
      padding: 2px 0;
      font-size: 11px;
      line-height: 1.6;
      color: $dark-text-secondary;
      background: $dark-bg-tertiary;
      border: 1px solid $dark-border;
      border-radius: 2px;
      cursor: pointer;
      transition: all .15s;
      &:hover { color: var(--of-accent); border-color: var(--of-accent); }
      &.chip-primary {
        color: $primary-color;
        border-color: rgba($primary-color, .5);
        background: rgba($primary-color, .12);
      }
    }
  }

  /* ④ 费用估算:一行两列 */
  .of-estimate {
    display: flex;
    gap: 12px;
    padding: 8px 10px;
    margin-bottom: 12px;
    background: $dark-bg-tertiary;
    border: 1px solid $dark-border;
    border-radius: 3px;
    .of-est-item {
      flex: 1;
      display: flex;
      justify-content: space-between;
      align-items: baseline;
      min-width: 0;
      .est-label { color: $dark-text-secondary; font-size: 12px; }
      .est-value {
        color: $dark-text-primary;
        font-weight: 600;
        font-size: 13px;
        font-variant-numeric: tabular-nums;
        white-space: nowrap;
      }
    }
  }

  /* ⑤ 提交按钮 */
  .of-submit {
    width: 100%;
    height: 40px;
    font-size: 15px;
    font-weight: 600;
    letter-spacing: 2px;
    &.buy-button  { background: #f56c6c; border-color: #f56c6c; }
    &.sell-button { background: #67c23a; border-color: #67c23a; }
  }

  .of-opt-avail { float: right; color: #8492a6; font-size: 12px; }
}

/* ⚠️ 这里原本有一段 html[data-theme='dark'] 的兜底 —— 已删除。
   本项目**不使用 data-theme 切换**:深色就是默认样式,变量定义在
   styles/variables.scss:19-26。写 data-theme 选择器永远不会命中,
   结果是浅色值直接生效(实测 .of-estimate 呈 rgb(250,250,250))。
   @yutiansut @quantaxis */

/* 窄屏:价格与数量各占整行 */
@media (max-width: 768px) {
  .order-form .of-row .of-half { flex: 1 1 100%; }
}
</style>