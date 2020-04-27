/*
 * EpiRust
 * Copyright (c) 2020  ThoughtWorks, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

const extractStatus = (simulations) => {
  if(simulations.some(s => s.status === 'failed')) return 'failed'
  if(simulations.some(s => s.status === 'running')) return 'running'
  if(simulations.every(s => s.status === 'finished')) return 'finished'
  if(simulations.some(s => s.status === 'finished')) return 'running'
  return 'in-queue'
}

export const reduceStatus = (data) => {
  return {
    ...data,
    status: extractStatus(data.simulations)
  }
}
