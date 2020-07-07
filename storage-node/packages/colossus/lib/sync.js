/*
 * This file is part of the storage node for the Joystream project.
 * Copyright (C) 2019 Joystream Contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

'use strict';

const debug = require('debug')('joystream:sync');

async function sync_callback(api, storage) {
  // The first step is to gather all data objects from chain.
  // TODO: in future, limit to a configured tranche
  // FIXME this isn't actually on chain yet, so we'll fake it.
  const knownContentIds = await api.assets.getKnownContentIds() || [];

  const role_addr = api.identities.key.address
  const providerId = api.storageProviderId

  // Iterate over all sync objects, and ensure they're synced.
  const allChecks = knownContentIds.map(async (content_id) => {
    let { relationship, relationshipId } = await api.assets.getStorageRelationshipAndId(providerId, content_id);

    // get the data object
    // make sure the data object was Accepted by the liaison,
    // don't just blindly attempt to fetch them

    let fileLocal;
    try {
      // check if we have content or not
      let stats = await storage.stat(content_id);
      fileLocal = stats.local;
    } catch (err) {
      // on error stating or timeout
      debug(err.message);
      // we don't have content if we can't stat it
      fileLocal = false;
    }

    if (!fileLocal) {
      try {
        await storage.synchronize(content_id);
      } catch (err) {
        // duplicate logging
        // debug(err.message)
        return
      }
      // why are we returning, if we synced the file
      return;
    }

    if (!relationship) {
      // create relationship
      debug(`Creating new storage relationship for ${content_id.encode()}`);
      try {
        relationshipId = await api.assets.createAndReturnStorageRelationship(role_addr, providerId, content_id);
        await api.assets.toggleStorageRelationshipReady(role_addr, providerId, relationshipId, true);
      } catch (err) {
        debug(`Error creating new storage relationship ${content_id.encode()}: ${err.stack}`);
        return;
      }
    } else if (!relationship.ready) {
      debug(`Updating storage relationship to ready for ${content_id.encode()}`);
      // update to ready. (Why would there be a relationship set to ready: false?)
      try {
        await api.assets.toggleStorageRelationshipReady(role_addr, providerId, relationshipId, true);
      } catch(err) {
        debug(`Error setting relationship ready ${content_id.encode()}: ${err.stack}`);
      }
    } else {
      // we already have content and a ready relationship set. No need to do anything
      // debug(`content already stored locally ${content_id.encode()}`);
    }
  });


  return Promise.all(allChecks);
}


async function sync_periodic(api, flags, storage)
{
  try {
    debug('Starting sync run...')
    await sync_callback(api, storage)
    debug('sync run complete')
  } catch (err) {
    debug(`Error in sync_periodic ${err.stack}`);
  }
  // always try again
  setTimeout(sync_periodic, flags.syncPeriod, api, flags, storage);
}


function start_syncing(api, flags, storage)
{
  sync_periodic(api, flags, storage);
}

module.exports = {
  start_syncing: start_syncing,
}