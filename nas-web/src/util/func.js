export function set_move_win(win_class, move_bar_class) {
  let isDragging = false
  let offsetX = 0
  let offsetY = 0
  let currentWin = null

  document.addEventListener('mousedown', (e) => {
    const header = e.target.closest('.' + move_bar_class)
    if (!header) return

    const win = header.closest('.' + win_class)
    if (!win) return

    isDragging = true
    currentWin = win
    offsetX = e.clientX - win.offsetLeft
    offsetY = e.clientY - win.offsetTop

    document.addEventListener('mousemove', onMouseMove)
    document.addEventListener('mouseup', onMouseUp)
  })

  function onMouseMove(e) {
    if (!isDragging || !currentWin) return
    currentWin.style.position = 'absolute'
    currentWin.style.left = e.clientX - offsetX + 'px'
    currentWin.style.top = e.clientY - offsetY + 'px'
  }

  function onMouseUp() {
    isDragging = false
    currentWin = null
    document.removeEventListener('mousemove', onMouseMove)
    document.removeEventListener('mouseup', onMouseUp)
  }
}

export const imgextlist = ['.jpg', '.png', '.webp', '.ico', '.svg']
export const docextlist = ['.docx', '.txt', '.md']
export const langextlist = ['.rs', '.js', '.java', '.py', '.c']
export const videoextlist = ['.mp4', '.webm', '.mov']

/**
 * @param {Object} f 文件对象
 * @param {Boolean} previewVideo 是否尝试生成视频第一帧
 * @returns Promise<string> 预览图 URL
 */
export async function get_file_url(f, previewVideo = true) {
  const fname = f.name + ''

  if (f.is_dir) return '/dir.png'

  for (const ext of langextlist) {
    if (fname.endsWith(ext)) {
      return '/' + ext.replace('.', '') + '.png'
    }
  }

  for (const ext of docextlist) {
    if (fname.endsWith(ext)) {
      return '/txt.png'
    }
  }

  for (const ext of imgextlist) {
    if (fname.endsWith(ext)) {
      return '/auth/open/' + f.path
    }
  }

  for (const ext of videoextlist) {
    if (fname.endsWith(ext)) {
      if (previewVideo) {
        try {
          const thumb = await extractVideoThumbnail('/auth/open/' + f.path)
          return thumb
        } catch (e) {
          console.log(e)
          return '/video.png'
        }
      }
      return '/video.png'
    }
  }

  return '/unknown.png'
}

export async function extractVideoThumbnail(videoUrl, seekTo = 0.1) {
  return new Promise((resolve, reject) => {
    const video = document.createElement('video')
    video.src = videoUrl
    video.crossOrigin = 'anonymous' // 若为跨域资源

    video.onloadedmetadata = () => {
      // 若视频长度太短，确保 seekTo 不超过 duration
      if (seekTo > video.duration) seekTo = 0

      video.currentTime = seekTo
    }

    video.onseeked = () => {
      const canvas = document.createElement('canvas')
      canvas.width = video.videoWidth
      canvas.height = video.videoHeight

      const ctx = canvas.getContext('2d')
      ctx.drawImage(video, 0, 0, canvas.width, canvas.height)
      canvas.toBlob((blob) => {
        const thumbnailUrl = URL.createObjectURL(blob)
        resolve(thumbnailUrl)
      }, 'image/jpeg')
    }

    video.onerror = (e) => {
      console.log(e)
      reject('视频加载失败')
    }
  })
}

export function is_photo(fname) {
  for (const ext of imgextlist) {
    if (fname.endsWith(ext)) {
      return true
    }
  }
  false
}

export async function upload(path, files) {
  if (!files.length) {
    return
  }
  const form_data = new FormData()
  form_data.append('path', path)
  for (const file of files) {
    form_data.append('files', file)
  }
  return fetch('/auth/upload', { method: 'POST', body: form_data })
}

export async function rename_file(old_path, new_name) {
  return fetch('/auth/rename', {
    method: 'POST',
    body: JSON.stringify({ old_path: old_path, new_name: new_name }),
    headers: { 'Content-Type': 'application/json' },
  })
}

export async function delete_file(path) {
  return fetch('/auth/delete', {
    method: 'POST',
    body: JSON.stringify({ path }),
    headers: { 'Content-Type': 'application/json' },
  })
}

export async function mkdir(path) {
  return fetch('/auth/mkdir', {
    method: 'POST',
    body: JSON.stringify({ path }),
    headers: { 'Content-Type': 'application/json' },
  })
}

export async function set_avator(path) {
  console.log(path)

  return fetch('/auth/set_avator', {
    method: 'POST',
    body: JSON.stringify({ avator_path: path }),
    headers: { 'Content-Type': 'application/json' },
  })
}
export async function set_as_bg(path) {
  return fetch('/auth/set_bg', {
    method: 'POST',
    body: JSON.stringify({ path: path }),
    headers: { 'Content-Type': 'application/json' },
  })
}
