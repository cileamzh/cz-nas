import chokidar from 'chokidar'
import { exec } from 'child_process'
import { fileURLToPath } from 'url'
import { dirname } from 'path'

// 获取当前模块的路径
const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

// 监听当前文件夹
const folderPath = __dirname // 使用当前目录

// 节流函数：确保命令执行间隔不小于一定时间
let lastExecuted = 0
const throttle = (func, interval) => {
  const now = Date.now()
  if (now - lastExecuted >= interval) {
    lastExecuted = now
    func()
  }
}

// 启动文件夹监听
const watcher = chokidar.watch(folderPath, {
  ignored: /^\./, // 忽略以点（.）开头的文件（如.git、.DS_Store等）
  persistent: true,
  ignoreInitial: true, // 忽略初始状态的触发
})

// 当文件或文件夹有变化时，执行npm run dev命令（节流）
watcher.on('all', (event, path) => {
  console.log(`File ${path} has been ${event}`)

  // 节流：间隔 2000ms 执行 npm run dev
  throttle(() => {
    console.log('Running npm run dev...')
    exec('npm run build', (err, stdout, stderr) => {
      if (err) {
        console.error(`Error executing npm run dev: ${err}`)
        return
      }
      console.log(stdout)
      if (stderr) {
        console.error(stderr)
      }
    })
  }, 2000) // 间隔 2000ms 执行一次
})

console.log(`Watching for changes in ${folderPath}...`)

// 保持进程持续运行，直到手动停止
setInterval(() => {}, 1000) // 保持事件循环
