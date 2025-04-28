import FileExploer from '@/components/FileExploer.vue'
import Setting from '@/components/SettingApp.vue'
import { reactive } from 'vue'

// user info manage
export const user = reactive({
  info: {
    avator: '',
    username: '',
    bg: '/bg.png',
  },
  user_dir: {},
})

export const msg_map = reactive(new Map())

export function rm_msg(idx) {
  msg_map.splice(idx, 1)
}

export function add_msg(msg) {
  let key = Date.now()
  msg.idx = msg_map.size
  msg_map.set(key, msg)
  setTimeout(
    () => {
      msg_map.delete(key)
    },
    isNaN(msg.last) ? 2000 : msg.last,
  )
}
// tasks manage
export const tasks = reactive(new Map())

export function add_task(task) {
  tasks.set(task.name, {
    name: task.name,
    icon: task.icon,
    component: task.component,
    minimize: false,
  })
  set_index(task.name)
}
export function set_index(key) {
  tasks.forEach((v, k) => {
    if (key != k) {
      v.zIndex = 1
    } else {
      v.zIndex = 9
    }
  })
}
export function minimize(key) {
  tasks.forEach((v, k) => {
    if (key != k) {
      v.minimize = false
    } else {
      v.minimize = true
    }
  })
}
export function showtask(key) {
  tasks.forEach((v, k) => {
    if (key == k) {
      v.minimize = false
    }
  })
}
// define apps
export const apps = reactive([
  { name: 'setting', component: Setting, icon: '/set.png' },
  {
    name: 'file_manager',
    component: FileExploer,
    icon: '/folder.png',
  },
])

// explorer status manage
export const explorer_status = reactive({
  is_dargging: false,
  current_dir: { dir: [], path: '' },
  dir_tree: {},
  mode: {
    list: ['list', 'view'],
    current: '',
  },
  init() {
    explorer_status.mode.current = localStorage.getItem('explorer-mode') || 'view'
  },
})

export function load_current_dir(path) {
  fetch('/auth/open/' + path)
    .then((res) => res.json())
    .then((rst) => {
      const newRows = rst.rows
      const oldRows = explorer_status.current_dir.dir

      // 1. 创建一个映射，方便查找
      const oldMap = new Map(oldRows.map((item) => [item.path, item]))

      const updated = []

      for (const newItem of newRows) {
        const oldItem = oldMap.get(newItem.path)
        if (oldItem) {
          // 已存在，更新内容
          Object.assign(oldItem, newItem)
        } else {
          // 不存在，是新增的
          updated.push(newItem)
        }
      }

      // 2. 如果有新增，追加到旧数组
      if (updated.length > 0) {
        explorer_status.current_dir.dir.push(...updated)
      }

      // 3. 删除那些在新数据里已经不存在的
      explorer_status.current_dir.dir = explorer_status.current_dir.dir.filter((item) =>
        newRows.some((newItem) => newItem.path === item.path),
      )

      // 4. 最后，判断是否需要改路径
      if (explorer_status.current_dir.path !== path) {
        explorer_status.current_dir.path = path
      }
    })
}

