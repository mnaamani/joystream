#!/usr/bin/env node
/* es-lint disable*/

'use strict'

// Node requires
const path = require('path')

// npm requires
const meow = require('meow')
const chalk = require('chalk')
const figlet = require('figlet')
const _ = require('lodash')

const debug = require('debug')('joystream:colossus')

// Project root
const PROJECT_ROOT = path.resolve(__dirname, '..')

// Number of milliseconds to wait between synchronization runs.
const SYNC_PERIOD_MS = 300000 // 5min

// Parse CLI
const FLAG_DEFINITIONS = {
  port: {
    type: 'number',
    alias: 'p',
    default: 3000,
  },
  keyFile: {
    type: 'string',
    isRequired: (flags, input) => {
      // Only required if running server command and not in dev mode
      const serverCmd = input[0] === 'server'
      return !flags.dev && serverCmd
    },
  },
  publicUrl: {
    type: 'string',
    alias: 'u',
    isRequired: (flags, input) => {
      // Only required if running server command and not in dev mode
      const serverCmd = input[0] === 'server'
      return !flags.dev && serverCmd
    },
  },
  passphrase: {
    type: 'string',
  },
  wsProvider: {
    type: 'string',
    default: 'ws://localhost:9944',
  },
  providerId: {
    type: 'number',
    alias: 'i',
    isRequired: (flags, input) => {
      // Only required if running server command and not in dev mode
      const serverCmd = input[0] === 'server'
      return !flags.dev && serverCmd
    },
  },
}

const cli = meow(
  `
  Usage:
    $ colossus [command] [arguments]

  Commands:
    leacher         leacher node.

  Arguments (optional):
    --dev                   Runs server with developer settings.
    --passphrase            Optional passphrase to use to decrypt the key-file.
    --port=PORT, -p PORT    Port number to listen on, defaults to 3000.
    --ws-provider WS_URL    Joystream-node websocket provider, defaults to ws://localhost:9944
  `,
  { flags: FLAG_DEFINITIONS }
)

// All-important banner!
function banner() {
  console.log(chalk.blue(figlet.textSync('joystream', 'Speed')))
}

// Get an initialized storage instance
function getStorage(runtimeApi) {
  // TODO at some point, we can figure out what backend-specific connection
  // options make sense. For now, just don't use any configuration.
  const { Storage } = require('@joystream/storage-node-backend')

  const options = {
    resolve_content_id: async (contentId) => {
      // Resolve via API
      const obj = await runtimeApi.assets.getDataObject(contentId)
      if (!obj || obj.isNone) {
        return
      }
      // if obj.liaison_judgement !== Accepted .. throw ?
      return obj.unwrap().ipfs_content_id.toString()
    },
  }

  return Storage.create(options)
}

async function initApiProduction({ wsProvider, providerId, keyFile, passphrase }) {
  // Load key information
  const { RuntimeApi } = require('@joystream/storage-runtime-api')

  const api = await RuntimeApi.create({
    account_file: keyFile,
    passphrase,
    provider_url: wsProvider,
    storageProviderId: providerId,
  })

  await api.untilChainIsSynced()

  return api
}

async function startColossus({ api }) {
  // TODO: check valid url, and valid port number
  const store = getStorage(api)
  banner()
  const { startSyncing } = require('../lib/sync')
  startSyncing(api, { syncPeriod: SYNC_PERIOD_MS }, store)
}

const commands = {
  leacher: async () => {
    const api = await initApiProduction(cli.flags)
    startColossus({ api })
    await new Promise(function (resolve, reject) {
      // do nothing
    })
  },
}

async function main() {
  // Simple CLI commands
  let command = cli.input[0]
  if (!command) {
    command = 'leacher'
  }

  if (Object.prototype.hasOwnProperty.call(commands, command)) {
    // Command recognized
    const args = _.clone(cli.input).slice(1)
    await commands[command](...args)
  } else {
    throw new Error(`Command '${command}' not recognized, aborting!`)
  }
}

main()
  .then(() => {
    process.exit(0)
  })
  .catch((err) => {
    console.error(chalk.red(err.stack))
    process.exit(-1)
  })
