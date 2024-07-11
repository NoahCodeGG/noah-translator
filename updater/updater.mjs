import fs from 'node:fs'
import fetch from 'node-fetch'

async function resolveUpdater() {
  if (process.env.GITHUB_TOKEN === undefined) {
    throw new Error('GITHUB_TOKEN is required')
  }

  const TOKEN = process.env.GITHUB_TOKEN
  const version = await getVersion(TOKEN)
  const changelog = await getChangeLog(TOKEN)

  const darwin_aarch64 = `https://github.com/NoahCodeGG/noah-translator/releases/download/${version}/noah-translator_${version}_aarch64.app.tar.gz`
  const darwin_aarch64_sig = await getSignature(`${darwin_aarch64}.sig`)
  const darwin_x86_64 = `https://github.com/NoahCodeGG/noah-translator/releases/download/${version}/noah-translator_${version}_x64.app.tar.gz`
  const darwin_x86_64_sig = await getSignature(`${darwin_x86_64}.sig`)
  const windows_x86_64 = `https://github.com/NoahCodeGG/noah-translator/releases/download/${version}/noah-translator_${version}_x64-setup.nsis.zip`
  const windows_x86_64_sig = await getSignature(`${windows_x86_64}.sig`)
  const windows_i686 = `https://github.com/NoahCodeGG/noah-translator/releases/download/${version}/noah-translator_${version}_x86-setup.nsis.zip`
  const windows_i686_sig = await getSignature(`${windows_i686}.sig`)
  const windows_aarch64 = `https://github.com/NoahCodeGG/noah-translator/releases/download/${version}/noah-translator_${version}_arm64-setup.nsis.zip`
  const windows_aarch64_sig = await getSignature(`${windows_aarch64}.sig`)

  const updateData = {
    name: version,
    notes: changelog,
    pub_date: new Date().toISOString(),
    platforms: {
      'darwin-aarch64': { signature: darwin_aarch64_sig, url: `https://ghproxy.com/${darwin_aarch64}` },
      'darwin-x86_64': { signature: darwin_x86_64_sig, url: `https://ghproxy.com/${darwin_x86_64}` },
      'windows-x86_64': { signature: windows_x86_64_sig, url: `https://ghproxy.com/${windows_x86_64}` },
      'windows-i686': { signature: windows_i686_sig, url: `https://ghproxy.com/${windows_i686}` },
      'windows-aarch64': { signature: windows_aarch64_sig, url: `https://ghproxy.com/${windows_aarch64}` },
    },
  }
  fs.writeFile('./update.json', JSON.stringify(updateData), (e) => {
    console.log(e)
  })
}

async function getVersion(token) {
  const res = await fetch('https://api.github.com/repos/NoahCodeGG/noah-translator/releases/latest', {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${token}`,
    },
  })

  if (res.ok) {
    const data = await res.json()
    if (data.tag_name) {
      return data.tag_name
    }
  }
}

async function getChangeLog(token) {
  const res = await fetch('https://api.github.com/repos/NoahCodeGG/noah-translator/releases/latest', {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${token}`,
    },
  })

  if (res.ok) {
    const data = await res.json()
    if (data.body) {
      return data.body
    }
  }
}

async function getSignature(url) {
  const response = await fetch(url, {
    method: 'GET',
    headers: { 'Content-Type': 'application/octet-stream' },
  })

  if (response.ok) {
    return response.text()
  }

  return ''
}

resolveUpdater().catch(console.error)