// lang setting
export const lang = reactive({
  list: [
    { name: '中文', value: 'zh' },
    { name: 'English', value: 'en' },
    { name: '日本語', value: 'ja' },
    { name: '한국어', value: 'ko' },
  ],
  current: null,
  selected: '',
  zh: {
    close: '关闭',
    minimize: '最小化',
    username: '用户名',
    password: '密码',
    file_manager: '文件资源管理',
    setting: '设置',
    set_avator: '设为头像',
    file_manager_mode: '文件管理查看模式',
    login_header: '欢迎使用Cz NAS',
    login: '登录',
    view: '视图',
    list: '列表',
    rename: '重命名',
    delete: '删除',
    failed_rename: '重命名失败',
    success_rename: '重命名成功',
    failed_delete: '删除失败',
    success_delete: '删除成功',
    success_upload: '上传成功',
    fail_upload: '上传失败',
    dir_name: '文件夹名称',
    mkdir: '创建文件夹',
    fail_mkdir: '创建文件夹失败',
    success_mkdir: '创建文件夹成功',
    cancel_mkdir: '已取消创建',
    cancel: '已取消',
    confirm_delete: '确认删除',
    download: '下载',
    success_set: '设置成功',
    fail_set: '设置失败',
    set_as_bg: '设为壁纸',
  },
  en: {
    minimize: 'minimize',
    close: 'close',
    username: 'username',
    password: 'password',
    login: 'Log In',
    login_header: 'Welcome To Cz NAS',
    file_manager: 'File Manager',
    setting: 'Setting',
    rename: 'rename',
    set_avator: 'set as avator',
    file_manager_mode: 'file manager view mode',
    view: 'view',
    list: 'list',
    delete: 'delete',
    failed_rename: 'fail to rename',
    success_rename: 'success to rename',
    failed_delete: 'fail to delete',
    success_delete: 'success to delete',
    dir_name: 'dir name',
    mkdir: 'make dir',
    fail_mkdir: 'fail to make dir',
    success_upload: 'success to upload',
    fail_upload: 'fail to upload',
    success_mkdir: 'success to make dir',
    cancel_mkdir: 'stop making dir',
    cancel: 'canceled',
    confirm_delete: 'confirm to delete',
    download: 'download',
    success_set: 'success to set',
    fail_set: 'fail to set',
  },
  ja: {
    minimize: '最小化',
    close: '閉じる',
    username: 'ユーザー名',
    password: 'パスワード',
    login: 'ログイン',
    login_header: 'Cz NASへようこそ',
    file_manager: 'ファイルマネージャー',
    setting: '設定',
    rename: '名前を変更',
    set_avator: 'アバターに設定',
    file_manager_mode: 'ファイル管理モード',
    view: 'ビュー',
    list: 'リスト',
    delete: '削除',
    failed_rename: '名前変更に失敗しました',
    success_rename: '名前変更に成功しました',
    failed_delete: '削除に失敗しました',
    success_delete: '削除に成功しました',
    dir_name: 'フォルダー名',
    mkdir: 'フォルダーを作成',
    fail_mkdir: 'フォルダー作成に失敗しました',
    success_upload: 'アップロード成功',
    fail_upload: 'アップロード失敗',
    success_mkdir: 'フォルダー作成成功',
    cancel_mkdir: '作成をキャンセルしました',
    cancel: 'キャンセルしました',
    confirm_delete: '削除を確認',
    download: 'ダウンロード',
    success_set: '設定成功',
    fail_set: '設定失敗',
  },
  ko: {
    minimize: '최소화',
    close: '닫기',
    username: '사용자 이름',
    password: '비밀번호',
    login: '로그인',
    login_header: 'Cz NAS에 오신 것을 환영합니다',
    file_manager: '파일 관리자',
    setting: '설정',
    rename: '이름 변경',
    set_avator: '아바타로 설정',
    file_manager_mode: '파일 관리자 보기 모드',
    view: '보기',
    list: '목록',
    delete: '삭제',
    failed_rename: '이름 변경 실패',
    success_rename: '이름 변경 성공',
    failed_delete: '삭제 실패',
    success_delete: '삭제 성공',
    dir_name: '폴더 이름',
    mkdir: '폴더 만들기',
    fail_mkdir: '폴더 만들기 실패',
    success_upload: '업로드 성공',
    fail_upload: '업로드 실패',
    success_mkdir: '폴더 만들기 성공',
    cancel_mkdir: '폴더 만들기 취소',
    cancel: '취소됨',
    confirm_delete: '삭제 확인',
    download: '다운로드',
    success_set: '설정 성공',
    fail_set: '설정 실패',
  },
  init() {
    lang.current =
      lang[
        JSON.parse(
          localStorage.getItem('lang') || JSON.stringify({ name: '中文', value: 'zh' }),
        ).value
      ]
    lang.selected = JSON.parse(
      localStorage.getItem('lang') || JSON.stringify({ name: '中文', value: 'zh' }),
    )
  },
})

export const context_menu = reactive({
  x: 0,
  y: 0,
  visible: false,
  component: null,
  data: null,
})
